//! Transport control tools (play, stop, record, tempo).

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::OscType;

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{SetMetronomeParams, SetTempoParams, SetTimeParams};

#[tool_router(router = transport_router, vis = "pub")]
impl AbletonServer {
    /// Start playback in Ableton Live.
    #[tool(description = "Start playback in Ableton Live")]
    pub async fn play(&self) -> Result<String, Error> {
        self.osc.send("/live/song/start_playing", vec![]).await?;
        Ok("Playback started".to_string())
    }

    /// Stop playback in Ableton Live.
    #[tool(description = "Stop playback in Ableton Live")]
    pub async fn stop(&self) -> Result<String, Error> {
        self.osc.send("/live/song/stop_playing", vec![]).await?;
        Ok("Playback stopped".to_string())
    }

    /// Continue playback from the current position.
    #[tool(description = "Continue playback from the current position")]
    pub async fn continue_playing(&self) -> Result<String, Error> {
        self.osc.send("/live/song/continue_playing", vec![]).await?;
        Ok("Playback continued".to_string())
    }

    /// Toggle recording mode in Ableton Live.
    #[tool(description = "Toggle recording mode in Ableton Live")]
    pub async fn record(&self) -> Result<String, Error> {
        // Get current record state and toggle it
        let is_recording: bool = self.osc.query("/live/song/get/record_mode", vec![]).await?;
        let new_state = !is_recording;
        self.osc
            .send(
                "/live/song/set/record_mode",
                vec![OscType::Int(if new_state { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Recording {}",
            if new_state { "enabled" } else { "disabled" }
        ))
    }

    /// Get the current tempo in BPM.
    #[tool(description = "Get the current tempo in BPM")]
    pub async fn get_tempo(&self) -> Result<String, Error> {
        let tempo: f32 = self.osc.query("/live/song/get/tempo", vec![]).await?;
        Ok(format!("Current tempo: {tempo} BPM"))
    }

    /// Set the tempo in BPM (20-999).
    #[tool(description = "Set the tempo in BPM (20-999)")]
    pub async fn set_tempo(
        &self,
        Parameters(params): Parameters<SetTempoParams>,
    ) -> Result<String, Error> {
        let bpm = params.bpm;
        if !(20.0..=999.0).contains(&bpm) {
            return Err(Error::InvalidParameter(
                "BPM must be between 20 and 999".to_string(),
            ));
        }
        self.osc
            .send("/live/song/set/tempo", vec![OscType::Float(bpm)])
            .await?;
        Ok(format!("Tempo set to {bpm} BPM"))
    }

    /// Get the current playback position in beats.
    #[tool(description = "Get the current playback position in beats")]
    pub async fn get_time(&self) -> Result<String, Error> {
        let time: f32 = self
            .osc
            .query("/live/song/get/current_song_time", vec![])
            .await?;
        Ok(format!("Current position: {time} beats"))
    }

    /// Jump to a specific position in beats.
    #[tool(description = "Jump to a specific position in beats")]
    pub async fn set_time(
        &self,
        Parameters(params): Parameters<SetTimeParams>,
    ) -> Result<String, Error> {
        let beats = params.beats;
        self.osc
            .send(
                "/live/song/set/current_song_time",
                vec![OscType::Float(beats)],
            )
            .await?;
        Ok(format!("Jumped to beat {beats}"))
    }

    /// Tap tempo - register a beat for tempo detection.
    #[tool(description = "Tap tempo - register a beat for tempo detection")]
    pub async fn tap_tempo(&self) -> Result<String, Error> {
        self.osc.send("/live/song/tap_tempo", vec![]).await?;
        Ok("Tap tempo registered".to_string())
    }

    /// Enable or disable the metronome.
    #[tool(description = "Enable or disable the metronome")]
    pub async fn set_metronome(
        &self,
        Parameters(params): Parameters<SetMetronomeParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/metronome",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Metronome {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }
}
