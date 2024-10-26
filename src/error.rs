use std::fmt::Display;

use crate::gateway;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Websocket(tokio_tungstenite::tungstenite::Error),
    Gateway(gateway::error::Error),
    Io(std::io::Error)
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Websocket(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<gateway::error::Error> for Error {
    fn from(e: gateway::error::Error) -> Self {
        Self::Gateway(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json(e) => Display::fmt(&e, f),
            Error::Websocket(e) => Display::fmt(&e, f),
            Error::Io(e) => Display::fmt(&e, f),
            Error::Gateway(e) => Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(e) => Some(e),
            Error::Websocket(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Gateway(e) => Some(e),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;