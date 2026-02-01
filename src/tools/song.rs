//! Song-level operations.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    DeleteReturnTrackParams, JumpByParams, SetCurrentTimeParams, SetEnabledParams,
    SetGrooveAmountParams, SetLoopBeatsParams, SetLoopEnabledParams, SetQuantizationParams,
    SetRootNoteParams, SetScaleNameParams, SetSignatureDenominatorParams,
    SetSignatureNumeratorParams, SongDetailedInfo, SongInfo, TrackParams,
};

#[tool_router(router = song_router, vis = "pub")]
impl AbletonServer {
    /// Get basic song information.
    #[tool(
        description = "Get basic song information (tempo, playing state, time, track/scene counts)"
    )]
    pub async fn get_song_info(&self) -> Result<String, Error> {
        let tempo: f32 = self.osc.query("/live/song/get/tempo", vec![]).await?;

        let is_playing: bool = self
            .osc
            .query("/live/song/get/is_playing", vec![])
            .await
            .unwrap_or(false);

        let current_time: f32 = self
            .osc
            .query("/live/song/get/current_song_time", vec![])
            .await
            .unwrap_or(0.0);

        let track_count: i32 = self
            .osc
            .query("/live/song/get/num_tracks", vec![])
            .await
            .unwrap_or(0);

        let scene_count: i32 = self
            .osc
            .query("/live/song/get/num_scenes", vec![])
            .await
            .unwrap_or(0);

        let info = SongInfo {
            tempo,
            is_playing,
            current_time,
            track_count: track_count as u32,
            scene_count: scene_count as u32,
        };
        Ok(serde_json::to_string_pretty(&info).unwrap_or_else(|_| "{}".into()))
    }

    /// Get detailed song information.
    #[tool(
        description = "Get detailed song information including groove, metronome, loop, scale settings"
    )]
    pub async fn get_song_detailed_info(&self) -> Result<String, Error> {
        let tempo: f32 = self.osc.query("/live/song/get/tempo", vec![]).await?;

        let is_playing: bool = self
            .osc
            .query("/live/song/get/is_playing", vec![])
            .await
            .unwrap_or(false);

        let current_time: f32 = self
            .osc
            .query("/live/song/get/current_song_time", vec![])
            .await
            .unwrap_or(0.0);

        let song_length: f32 = self
            .osc
            .query("/live/song/get/song_length", vec![])
            .await
            .unwrap_or(0.0);

        let track_count: i32 = self
            .osc
            .query("/live/song/get/num_tracks", vec![])
            .await
            .unwrap_or(0);

        let scene_count: i32 = self
            .osc
            .query("/live/song/get/num_scenes", vec![])
            .await
            .unwrap_or(0);

        let can_undo: bool = self
            .osc
            .query("/live/song/get/can_undo", vec![])
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let can_redo: bool = self
            .osc
            .query("/live/song/get/can_redo", vec![])
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let signature_numerator: i32 = self
            .osc
            .query("/live/song/get/signature_numerator", vec![])
            .await
            .unwrap_or(4);

        let signature_denominator: i32 = self
            .osc
            .query("/live/song/get/signature_denominator", vec![])
            .await
            .unwrap_or(4);

        let groove_amount: f32 = self
            .osc
            .query("/live/song/get/groove_amount", vec![])
            .await
            .unwrap_or(0.0);

        let metronome: bool = self
            .osc
            .query("/live/song/get/metronome", vec![])
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let loop_enabled: bool = self
            .osc
            .query("/live/song/get/loop", vec![])
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let loop_start: f32 = self
            .osc
            .query("/live/song/get/loop_start", vec![])
            .await
            .unwrap_or(0.0);

        let loop_length: f32 = self
            .osc
            .query("/live/song/get/loop_length", vec![])
            .await
            .unwrap_or(4.0);

        let root_note: i32 = self
            .osc
            .query("/live/song/get/root_note", vec![])
            .await
            .unwrap_or(0);

        let scale_name: String = self
            .osc
            .query("/live/song/get/scale_name", vec![])
            .await
            .unwrap_or_else(|_| "Major".to_string());

        let info = SongDetailedInfo {
            tempo,
            is_playing,
            current_time,
            song_length,
            track_count: track_count as u32,
            scene_count: scene_count as u32,
            can_undo,
            can_redo,
            signature_numerator,
            signature_denominator,
            groove_amount,
            metronome,
            loop_enabled,
            loop_start,
            loop_length,
            root_note,
            scale_name,
        };
        Ok(serde_json::to_string_pretty(&info).unwrap_or_else(|_| "{}".into()))
    }

    /// Undo the last action.
    #[tool(description = "Undo the last action")]
    pub async fn undo(&self) -> Result<String, Error> {
        self.osc.send("/live/song/undo", vec![]).await?;
        Ok("Undid last action".to_string())
    }

    /// Redo the last undone action.
    #[tool(description = "Redo the last undone action")]
    pub async fn redo(&self) -> Result<String, Error> {
        self.osc.send("/live/song/redo", vec![]).await?;
        Ok("Redid last action".to_string())
    }

    /// Save the current Live set.
    #[tool(description = "Save the current Live set")]
    pub async fn save(&self) -> Result<String, Error> {
        self.osc.send("/live/song/save", vec![]).await?;
        Ok("Live set saved".to_string())
    }

    /// Set the loop start position in beats.
    #[tool(description = "Set the loop start position in beats")]
    pub async fn set_loop_start(
        &self,
        Parameters(params): Parameters<SetLoopBeatsParams>,
    ) -> Result<String, Error> {
        let beats = params.beats;
        self.osc
            .send("/live/song/set/loop_start", vec![OscType::Float(beats)])
            .await?;
        Ok(format!("Loop start set to beat {beats}"))
    }

    /// Set the loop length in beats.
    #[tool(description = "Set the loop length in beats")]
    pub async fn set_loop_length(
        &self,
        Parameters(params): Parameters<SetLoopBeatsParams>,
    ) -> Result<String, Error> {
        let beats = params.beats;
        self.osc
            .send("/live/song/set/loop_length", vec![OscType::Float(beats)])
            .await?;
        Ok(format!("Loop length set to {beats} beats"))
    }

    /// Enable or disable the loop.
    #[tool(description = "Enable or disable the loop")]
    pub async fn set_loop_enabled(
        &self,
        Parameters(params): Parameters<SetLoopEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/loop",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Loop {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get the current loop state.
    #[tool(description = "Get the current loop state")]
    pub async fn get_loop_enabled(&self) -> Result<String, Error> {
        let enabled: bool = self.osc.query("/live/song/get/loop", vec![]).await?;
        Ok(format!(
            "Loop is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get clip trigger quantization.
    #[tool(description = "Get clip trigger quantization (0=None, 1=8 Bars, 4=1 Bar, 7=1/4, etc.)")]
    pub async fn get_quantization(&self) -> Result<String, Error> {
        let quantization: i32 = self
            .osc
            .query("/live/song/get/clip_trigger_quantization", vec![])
            .await?;
        Ok(format!("Clip trigger quantization: {quantization}"))
    }

    /// Set clip trigger quantization.
    #[tool(description = "Set clip trigger quantization (0=None, 1=8 Bars, 4=1 Bar, 7=1/4, etc.)")]
    pub async fn set_quantization(
        &self,
        Parameters(params): Parameters<SetQuantizationParams>,
    ) -> Result<String, Error> {
        let quantization = params.quantization;
        self.osc
            .send(
                "/live/song/set/clip_trigger_quantization",
                vec![OscType::Int(quantization)],
            )
            .await?;
        Ok(format!("Clip trigger quantization set to {quantization}"))
    }

    /// Get MIDI recording quantization.
    #[tool(description = "Get MIDI recording quantization")]
    pub async fn get_midi_recording_quantization(&self) -> Result<String, Error> {
        let quantization: i32 = self
            .osc
            .query("/live/song/get/midi_recording_quantization", vec![])
            .await?;
        Ok(format!("MIDI recording quantization: {quantization}"))
    }

    /// Set MIDI recording quantization.
    #[tool(description = "Set MIDI recording quantization")]
    pub async fn set_midi_recording_quantization(
        &self,
        Parameters(params): Parameters<SetQuantizationParams>,
    ) -> Result<String, Error> {
        let quantization = params.quantization;
        self.osc
            .send(
                "/live/song/set/midi_recording_quantization",
                vec![OscType::Int(quantization)],
            )
            .await?;
        Ok(format!("MIDI recording quantization set to {quantization}"))
    }

    /// Get global groove amount (0.0 to 1.0).
    #[tool(description = "Get global groove amount (0.0 to 1.0)")]
    pub async fn get_groove_amount(&self) -> Result<String, Error> {
        let amount: f32 = self
            .osc
            .query("/live/song/get/groove_amount", vec![])
            .await?;
        Ok(format!("Groove amount: {amount}"))
    }

    /// Set global groove amount (0.0 to 1.0).
    #[tool(description = "Set global groove amount (0.0 to 1.0)")]
    pub async fn set_groove_amount(
        &self,
        Parameters(params): Parameters<SetGrooveAmountParams>,
    ) -> Result<String, Error> {
        let amount = params.amount;
        if !(0.0..=1.0).contains(&amount) {
            return Err(Error::InvalidParameter(
                "Groove amount must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.osc
            .send("/live/song/set/groove_amount", vec![OscType::Float(amount)])
            .await?;
        Ok(format!("Groove amount set to {amount}"))
    }

    /// Get time signature numerator.
    #[tool(description = "Get time signature numerator")]
    pub async fn get_signature_numerator(&self) -> Result<String, Error> {
        let numerator: i32 = self
            .osc
            .query("/live/song/get/signature_numerator", vec![])
            .await?;
        Ok(format!("Time signature numerator: {numerator}"))
    }

    /// Get time signature denominator.
    #[tool(description = "Get time signature denominator")]
    pub async fn get_signature_denominator(&self) -> Result<String, Error> {
        let denominator: i32 = self
            .osc
            .query("/live/song/get/signature_denominator", vec![])
            .await?;
        Ok(format!("Time signature denominator: {denominator}"))
    }

    /// Set time signature numerator.
    #[tool(description = "Set time signature numerator")]
    pub async fn set_signature_numerator(
        &self,
        Parameters(params): Parameters<SetSignatureNumeratorParams>,
    ) -> Result<String, Error> {
        let numerator = params.numerator;
        self.osc
            .send(
                "/live/song/set/signature_numerator",
                vec![OscType::Int(numerator)],
            )
            .await?;
        Ok(format!("Time signature numerator set to {numerator}"))
    }

    /// Set time signature denominator.
    #[tool(description = "Set time signature denominator")]
    pub async fn set_signature_denominator(
        &self,
        Parameters(params): Parameters<SetSignatureDenominatorParams>,
    ) -> Result<String, Error> {
        let denominator = params.denominator;
        self.osc
            .send(
                "/live/song/set/signature_denominator",
                vec![OscType::Int(denominator)],
            )
            .await?;
        Ok(format!("Time signature denominator set to {denominator}"))
    }

    /// Get punch in state.
    #[tool(description = "Get punch in state")]
    pub async fn get_punch_in(&self) -> Result<String, Error> {
        let result: i32 = self.osc.query("/live/song/get/punch_in", vec![]).await?;
        let enabled = result != 0;
        Ok(format!(
            "Punch in is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set punch in state.
    #[tool(description = "Set punch in state")]
    pub async fn set_punch_in(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/punch_in",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Punch in {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get punch out state.
    #[tool(description = "Get punch out state")]
    pub async fn get_punch_out(&self) -> Result<String, Error> {
        let result: i32 = self.osc.query("/live/song/get/punch_out", vec![]).await?;
        let enabled = result != 0;
        Ok(format!(
            "Punch out is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set punch out state.
    #[tool(description = "Set punch out state")]
    pub async fn set_punch_out(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/punch_out",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Punch out {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get arrangement overdub state.
    #[tool(description = "Get arrangement overdub state")]
    pub async fn get_arrangement_overdub(&self) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query("/live/song/get/arrangement_overdub", vec![])
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Arrangement overdub is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set arrangement overdub state.
    #[tool(description = "Set arrangement overdub state")]
    pub async fn set_arrangement_overdub(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/arrangement_overdub",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Arrangement overdub {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get session record state.
    #[tool(description = "Get session record state")]
    pub async fn get_session_record(&self) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query("/live/song/get/session_record", vec![])
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Session record is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set session record state.
    #[tool(description = "Set session record state")]
    pub async fn set_session_record(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/session_record",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Session record {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Capture MIDI from the last played notes.
    #[tool(description = "Capture MIDI from the last played notes")]
    pub async fn capture_midi(&self) -> Result<String, Error> {
        self.osc.send("/live/song/capture_midi", vec![]).await?;
        Ok("Captured MIDI".to_string())
    }

    /// Stop all playing clips.
    #[tool(description = "Stop all playing clips")]
    pub async fn stop_all_clips(&self) -> Result<String, Error> {
        self.osc.send("/live/song/stop_all_clips", vec![]).await?;
        Ok("Stopped all clips".to_string())
    }

    /// Trigger session recording.
    #[tool(description = "Trigger session recording")]
    pub async fn trigger_session_record(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/trigger_session_record", vec![])
            .await?;
        Ok("Triggered session record".to_string())
    }

    /// Create a return track.
    #[tool(description = "Create a return track")]
    pub async fn create_return_track(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/create_return_track", vec![])
            .await?;
        Ok("Created return track".to_string())
    }

    /// Delete a return track by index.
    #[tool(description = "Delete a return track by index")]
    pub async fn delete_return_track(
        &self,
        Parameters(params): Parameters<DeleteReturnTrackParams>,
    ) -> Result<String, Error> {
        let index = params.index;
        self.osc
            .send(
                "/live/song/delete_return_track",
                vec![OscType::Int(index as i32)],
            )
            .await?;
        Ok(format!("Deleted return track {index}"))
    }

    /// Duplicate a track.
    #[tool(description = "Duplicate a track")]
    pub async fn duplicate_track(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        self.osc
            .send(
                "/live/song/duplicate_track",
                vec![OscType::Int(track as i32)],
            )
            .await?;
        Ok(format!("Duplicated track {track}"))
    }

    /// Jump forward or backward by beats.
    #[tool(description = "Jump forward or backward by beats (negative = backward)")]
    pub async fn jump_by(
        &self,
        Parameters(params): Parameters<JumpByParams>,
    ) -> Result<String, Error> {
        let beats = params.beats;
        self.osc
            .send("/live/song/jump_by", vec![OscType::Float(beats)])
            .await?;
        Ok(format!("Jumped by {beats} beats"))
    }

    /// Get back to arranger state.
    #[tool(description = "Get back to arranger state")]
    pub async fn get_back_to_arranger(&self) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query("/live/song/get/back_to_arranger", vec![])
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Back to arranger is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set back to arranger state.
    #[tool(description = "Set back to arranger state")]
    pub async fn set_back_to_arranger(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/back_to_arranger",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Back to arranger {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get song length in beats.
    #[tool(description = "Get song length in beats")]
    pub async fn get_song_length(&self) -> Result<String, Error> {
        let length: f32 = self.osc.query("/live/song/get/song_length", vec![]).await?;
        Ok(format!("Song length: {length} beats"))
    }

    /// Check if undo is available.
    #[tool(description = "Check if undo is available")]
    pub async fn can_undo(&self) -> Result<String, Error> {
        let result: i32 = self.osc.query("/live/song/get/can_undo", vec![]).await?;
        let can = result != 0;
        Ok(format!("Can undo: {can}"))
    }

    /// Check if redo is available.
    #[tool(description = "Check if redo is available")]
    pub async fn can_redo(&self) -> Result<String, Error> {
        let result: i32 = self.osc.query("/live/song/get/can_redo", vec![]).await?;
        let can = result != 0;
        Ok(format!("Can redo: {can}"))
    }

    /// Get session record status.
    #[tool(description = "Get session record status")]
    pub async fn get_session_record_status(&self) -> Result<String, Error> {
        let status: i32 = self
            .osc
            .query("/live/song/get/session_record_status", vec![])
            .await?;
        Ok(format!("Session record status: {status}"))
    }

    /// Capture and insert scene.
    #[tool(description = "Capture and insert scene from currently playing clips")]
    pub async fn capture_and_insert_scene(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/capture_and_insert_scene", vec![])
            .await?;
        Ok("Captured and inserted scene".to_string())
    }

    /// Force link beat time.
    #[tool(description = "Force link beat time")]
    pub async fn force_link_beat_time(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/force_link_beat_time", vec![])
            .await?;
        Ok("Forced link beat time".to_string())
    }

    /// Re-enable automation.
    #[tool(description = "Re-enable automation that was overridden")]
    pub async fn re_enable_automation(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/re_enable_automation", vec![])
            .await?;
        Ok("Re-enabled automation".to_string())
    }

    /// Get Ableton Link enabled state.
    #[tool(description = "Get Ableton Link enabled state")]
    pub async fn get_link_enabled(&self) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query("/live/song/get/is_ableton_link_enabled", vec![])
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Ableton Link is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set Ableton Link enabled state.
    #[tool(description = "Set Ableton Link enabled state")]
    pub async fn set_link_enabled(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/is_ableton_link_enabled",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Ableton Link {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Nudge tempo down.
    #[tool(description = "Nudge tempo down")]
    pub async fn nudge_down(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/set/nudge_down", vec![OscType::Int(1)])
            .await?;
        Ok("Nudged tempo down".to_string())
    }

    /// Nudge tempo up.
    #[tool(description = "Nudge tempo up")]
    pub async fn nudge_up(&self) -> Result<String, Error> {
        self.osc
            .send("/live/song/set/nudge_up", vec![OscType::Int(1)])
            .await?;
        Ok("Nudged tempo up".to_string())
    }

    /// Get record mode state.
    #[tool(description = "Get record mode state")]
    pub async fn get_record_mode(&self) -> Result<String, Error> {
        let result: i32 = self.osc.query("/live/song/get/record_mode", vec![]).await?;
        let enabled = result != 0;
        Ok(format!(
            "Record mode is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set record mode state.
    #[tool(description = "Set record mode state")]
    pub async fn set_record_mode(
        &self,
        Parameters(params): Parameters<SetEnabledParams>,
    ) -> Result<String, Error> {
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/song/set/record_mode",
                vec![OscType::Int(if enabled { 1 } else { 0 })],
            )
            .await?;
        Ok(format!(
            "Record mode {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get root note (0-11, where 0=C).
    #[tool(description = "Get root note (0-11, where 0=C, 1=C#, ..., 11=B)")]
    pub async fn get_root_note(&self) -> Result<String, Error> {
        let root_note: i32 = self.osc.query("/live/song/get/root_note", vec![]).await?;
        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let name = note_names.get(root_note as usize).unwrap_or(&"Unknown");
        Ok(format!("Root note: {name} ({root_note})"))
    }

    /// Set root note (0-11, where 0=C).
    #[tool(description = "Set root note (0-11, where 0=C, 1=C#, ..., 11=B)")]
    pub async fn set_root_note(
        &self,
        Parameters(params): Parameters<SetRootNoteParams>,
    ) -> Result<String, Error> {
        let root_note = params.root_note;
        if !(0..=11).contains(&root_note) {
            return Err(Error::InvalidParameter(
                "Root note must be 0-11 (C=0, C#=1, ..., B=11)".to_string(),
            ));
        }
        self.osc
            .send("/live/song/set/root_note", vec![OscType::Int(root_note)])
            .await?;
        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        Ok(format!(
            "Root note set to {}",
            note_names[root_note as usize]
        ))
    }

    /// Get scale name.
    #[tool(description = "Get scale name")]
    pub async fn get_scale_name(&self) -> Result<String, Error> {
        let scale_name: String = self.osc.query("/live/song/get/scale_name", vec![]).await?;
        Ok(format!("Scale: {scale_name}"))
    }

    /// Set scale name.
    #[tool(description = "Set scale name (e.g., Major, Minor, Dorian, etc.)")]
    pub async fn set_scale_name(
        &self,
        Parameters(params): Parameters<SetScaleNameParams>,
    ) -> Result<String, Error> {
        let scale_name = params.scale_name;
        self.osc
            .send(
                "/live/song/set/scale_name",
                vec![OscType::String(scale_name.clone())],
            )
            .await?;
        Ok(format!("Scale set to {scale_name}"))
    }

    /// Get all track names.
    #[tool(description = "Get all track names")]
    pub async fn get_track_names(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/song/get/track_names", vec![])
            .await
            .unwrap_or_default();

        let mut names = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        names.push(s);
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| "[]".into()))
    }

    /// Get all scene names.
    #[tool(description = "Get all scene names")]
    pub async fn get_scene_names(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/song/get/scenes/name", vec![])
            .await
            .unwrap_or_default();

        let mut names = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        names.push(s);
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| "[]".into()))
    }

    /// Set current song time in beats.
    #[tool(description = "Set current song time in beats")]
    pub async fn set_current_time(
        &self,
        Parameters(params): Parameters<SetCurrentTimeParams>,
    ) -> Result<String, Error> {
        let time = params.time;
        self.osc
            .send(
                "/live/song/set/current_song_time",
                vec![OscType::Float(time)],
            )
            .await?;
        Ok(format!("Current time set to {time} beats"))
    }
}
