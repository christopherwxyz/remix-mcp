//! Application-level tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::OscType;

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::ShowMessageParams;

#[tool_router(router = application_router, vis = "pub")]
impl AbletonServer {
    /// Get Ableton Live version string.
    #[tool(description = "Get Ableton Live version string")]
    pub async fn get_version(&self) -> Result<String, Error> {
        let version: String = self
            .osc
            .query("/live/application/get/version", vec![])
            .await?;
        Ok(format!("Ableton Live version: {version}"))
    }

    /// Show a message in Ableton's status bar.
    #[tool(description = "Show a message in Ableton's status bar")]
    pub async fn show_message(
        &self,
        Parameters(params): Parameters<ShowMessageParams>,
    ) -> Result<String, Error> {
        let message = params.message;
        self.osc
            .send(
                "/live/api/show_message",
                vec![OscType::String(message.clone())],
            )
            .await?;
        Ok(format!("Displayed message: \"{message}\""))
    }

    /// Reload the `AbletonOSC` API (hot reload).
    #[tool(description = "Reload the AbletonOSC API (hot reload)")]
    pub async fn reload_api(&self) -> Result<String, Error> {
        self.osc.send("/live/api/reload", vec![]).await?;
        Ok("Reloaded AbletonOSC API".to_string())
    }

    /// Test connection to Ableton Live.
    #[tool(description = "Test connection to Ableton Live")]
    pub async fn test_connection(&self) -> Result<String, Error> {
        match self.osc.test_connection().await {
            Ok(true) => Ok("Connection to Ableton Live is working".to_string()),
            Ok(false) => Ok("No response from Ableton Live - is AbletonOSC enabled?".to_string()),
            Err(e) => Err(e),
        }
    }
}
