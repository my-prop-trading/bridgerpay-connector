use crate::rest::endpoints::RestApiEndpoint;
use crate::rest::errors::Error;
use crate::rest::{
    CashierSessionModel, CreateCashierSessionRequest, LoginModel, LoginRequest, Response,
};
use error_chain::bail;
use flurl::{FlUrl, FlUrlResponse};
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Duration;

const CHECKOUT_WIDGET_TEMPLATE: &str = "<html><body><script src='https://checkout.bridgerpay.com/v2/launcher' data-cashier-key='{{cashier_key}}' data-cashier-token='{{cashier_token}}'></script></body></html>";
const WRAPPED_CHECKOUT_WIDGET_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/iframe-resizer/4.3.9/iframeResizer.min.js" integrity="sha512-+bpyZqiNr/4QlUd6YnrAeLXzgooA1HKN5yUagHgPSMACPZgj8bkpCyZezPtDy5XbviRm4w8Z1RhfuWyoWaeCyg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
</head>
<body>
    <iframe id="wrappedCheckout" style="border:none" width="100%" srcdoc="<html>
        <head>
            <script src='https://cdnjs.cloudflare.com/ajax/libs/iframe-resizer/4.3.9/iframeResizer.contentWindow.min.js' integrity='sha512-mdT/HQRzoRP4laVz49Mndx6rcCGA3IhuyhP3gaY0E9sZPkwbtDk9ttQIq9o8qGCf5VvJv1Xsy3k2yTjfUoczqw==' crossorigin='anonymous' referrerpolicy='no-referrer'></script>
        </head>
        <body>
            <script src='https://checkout.bridgerpay.com/v2/launcher'
            				data-cashier-key='{{cashier_key}}'
            				data-cashier-token='{{cashier_token}}'
            ></script>
            <script>
              window.addEventListener(
                '[bp]:redirect', 
                ({ detail: { url }}) => window.top.location.href = url
              )                                       
            </script>
        </body>
    </html>">
    </iframe>
    <script>
        iFrameResize({ checkOrigin: false }, '#wrappedCheckout')
    </script>
</body>
</html>"#;
const WALLET_CHECKOUT_WIDGET_TEMPLATE: &str = "<html><body><script src='https://checkout.bridgerpay.com/v2/launcher' data-cashier-key='{{cashier_key}}' data-cashier-token='{{cashier_token}}' data-button-mode='wallet'></script></body></html>";

#[async_trait::async_trait]
pub trait RestApiConfig {
    async fn get_api_url(&self) -> String;
    async fn get_api_key(&self) -> String;
    async fn get_timeout(&self) -> Duration;
    async fn get_user_name(&self) -> String;
    async fn get_password(&self) -> String;
    async fn get_cashier_key(&self) -> String;
}

pub struct RestApiClient<C: RestApiConfig> {
    pub config: C,
    login_result: std::sync::Mutex<Option<LoginModel>>,
}

impl<C: RestApiConfig> RestApiClient<C> {
    pub fn new(config: C) -> Self {
        Self {
            config,
            login_result: Default::default(),
        }
    }

    pub async fn login(&self) -> Result<LoginModel, Error> {
        let endpoint = RestApiEndpoint::AuthLogin;
        let request = LoginRequest {
            user_name: self.config.get_user_name().await,
            password: self.config.get_password().await,
        };
        let resp: LoginModel = self
            .send_deserialized(endpoint, Some(&request), None)
            .await?;

        let mut access_token = self.login_result.lock().unwrap();
        access_token.replace(resp.clone());

        Ok(resp)
    }

    pub async fn is_logged_in(&self) -> Result<bool, Error> {
        Ok(self.login_result.lock().unwrap().is_some())
    }

    pub async fn create_cashier_session(
        &self,
        request: CreateCashierSessionRequest,
    ) -> Result<CashierSessionModel, Error> {
        let endpoint = RestApiEndpoint::CreateCashierSession;
        let mut request = request;

        if request.cashier_key.is_none() {
            request.cashier_key = Some(self.config.get_cashier_key().await);
        }

        let resp: CashierSessionModel = self
            .send_deserialized(
                endpoint,
                Some(&request),
                Some(&self.config.get_api_key().await),
            )
            .await?;

        Ok(resp)
    }

    pub async fn generate_checkout_widget(
        &self,
        request: CreateCashierSessionRequest,
        widget_type: CheckoutWidgetType,
    ) -> Result<String, String> {
        let _ = self.login().await.map_err(|e| e.to_string())?;
        let session = self
            .create_cashier_session(request)
            .await
            .map_err(|e| e.to_string())?;

        let template = match widget_type {
            CheckoutWidgetType::Regular => CHECKOUT_WIDGET_TEMPLATE,
            CheckoutWidgetType::Wrapped => WRAPPED_CHECKOUT_WIDGET_TEMPLATE,
            CheckoutWidgetType::Wallet => WALLET_CHECKOUT_WIDGET_TEMPLATE,
        };
        let html = template
            .replace("{{cashier_key}}", &self.config.get_cashier_key().await)
            .replace("{{cashier_token}}", &session.cashier_token);

        Ok(html)
    }

    async fn _send<R: Serialize + Debug>(
        &self,
        endpoint: RestApiEndpoint,
        request: Option<&R>,
        path_params: Option<&str>,
    ) -> Result<String, Error> {
        if std::env::var("DEBUG").is_ok() {
            println!("execute send: {:?} {:?}", endpoint, request);
        }

        let timeout = self.config.get_timeout().await;
        let response =
            tokio::time::timeout(timeout, self.send_flurl(&endpoint, request, path_params)).await;

        let Ok(response) = response else {
            let msg = format!(
                "Failed {:?} {:?}: Timeout",
                endpoint.get_http_method(),
                endpoint
            );
            return Err(msg.into());
        };

        response
    }

