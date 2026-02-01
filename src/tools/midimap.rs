//! MIDI mapping tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::OscType;

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::MapMidiCcParams;

#[tool_router(router = midimap_router, vis = "pub")]
impl AbletonServer {
    /// Map a MIDI CC to a device parameter.
    #[tool(
        description = "Map a MIDI CC to a device parameter (track, device, parameter, channel 0-15, cc 0-127)"
    )]
    pub async fn map_midi_cc(
        &self,
        Parameters(params): Parameters<MapMidiCcParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let parameter = params.parameter;
        let channel = params.channel;
        let cc = params.cc;

        if channel > 15 {
            return Err(Error::InvalidParameter(
                "MIDI channel must be 0-15".to_string(),
            ));
        }
        if cc > 127 {
            return Err(Error::InvalidParameter(
                "CC number must be 0-127".to_string(),
            ));
        }

        self.osc
            .send(
                "/live/midimap/map_cc",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(device as i32),
                    OscType::Int(parameter as i32),
                    OscType::Int(channel as i32),
                    OscType::Int(cc as i32),
                ],
            )
            .await?;

        Ok(format!(
            "Mapped track {track} device {device} parameter {parameter} to MIDI CC {cc} on channel {channel}"
        ))
    }
}
