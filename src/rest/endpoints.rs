use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum RestApiEndpoint {
    Login,
}

impl From<&RestApiEndpoint> for String {
    fn from(item: &RestApiEndpoint) -> Self {
        let api_version = "v2";
        let api_name = "rest-api";

        match item {
            RestApiEndpoint::Login => {
                format!("/{api_version}/auth/login", )
            }
        }
    }
}

impl RestApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            RestApiEndpoint::Login => Method::POST,
        }
    }
}