    async fn send_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: RestApiEndpoint,
        request: Option<&R>,
        path_params: Option<&str>,
    ) -> Result<T, Error> {
        if std::env::var("DEBUG").is_ok() {
            println!("execute send_deserialized: {:?} {:?}", endpoint, request);
        }

        let timeout = self.config.get_timeout().await;
        let response = tokio::time::timeout(
            timeout,
            self.send_flurl_deserialized(&endpoint, request, path_params),
        )
        .await;

        let Ok(response) = response else {
            let msg = format!(
                "Failed {:?} {:?}: Timeout",
                endpoint.get_http_method(),
                endpoint
            );
            return Err(msg.into());
        };

        response
    }

    fn build_full_url(
        &self,
        base_url: &str,
        endpoint: &RestApiEndpoint,
        query_string: Option<String>,
    ) -> String {
        let endpoint_str = String::from(endpoint);

        if let Some(query_string) = query_string {
            format!("{base_url}{endpoint_str}?{query_string}")
        } else {
            format!("{base_url}{endpoint_str}")
        }
    }

    async fn send_flurl_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: &RestApiEndpoint,
        request: Option<&R>,
        path_params: Option<&str>,
    ) -> Result<T, Error> {
        let response = self.send_flurl(endpoint, request, path_params).await?;
        let result: Result<Response<T>, _> = serde_json::from_str(&response);

        let Ok(body) = result else {
            let msg = format!(
                "Failed to deserialize. Url: {:?} {:?}. Request: {:?}. Body: {}",
                endpoint.get_http_method(),
                String::from(endpoint),
                request,
                response
            );
            return Err(msg.into());
        };

        if body.response.status != "OK" {
            return Err(format!("Failed {:?} {:?}", endpoint, body.response).into());
        }

        Ok(body.result.unwrap())
    }

    async fn send_flurl<R: Serialize + Debug>(
        &self,
        endpoint: &RestApiEndpoint,
        request: Option<&R>,
        path_params: Option<&str>,
    ) -> Result<String, Error> {
        let mut request_json = None;

        if let Some(request) = request {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
        }

        let request_bytes: Option<Vec<u8>> = if let Some(request) = request {
            Some(serde_json::to_string(request)?.into_bytes())
        } else {
            None
        };
        let (flurl, url) = self.build_flurl(endpoint, request, path_params).await?;
        let http_method = endpoint.get_http_method();

        let result = if http_method == Method::GET {
            flurl.get().await
        } else if http_method == Method::POST {
            flurl.post(request_bytes).await
        } else if http_method == Method::PUT {
            flurl.put(request_bytes).await
        } else if http_method == Method::PATCH {
            flurl.patch(request_bytes).await
        } else if http_method == Method::DELETE {
            flurl.delete().await
        } else {
            panic!("not implemented");
        };

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {}. Request: {:?}. {:?}",
                url,
                request_json,
                result.unwrap_err()
            )
            .into());
        };

        handle_flurl_text(resp, &request_json, &url, endpoint.get_http_method()).await
    }

    pub async fn build_flurl<R: Serialize>(
        &self,
        endpoint: &RestApiEndpoint,
        request: Option<&R>,
        path_params: Option<&str>,
    ) -> Result<(FlUrl, String), Error> {
        let base_url = self.config.get_api_url().await;
        let http_method = endpoint.get_http_method();

        let mut url = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            self.build_full_url(&base_url, endpoint, Some(query_string))
        } else {
            self.build_full_url(&base_url, endpoint, None)
        };

        if let Some(path_params) = path_params {
            url = format!("{url}/{path_params}");
        }

        let flurl = self.add_headers(FlUrl::new(&url)).await;

        Ok((flurl, url))
    }

    async fn add_headers(&self, flurl: FlUrl) -> FlUrl {
        let json_content_str = "application/json";

        let mut flurl = flurl
            .with_header("Content-Type", json_content_str)
            .with_header("Accept", json_content_str)
            .with_header(
                "Host",
                self.config.get_api_url().await.replace("https://", ""),
            );

        if let Some(result) = self.login_result.lock().unwrap().as_ref() {
            flurl = flurl.with_header(
                "Authorization",
                format!("Bearer {}", result.access_token.token),
            );
        }

        flurl
    }

    pub fn build_query_string(&self, params: Vec<(&str, &str)>) -> String {
        let mut query_string = String::new();

        for (key, value) in params {
            let param = format!("{key}={value}&");
            query_string.push_str(&param);
        }

        query_string.pop(); // remove last & symbol

        query_string
    }
}

async fn handle_flurl_text(
    response: FlUrlResponse,
    request_json: &Option<String>,
    request_url: &str,
    request_method: Method,
) -> Result<String, Error> {
    let status_code = StatusCode::from_u16(response.get_status_code()).unwrap();
    let result = response.receive_body().await;

    let Ok(body_bytes) = result else {
        return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
    };

    let body_str = String::from_utf8(body_bytes).unwrap();

    match status_code {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(body_str),
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!(format!(
                "Internal Server Error. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!(format!(
                "Service Unavailable. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            bail!(format!(
                "Unauthorized or forbidden. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::BAD_REQUEST => {
            let error = body_str;
            bail!(format!(
                "Received bad request status. Url: {request_method:?} {request_url}. Request: {request_json:?}. Response: {error:?}"
            ));
        }
        code => {
            let error = body_str;
            bail!(format!("Received response code: {code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {error:?}"));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckoutWidgetType {
    Regular,
    Wrapped,
    Wallet,
}
