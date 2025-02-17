use crate::rest::endpoints::RestApiEndpoint;
use crate::rest::errors::Error;
use crate::rest::{
    CreateCashierSessionRequest, CreateCashierSessionResponse, LoginRequest, LoginResponse,
    LoginResultModel,
};
use error_chain::bail;
use flurl::{FlUrl, FlUrlResponse};
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Duration;

#[async_trait::async_trait]
pub trait RestApiConfig {
    async fn get_api_url(&self) -> String;
    async fn get_api_key(&self) -> String;
    async fn get_timeout(&self) -> Duration;
}

pub struct RestApiClient<C: RestApiConfig> {
    config: C,
    login_result: std::sync::Mutex<Option<LoginResultModel>>,
}

impl<C: RestApiConfig> RestApiClient<C> {
    pub fn new(config: C) -> Self {
        Self {
            config,
            login_result: Default::default(),
        }
    }

    pub async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, Error> {
        let endpoint = RestApiEndpoint::AuthLogin;
        let resp: LoginResponse = self
            .send_deserialized(endpoint, Some(request), None)
            .await?;

        if resp.response.status != "OK" {
            return Err(format!("Failed to login {:?}", resp.response).into());
        }

        let mut access_token = self.login_result.lock().unwrap();
        access_token.replace(resp.result.clone());

        Ok(resp)
    }

    pub async fn create_cashier_session(
        &self,
        request: &CreateCashierSessionRequest,
    ) -> Result<CreateCashierSessionResponse, Error> {
        let endpoint = RestApiEndpoint::CashierCreateSession;

        self.send_deserialized(
            endpoint,
            Some(&request),
            Some(&self.config.get_api_key().await),
        )
        .await
    }

    async fn send<R: Serialize + Debug>(
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
        let result: Result<T, _> = serde_json::from_str(&response);

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

        Ok(body)
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
