use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum RestApiEndpoint {
    AuthLogin,
    CashierCreateSession,
}

impl From<&RestApiEndpoint> for String {
    fn from(item: &RestApiEndpoint) -> Self {
        let api_version = "v2";

        match item {
            RestApiEndpoint::AuthLogin => format!("/{api_version}/auth/login"),
            RestApiEndpoint::CashierCreateSession => {
                format!("/{api_version}/cashier/session/create")
            }
        }
    }
}

impl RestApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            RestApiEndpoint::AuthLogin => Method::POST,
            RestApiEndpoint::CashierCreateSession => Method::POST,
        }
    }
}
