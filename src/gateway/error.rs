use std::fmt::Display;

use tokio_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(Debug)]
pub enum Error {
    NoSessionToResume,
    Closed(Option<CloseFrame<'static>>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoSessionToResume => write!(f, "No session to resume"),
            Error::Closed(frame) => match frame {
                Some(frame) => write!(f, "Websocket closed with code {}", frame.code),
                None => write!(f, "Websocket closed"),
            },
        }
    }
}

impl std::error::Error for Error {}
