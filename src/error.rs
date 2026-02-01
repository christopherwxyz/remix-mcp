//! Error types for the Ableton MCP server.

use rmcp::model::{Content, IntoContents};
use thiserror::Error;

/// Errors that can occur in the Ableton MCP server.
#[derive(Debug, Error)]
pub enum Error {
    /// OSC encoding error.
    #[error("OSC encoding error: {0}")]
    OscEncode(#[from] rosc::OscError),

    /// Network I/O error.
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    /// Timeout waiting for OSC response.
    #[error("Timeout waiting for response from Ableton Live")]
    Timeout,

    /// Invalid response from Ableton.
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Parameter validation error.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Ableton Live not connected.
    #[error("Ableton Live is not connected or `AbletonOSC` is not running")]
    NotConnected,
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        Self::Timeout
    }
}

impl From<Error> for rmcp::ErrorData {
    fn from(err: Error) -> Self {
        Self::internal_error(err.to_string(), None)
    }
}

impl IntoContents for Error {
    fn into_contents(self) -> Vec<Content> {
        vec![Content::text(self.to_string())]
    }
}
