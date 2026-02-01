//! View and selection tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{SceneParams, SetSelectedClipParams, SetSelectedDeviceParams, TrackParams};

#[tool_router(router = view_router, vis = "pub")]
impl AbletonServer {
    // ========== Selected Track ==========

    /// Get the currently selected track index.
    #[tool(description = "Get the currently selected track index")]
    pub async fn get_selected_track(&self) -> Result<String, Error> {
        let track: i32 = self
            .osc
            .query("/live/view/get/selected_track", vec![])
            .await?;
        Ok(format!("Selected track: {track}"))
    }

    /// Select a track by index.
    #[tool(description = "Select a track by index")]
    pub async fn set_selected_track(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        self.osc
            .send(
                "/live/view/set/selected_track",
                vec![OscType::Int(track as i32)],
            )
            .await?;
        Ok(format!("Selected track {track}"))
    }

    // ========== Selected Scene ==========

    /// Get the currently selected scene index.
    #[tool(description = "Get the currently selected scene index")]
    pub async fn get_selected_scene(&self) -> Result<String, Error> {
        let scene: i32 = self
            .osc
            .query("/live/view/get/selected_scene", vec![])
            .await?;
        Ok(format!("Selected scene: {scene}"))
    }

    /// Select a scene by index.
    #[tool(description = "Select a scene by index")]
    pub async fn set_selected_scene(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        self.osc
            .send(
                "/live/view/set/selected_scene",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        Ok(format!("Selected scene {scene}"))
    }

    // ========== Selected Clip ==========

    /// Get the currently selected clip (track, slot).
    #[tool(description = "Get the currently selected clip (track, slot)")]
    pub async fn get_selected_clip(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/view/get/selected_clip", vec![])
            .await?;

        // Extract integers from the response packets
        let mut values = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Int(v) = arg {
                        values.push(v);
                    }
                }
            }
        }

        if values.len() >= 2 {
            Ok(format!(
                "Selected clip: track {}, slot {}",
                values[0], values[1]
            ))
        } else {
            Err(Error::InvalidResponse("No clip selected".to_string()))
        }
    }

    /// Select a clip by track and slot index.
    #[tool(description = "Select a clip by track and slot index")]
    pub async fn set_selected_clip(
        &self,
        Parameters(params): Parameters<SetSelectedClipParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/view/set/selected_clip",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Selected clip at track {track}, slot {slot}"))
    }

    // ========== Selected Device ==========

    /// Get the currently selected device (track, device).
    #[tool(description = "Get the currently selected device (track, device)")]
    pub async fn get_selected_device(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/view/get/selected_device", vec![])
            .await?;

        // Extract integers from the response packets
        let mut values = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Int(v) = arg {
                        values.push(v);
                    }
                }
            }
        }

        if values.len() >= 2 {
            Ok(format!(
                "Selected device: track {}, device {}",
                values[0], values[1]
            ))
        } else {
            Err(Error::InvalidResponse("No device selected".to_string()))
        }
    }

    /// Select a device by track and device index.
    #[tool(description = "Select a device by track and device index")]
    pub async fn set_selected_device(
        &self,
        Parameters(params): Parameters<SetSelectedDeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        self.osc
            .send(
                "/live/view/set/selected_device",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;
        Ok(format!("Selected device {device} on track {track}"))
    }
}
