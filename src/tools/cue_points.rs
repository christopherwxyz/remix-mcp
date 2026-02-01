//! Cue point tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{CuePoint, JumpToCuePointParams, SetCuePointNameParams};

#[tool_router(router = cue_points_router, vis = "pub")]
impl AbletonServer {
    /// List all cue points in the song.
    #[tool(description = "List all cue points in the song")]
    pub async fn list_cue_points(&self) -> Result<String, Error> {
        // Get OSC packets and extract args
        let packets = self
            .osc
            .query_all("/live/song/get/cue_points", vec![])
            .await
            .unwrap_or_default();

        // Flatten all args from all packets
        let mut osc_args = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                osc_args.extend(msg.args);
            }
        }

        let mut cue_points = Vec::new();
        let mut i = 0;

        // Parse triplets of (id, time, name)
        while i + 2 < osc_args.len() {
            let id = match &osc_args[i] {
                OscType::Int(v) => *v as u32,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let time = match &osc_args[i + 1] {
                OscType::Float(v) => *v,
                OscType::Double(v) => *v as f32,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let name = match &osc_args[i + 2] {
                OscType::String(v) => v.clone(),
                _ => String::new(),
            };

            cue_points.push(CuePoint { id, time, name });
            i += 3;
        }

        Ok(serde_json::to_string_pretty(&cue_points).unwrap_or_else(|_| "[]".into()))
    }

    /// Jump to a cue point by index.
    #[tool(description = "Jump to a cue point by index")]
    pub async fn jump_to_cue_point(
        &self,
        Parameters(params): Parameters<JumpToCuePointParams>,
    ) -> Result<String, Error> {
        let index = params.index;
        self.osc
            .send(
                "/live/song/cue_point/jump",
                vec![OscType::Int(index as i32)],
            )
            .await?;
        Ok(format!("Jumped to cue point {index}"))
    }

    /// Jump to the next cue point.
    #[tool(description = "Jump to the next cue point")]
    pub async fn jump_to_next_cue(&self) -> Result<String, Error> {
        self.osc.send("/live/song/jump_to_next_cue", vec![]).await?;
        Ok("Jumped to next cue point".to_string())
    }

    /// Jump to the previous cue point.
    #[tool(description = "Jump to the previous cue point")]
    pub async fn jump_to_prev_cue(&self) -> Result<String, Error> {
        self.osc.send("/live/song/jump_to_prev_cue", vec![]).await?;
        Ok("Jumped to previous cue point".to_string())
    }

    /// Set cue point name.
    #[tool(description = "Set cue point name")]
    pub async fn set_cue_point_name(
        &self,
        Parameters(params): Parameters<SetCuePointNameParams>,
    ) -> Result<String, Error> {
        let index = params.index;
        let name = params.name.clone();
        self.osc
            .send(
                "/live/song/cue_point/set/name",
                vec![OscType::Int(index as i32), OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Renamed cue point {index} to \"{name}\""))
    }
}
