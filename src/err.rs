use crate::log::Logger;

#[derive(Debug)]
pub enum LcError {
    ConfigError,
    ClientBuildError,
    DirError,
    ParseCookieError,
    RequestError,
}

impl std::string::ToString for LcError {
    fn to_string(&self) -> String {
        match self {
            LcError::ConfigError => {
                "Config parse failed.".error()
            },
            LcError::ClientBuildError => {
                "Http client build failed.".error()
            },
            LcError::DirError => {
                "Directory can not open.".error()
            },
            LcError::ParseCookieError => {
                "Cookie parsed failed.".error()
            },
            LcError::RequestError => {
                "Request failed.".error()
            },
        }
    }
}
