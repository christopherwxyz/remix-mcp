//! Clip control tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    AddClipNotesParams, ClipDetailedInfo, ClipInfo, ClipLoopBounds, ClipSlotParams,
    CreateClipParams, DuplicateClipToParams, MidiNote, RemoveClipNotesParams,
    SetClipColorIndexParams, SetClipColorParams, SetClipGainParams, SetClipLaunchModeParams,
    SetClipLaunchQuantizationParams, SetClipLegatoParams, SetClipLoopBoundsParams,
    SetClipLoopPointParams, SetClipLoopingParams, SetClipMarkerParams, SetClipMutedParams,
    SetClipNameParams, SetClipPitchFineParams, SetClipPitchParams, SetClipPositionParams,
    SetClipRamModeParams, SetClipSlotHasStopButtonParams, SetClipVelocityAmountParams,
    SetClipWarpModeParams, SetClipWarpParams,
};

#[tool_router(router = clips_router, vis = "pub")]
impl AbletonServer {
    /// Fire (trigger) a clip.
    #[tool(description = "Fire (trigger) a clip at the specified track and slot")]
    pub async fn fire_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/clip_slot/fire",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Fired clip at track {track}, slot {slot}"))
    }

    /// Stop a clip.
    #[tool(description = "Stop a clip at the specified track and slot")]
    pub async fn stop_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/clip_slot/stop",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Stopped clip at track {track}, slot {slot}"))
    }

    /// Get clip information.
    #[tool(description = "Get information about a clip (name, length, playing state, etc.)")]
    pub async fn get_clip_info(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let args = vec![OscType::Int(track as i32), OscType::Int(slot as i32)];

        // Check if clip exists
        let has_clip: bool = self
            .osc
            .query("/live/clip_slot/get/has_clip", args.clone())
            .await
            .unwrap_or(false);

        if !has_clip {
            return Err(Error::InvalidResponse(format!(
                "No clip at track {track}, slot {slot}"
            )));
        }

        let name: String = self
            .osc
            .query("/live/clip/get/name", args.clone())
            .await
            .unwrap_or_else(|_| "Unnamed Clip".to_string());

        let length: f32 = self
            .osc
            .query("/live/clip/get/length", args.clone())
            .await
            .unwrap_or(0.0);

        let is_playing: bool = self
            .osc
            .query("/live/clip/get/is_playing", args.clone())
            .await
            .unwrap_or(false);

        let is_recording: bool = self
            .osc
            .query("/live/clip/get/is_recording", args.clone())
            .await
            .unwrap_or(false);

        let is_triggered: bool = self
            .osc
            .query("/live/clip/get/is_triggered", args.clone())
            .await
            .unwrap_or(false);

        let info = ClipInfo {
            track,
            slot,
            name,
            length,
            is_playing,
            is_recording,
            is_triggered,
        };
        Ok(serde_json::to_string_pretty(&info).unwrap_or_else(|_| format!("{info:?}")))
    }

    /// Set clip name.
    #[tool(description = "Set the name of a clip")]
    pub async fn set_clip_name(
        &self,
        Parameters(params): Parameters<SetClipNameParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let name = params.name;
        self.osc
            .send(
                "/live/clip/set/name",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::String(name.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} renamed to \"{name}\""
        ))
    }

    /// Duplicate a clip to the next available slot.
    #[tool(description = "Duplicate a clip to the next slot on the same track")]
    pub async fn duplicate_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/clip_slot/duplicate_clip_to",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(track as i32),
                    OscType::Int((slot + 1) as i32),
                ],
            )
            .await?;
        Ok(format!(
            "Duplicated clip from track {track}, slot {slot} to slot {}",
            slot + 1
        ))
    }

    /// Delete a clip.
    #[tool(description = "Delete a clip at the specified track and slot")]
    pub async fn delete_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/clip_slot/delete_clip",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Deleted clip at track {track}, slot {slot}"))
    }

    /// Create an empty MIDI clip.
    #[tool(description = "Create an empty MIDI clip with the specified length in beats")]
    pub async fn create_clip(
        &self,
        Parameters(params): Parameters<CreateClipParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let length = params.length;
        self.osc
            .send(
                "/live/clip_slot/create_clip",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(length),
                ],
            )
            .await?;
        Ok(format!(
            "Created {length} beat clip at track {track}, slot {slot}"
        ))
    }

    /// Set clip loop start.
    #[tool(description = "Set the loop start position in beats")]
    pub async fn set_clip_loop_start(
        &self,
        Parameters(params): Parameters<SetClipLoopPointParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let start = params.position;
        self.osc
            .send(
                "/live/clip/set/loop_start",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(start),
                ],
            )
            .await?;
        Ok(format!(
            "Set loop start to {start} for clip at track {track}, slot {slot}"
        ))
    }

    /// Set clip loop end.
    #[tool(description = "Set the loop end position in beats")]
    pub async fn set_clip_loop_end(
        &self,
        Parameters(params): Parameters<SetClipLoopPointParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let end = params.position;
        self.osc
            .send(
                "/live/clip/set/loop_end",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(end),
                ],
            )
            .await?;
        Ok(format!(
            "Set loop end to {end} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get all MIDI notes from a clip.
    #[tool(description = "Get all MIDI notes from a clip")]
    pub async fn get_clip_notes(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let args = vec![OscType::Int(track as i32), OscType::Int(slot as i32)];

        // Get OSC packets and extract args
        let packets = self
            .osc
            .query_all("/live/clip/get/notes", args)
            .await
            .unwrap_or_default();

        // Flatten all args from all packets
        let mut osc_args = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                osc_args.extend(msg.args);
            }
        }

        let mut notes = Vec::new();
        let mut i = 0;

        // Parse quintuplets of (pitch, start_time, duration, velocity, mute)
        while i + 4 < osc_args.len() {
            let pitch = match &osc_args[i] {
                OscType::Int(v) => *v as u8,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let start_time = match &osc_args[i + 1] {
                OscType::Float(v) => *v,
                OscType::Double(v) => *v as f32,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let duration = match &osc_args[i + 2] {
                OscType::Float(v) => *v,
                OscType::Double(v) => *v as f32,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let velocity = match &osc_args[i + 3] {
                OscType::Int(v) => *v as u8,
                OscType::Float(v) => *v as u8,
                _ => {
                    i += 1;
                    continue;
                }
            };
            let muted = match &osc_args[i + 4] {
                OscType::Int(v) => *v != 0,
                OscType::Bool(v) => *v,
                _ => false,
            };

            notes.push(MidiNote {
                pitch,
                start_time,
                duration,
                velocity,
                muted,
            });
            i += 5;
        }

        Ok(serde_json::to_string_pretty(&notes).unwrap_or_else(|_| format!("{notes:?}")))
    }

    /// Add MIDI notes to a clip.
    #[tool(description = "Add MIDI notes to a clip")]
    pub async fn add_clip_notes(
        &self,
        Parameters(params): Parameters<AddClipNotesParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let notes = params.notes;
        // Build OSC args: track, slot, then for each note: pitch, start, duration, velocity, mute
        let mut args = vec![OscType::Int(track as i32), OscType::Int(slot as i32)];

        for note in &notes {
            args.push(OscType::Int(note.pitch as i32));
            args.push(OscType::Float(note.start_time));
            args.push(OscType::Float(note.duration));
            args.push(OscType::Int(note.velocity as i32));
            args.push(OscType::Int(if note.muted { 1 } else { 0 }));
        }

        self.osc.send("/live/clip/add/notes", args).await?;
        Ok(format!(
            "Added {} notes to clip at track {track}, slot {slot}",
            notes.len()
        ))
    }

    /// Remove MIDI notes from a clip within a range.
    #[tool(description = "Remove MIDI notes from a clip within a time and pitch range")]
    pub async fn remove_clip_notes(
        &self,
        Parameters(params): Parameters<RemoveClipNotesParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let start_time = params.start_time;
        let end_time = params.end_time;
        let pitch_start = params.pitch_start;
        let pitch_end = params.pitch_end;
        self.osc
            .send(
                "/live/clip/remove/notes",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(start_time),
                    OscType::Float(end_time - start_time), // AbletonOSC uses duration, not end
                    OscType::Int(pitch_start as i32),
                    OscType::Int((pitch_end - pitch_start + 1) as i32), // pitch span
                ],
            )
            .await?;
        Ok(format!(
            "Removed notes from clip at track {track}, slot {slot} \
             (time {start_time}-{end_time}, pitch {pitch_start}-{pitch_end})"
        ))
    }

    /// Get clip color.
    #[tool(description = "Get clip color (RGB integer)")]
    pub async fn get_clip_color(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let color: i32 = self
            .osc
            .query(
                "/live/clip/get/color",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Clip at track {track}, slot {slot} color: {color}"))
    }

    /// Set clip color.
    #[tool(description = "Set clip color (RGB integer)")]
    pub async fn set_clip_color(
        &self,
        Parameters(params): Parameters<SetClipColorParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let color = params.color;
        self.osc
            .send(
                "/live/clip/set/color",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(color),
                ],
            )
            .await?;
        Ok(format!(
            "Set color to {color} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip gain (audio clips only).
    #[tool(description = "Get clip gain (audio clips only)")]
    pub async fn get_clip_gain(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let gain: f32 = self
            .osc
            .query(
                "/live/clip/get/gain",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!("Clip at track {track}, slot {slot} gain: {gain}"))
    }

    /// Set clip gain (audio clips only).
    #[tool(description = "Set clip gain (audio clips only)")]
    pub async fn set_clip_gain(
        &self,
        Parameters(params): Parameters<SetClipGainParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let gain = params.gain;
        self.osc
            .send(
                "/live/clip/set/gain",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(gain),
                ],
            )
            .await?;
        Ok(format!(
            "Set gain to {gain} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip pitch (coarse, in semitones).
    #[tool(description = "Get clip pitch in semitones")]
    pub async fn get_clip_pitch(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let pitch: i32 = self
            .osc
            .query(
                "/live/clip/get/pitch_coarse",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} pitch: {pitch} semitones"
        ))
    }

    /// Set clip pitch (coarse, in semitones, -48 to +48).
    #[tool(description = "Set clip pitch in semitones (-48 to +48)")]
    pub async fn set_clip_pitch(
        &self,
        Parameters(params): Parameters<SetClipPitchParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let semitones = params.semitones;
        if !(-48..=48).contains(&semitones) {
            return Err(Error::InvalidParameter(
                "Pitch must be between -48 and +48 semitones".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/clip/set/pitch_coarse",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(semitones),
                ],
            )
            .await?;
        Ok(format!(
            "Set pitch to {semitones} semitones for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip warp enabled state.
    #[tool(description = "Get whether warping is enabled for a clip")]
    pub async fn get_clip_warp(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/warping",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} warp: {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set clip warp enabled.
    #[tool(description = "Enable or disable warping for a clip")]
    pub async fn set_clip_warp(
        &self,
        Parameters(params): Parameters<SetClipWarpParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/clip/set/warping",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if enabled { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Warp {} for clip at track {track}, slot {slot}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get clip warp mode.
    #[tool(
        description = "Get clip warp mode (0=Beats, 1=Tones, 2=Texture, 3=Re-Pitch, 4=Complex, 5=Complex Pro)"
    )]
    pub async fn get_clip_warp_mode(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let mode: i32 = self
            .osc
            .query(
                "/live/clip/get/warp_mode",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let mode_name = match mode {
            0 => "Beats",
            1 => "Tones",
            2 => "Texture",
            3 => "Re-Pitch",
            4 => "Complex",
            5 => "Complex Pro",
            _ => "Unknown",
        };
        Ok(format!(
            "Clip at track {track}, slot {slot} warp mode: {mode_name} ({mode})"
        ))
    }

    /// Set clip warp mode.
    #[tool(
        description = "Set clip warp mode (0=Beats, 1=Tones, 2=Texture, 3=Re-Pitch, 4=Complex, 5=Complex Pro)"
    )]
    pub async fn set_clip_warp_mode(
        &self,
        Parameters(params): Parameters<SetClipWarpModeParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let mode = params.mode;
        if !(0..=5).contains(&mode) {
            return Err(Error::InvalidParameter(
                "Warp mode must be 0-5 (Beats, Tones, Texture, Re-Pitch, Complex, Complex Pro)"
                    .to_string(),
            ));
        }
        self.osc
            .send(
                "/live/clip/set/warp_mode",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(mode),
                ],
            )
            .await?;
        let mode_name = match mode {
            0 => "Beats",
            1 => "Tones",
            2 => "Texture",
            3 => "Re-Pitch",
            4 => "Complex",
            5 => "Complex Pro",
            _ => "Unknown",
        };
        Ok(format!(
            "Set warp mode to {mode_name} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip loop bounds.
    #[tool(description = "Get clip loop start and end positions")]
    pub async fn get_clip_loop_bounds(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let args = vec![OscType::Int(track as i32), OscType::Int(slot as i32)];

        let start: f32 = self
            .osc
            .query("/live/clip/get/loop_start", args.clone())
            .await
            .unwrap_or(0.0);

        let end: f32 = self
            .osc
            .query("/live/clip/get/loop_end", args)
            .await
            .unwrap_or(4.0);

        let bounds = ClipLoopBounds { start, end };
        Ok(serde_json::to_string_pretty(&bounds).unwrap_or_else(|_| format!("{bounds:?}")))
    }

    /// Set clip loop bounds.
    #[tool(description = "Set clip loop start and end positions")]
    pub async fn set_clip_loop_bounds(
        &self,
        Parameters(params): Parameters<SetClipLoopBoundsParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let start = params.start;
        let end = params.end;
        if start >= end {
            return Err(Error::InvalidParameter(
                "Loop start must be less than loop end".to_string(),
            ));
        }

        // Set start first, then end
        self.osc
            .send(
                "/live/clip/set/loop_start",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(start),
                ],
            )
            .await?;

        self.osc
            .send(
                "/live/clip/set/loop_end",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(end),
                ],
            )
            .await?;

        Ok(format!(
            "Set loop bounds to {start}-{end} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip launch mode.
    #[tool(description = "Get clip launch mode (0=Trigger, 1=Gate, 2=Toggle, 3=Repeat)")]
    pub async fn get_clip_launch_mode(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let mode: i32 = self
            .osc
            .query(
                "/live/clip/get/launch_mode",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let mode_name = match mode {
            0 => "Trigger",
            1 => "Gate",
            2 => "Toggle",
            3 => "Repeat",
            _ => "Unknown",
        };
        Ok(format!(
            "Clip at track {track}, slot {slot} launch mode: {mode_name} ({mode})"
        ))
    }

    /// Set clip launch mode.
    #[tool(description = "Set clip launch mode (0=Trigger, 1=Gate, 2=Toggle, 3=Repeat)")]
    pub async fn set_clip_launch_mode(
        &self,
        Parameters(params): Parameters<SetClipLaunchModeParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let mode = params.mode;
        if !(0..=3).contains(&mode) {
            return Err(Error::InvalidParameter(
                "Launch mode must be 0-3 (Trigger, Gate, Toggle, Repeat)".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/clip/set/launch_mode",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(mode),
                ],
            )
            .await?;
        let mode_name = match mode {
            0 => "Trigger",
            1 => "Gate",
            2 => "Toggle",
            3 => "Repeat",
            _ => "Unknown",
        };
        Ok(format!(
            "Set launch mode to {mode_name} for clip at track {track}, slot {slot}"
        ))
    }

    /// Get clip launch quantization.
    #[tool(description = "Get clip launch quantization value")]
    pub async fn get_clip_launch_quantization(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let quant: i32 = self
            .osc
            .query(
                "/live/clip/get/launch_quantization",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} launch quantization: {quant}"
        ))
    }

    /// Set clip launch quantization.
    #[tool(description = "Set clip launch quantization value")]
    pub async fn set_clip_launch_quantization(
        &self,
        Parameters(params): Parameters<SetClipLaunchQuantizationParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let quantization = params.quantization;
        self.osc
            .send(
                "/live/clip/set/launch_quantization",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(quantization),
                ],
            )
            .await?;
        Ok(format!(
            "Set launch quantization to {quantization} for clip at track {track}, slot {slot}"
        ))
    }

    /// Duplicate the clip's loop (double its length).
    #[tool(description = "Duplicate the clip's loop (double its length)")]
    pub async fn duplicate_clip_loop(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        self.osc
            .send(
                "/live/clip/duplicate_loop",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Duplicated loop for clip at track {track}, slot {slot}"
        ))
    }

    /// Check if a clip slot has a clip.
    #[tool(description = "Check if a clip slot has a clip")]
    pub async fn has_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip_slot/get/has_clip",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Clip slot at track {track}, slot {slot} {}",
            if has { "has a clip" } else { "is empty" }
        ))
    }

    /// Duplicate a clip to a specific slot.
    #[tool(description = "Duplicate a clip to a specific track and slot")]
    pub async fn duplicate_clip_to(
        &self,
        Parameters(params): Parameters<DuplicateClipToParams>,
    ) -> Result<String, Error> {
        let src_track = params.src_track;
        let src_slot = params.src_slot;
        let dst_track = params.dst_track;
        let dst_slot = params.dst_slot;
        self.osc
            .send(
                "/live/clip_slot/duplicate_clip_to",
                vec![
                    OscType::Int(src_track as i32),
                    OscType::Int(src_slot as i32),
                    OscType::Int(dst_track as i32),
                    OscType::Int(dst_slot as i32),
                ],
            )
            .await?;
        Ok(format!(
            "Duplicated clip from track {src_track}, slot {src_slot} to track {dst_track}, slot {dst_slot}"
        ))
    }

    /// Get clip slot stop button visibility.
    #[tool(description = "Check if clip slot has a stop button")]
    pub async fn get_clip_slot_has_stop_button(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip_slot/get/has_stop_button",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Clip slot at track {track}, slot {slot} stop button: {}",
            if has { "shown" } else { "hidden" }
        ))
    }

    /// Set clip slot stop button visibility.
    #[tool(description = "Show or hide the clip slot stop button")]
    pub async fn set_clip_slot_has_stop_button(
        &self,
        Parameters(params): Parameters<SetClipSlotHasStopButtonParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let has_stop_button = params.has_stop_button;
        self.osc
            .send(
                "/live/clip_slot/set/has_stop_button",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if has_stop_button { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Clip slot at track {track}, slot {slot} stop button {}",
            if has_stop_button { "shown" } else { "hidden" }
        ))
    }

    /// Get detailed clip information with all properties.
    #[tool(description = "Get detailed clip information with all properties")]
    pub async fn get_clip_detailed_info(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let args = vec![OscType::Int(track as i32), OscType::Int(slot as i32)];

        // Check if clip exists
        let has_clip: bool = self
            .osc
            .query("/live/clip_slot/get/has_clip", args.clone())
            .await
            .unwrap_or(false);

        if !has_clip {
            return Err(Error::InvalidResponse(format!(
                "No clip at track {track}, slot {slot}"
            )));
        }

        let name: String = self
            .osc
            .query("/live/clip/get/name", args.clone())
            .await
            .unwrap_or_default();

        let length: f32 = self
            .osc
            .query("/live/clip/get/length", args.clone())
            .await
            .unwrap_or(0.0);

        let is_playing: bool = self
            .osc
            .query("/live/clip/get/is_playing", args.clone())
            .await
            .unwrap_or(false);

        let is_recording: bool = self
            .osc
            .query("/live/clip/get/is_recording", args.clone())
            .await
            .unwrap_or(false);

        let is_triggered: bool = self
            .osc
            .query("/live/clip/get/is_triggered", args.clone())
            .await
            .unwrap_or(false);

        let is_midi_clip: bool = self
            .osc
            .query("/live/clip/get/is_midi_clip", args.clone())
            .await
            .unwrap_or(false);

        let is_audio_clip: bool = self
            .osc
            .query("/live/clip/get/is_audio_clip", args.clone())
            .await
            .unwrap_or(false);

        let start_time: f32 = self
            .osc
            .query("/live/clip/get/start_time", args.clone())
            .await
            .unwrap_or(0.0);

        let end_time: f32 = self
            .osc
            .query("/live/clip/get/end_time", args.clone())
            .await
            .unwrap_or(0.0);

        let loop_start: f32 = self
            .osc
            .query("/live/clip/get/loop_start", args.clone())
            .await
            .unwrap_or(0.0);

        let loop_end: f32 = self
            .osc
            .query("/live/clip/get/loop_end", args.clone())
            .await
            .unwrap_or(0.0);

        let looping_int: i32 = self
            .osc
            .query("/live/clip/get/looping", args.clone())
            .await
            .unwrap_or(0);

        let muted_int: i32 = self
            .osc
            .query("/live/clip/get/muted", args.clone())
            .await
            .unwrap_or(0);

        let color: i32 = self
            .osc
            .query("/live/clip/get/color", args.clone())
            .await
            .unwrap_or(0);

        let playing_position: f32 = self
            .osc
            .query("/live/clip/get/playing_position", args.clone())
            .await
            .unwrap_or(0.0);

        let info = ClipDetailedInfo {
            track,
            slot,
            name,
            length,
            is_playing,
            is_recording,
            is_triggered,
            is_midi_clip,
            is_audio_clip,
            start_time,
            end_time,
            loop_start,
            loop_end,
            looping: looping_int != 0,
            muted: muted_int != 0,
            color,
            playing_position,
        };
        Ok(serde_json::to_string_pretty(&info).unwrap_or_else(|_| format!("{info:?}")))
    }

    /// Check if clip is a MIDI clip.
    #[tool(description = "Check if clip is a MIDI clip")]
    pub async fn is_midi_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/is_midi_clip",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let is_midi = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} is {}a MIDI clip",
            if is_midi { "" } else { "not " }
        ))
    }

    /// Check if clip is an audio clip.
    #[tool(description = "Check if clip is an audio clip")]
    pub async fn is_audio_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/is_audio_clip",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let is_audio = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} is {}an audio clip",
            if is_audio { "" } else { "not " }
        ))
    }

    /// Get clip file path (audio clips only).
    #[tool(description = "Get the file path for an audio clip")]
    pub async fn get_clip_file_path(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let path: String = self
            .osc
            .query(
                "/live/clip/get/file_path",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} file path: {path}"
        ))
    }

    /// Get clip playing position.
    #[tool(description = "Get the current playing position within the clip")]
    pub async fn get_clip_playing_position(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let pos: f32 = self
            .osc
            .query(
                "/live/clip/get/playing_position",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} playing position: {pos}"
        ))
    }

    /// Get clip start time (arrangement).
    #[tool(description = "Get the clip start time in arrangement")]
    pub async fn get_clip_start_time(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let time: f32 = self
            .osc
            .query(
                "/live/clip/get/start_time",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} start time: {time}"
        ))
    }

    /// Get clip end time.
    #[tool(description = "Get the clip end time")]
    pub async fn get_clip_end_time(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let time: f32 = self
            .osc
            .query(
                "/live/clip/get/end_time",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} end time: {time}"
        ))
    }

    /// Get clip looping state.
    #[tool(description = "Check if clip looping is enabled")]
    pub async fn get_clip_looping(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/looping",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let looping = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} looping: {}",
            if looping { "enabled" } else { "disabled" }
        ))
    }

    /// Set clip looping state.
    #[tool(description = "Enable or disable clip looping")]
    pub async fn set_clip_looping(
        &self,
        Parameters(params): Parameters<SetClipLoopingParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let looping = params.looping;
        self.osc
            .send(
                "/live/clip/set/looping",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if looping { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} looping {}",
            if looping { "enabled" } else { "disabled" }
        ))
    }

    /// Get clip muted state.
    #[tool(description = "Check if clip is muted")]
    pub async fn get_clip_muted(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/muted",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let muted = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} is {}",
            if muted { "muted" } else { "unmuted" }
        ))
    }

    /// Set clip muted state.
    #[tool(description = "Mute or unmute a clip")]
    pub async fn set_clip_muted(
        &self,
        Parameters(params): Parameters<SetClipMutedParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let muted = params.muted;
        self.osc
            .send(
                "/live/clip/set/muted",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if muted { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} {}",
            if muted { "muted" } else { "unmuted" }
        ))
    }

    /// Get clip position (playback start).
    #[tool(description = "Get clip playback start position")]
    pub async fn get_clip_position(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let pos: f32 = self
            .osc
            .query(
                "/live/clip/get/position",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} position: {pos}"
        ))
    }

    /// Set clip position (playback start).
    #[tool(description = "Set clip playback start position")]
    pub async fn set_clip_position(
        &self,
        Parameters(params): Parameters<SetClipPositionParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let position = params.position;
        self.osc
            .send(
                "/live/clip/set/position",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(position),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} position set to {position}"
        ))
    }

    /// Get clip start marker.
    #[tool(description = "Get clip start marker position")]
    pub async fn get_clip_start_marker(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let marker: f32 = self
            .osc
            .query(
                "/live/clip/get/start_marker",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} start marker: {marker}"
        ))
    }

    /// Set clip start marker.
    #[tool(description = "Set clip start marker position")]
    pub async fn set_clip_start_marker(
        &self,
        Parameters(params): Parameters<SetClipMarkerParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let marker = params.marker;
        self.osc
            .send(
                "/live/clip/set/start_marker",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(marker),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} start marker set to {marker}"
        ))
    }

    /// Get clip end marker.
    #[tool(description = "Get clip end marker position")]
    pub async fn get_clip_end_marker(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let marker: f32 = self
            .osc
            .query(
                "/live/clip/get/end_marker",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} end marker: {marker}"
        ))
    }

    /// Set clip end marker.
    #[tool(description = "Set clip end marker position")]
    pub async fn set_clip_end_marker(
        &self,
        Parameters(params): Parameters<SetClipMarkerParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let marker = params.marker;
        self.osc
            .send(
                "/live/clip/set/end_marker",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(marker),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} end marker set to {marker}"
        ))
    }

    /// Get clip legato state.
    #[tool(description = "Check if clip legato is enabled")]
    pub async fn get_clip_legato(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/legato",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let legato = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} legato: {}",
            if legato { "enabled" } else { "disabled" }
        ))
    }

    /// Set clip legato state.
    #[tool(description = "Enable or disable clip legato")]
    pub async fn set_clip_legato(
        &self,
        Parameters(params): Parameters<SetClipLegatoParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let legato = params.legato;
        self.osc
            .send(
                "/live/clip/set/legato",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if legato { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} legato {}",
            if legato { "enabled" } else { "disabled" }
        ))
    }

    /// Get clip velocity amount.
    #[tool(description = "Get clip velocity amount (-1.0 to 1.0)")]
    pub async fn get_clip_velocity_amount(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let amount: f32 = self
            .osc
            .query(
                "/live/clip/get/velocity_amount",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} velocity amount: {amount}"
        ))
    }

    /// Set clip velocity amount.
    #[tool(description = "Set clip velocity amount (-1.0 to 1.0)")]
    pub async fn set_clip_velocity_amount(
        &self,
        Parameters(params): Parameters<SetClipVelocityAmountParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let amount = params.amount;
        self.osc
            .send(
                "/live/clip/set/velocity_amount",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(amount),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} velocity amount set to {amount}"
        ))
    }

    /// Get clip color index.
    #[tool(description = "Get clip color index")]
    pub async fn get_clip_color_index(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let index: i32 = self
            .osc
            .query(
                "/live/clip/get/color_index",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} color index: {index}"
        ))
    }

    /// Set clip color index.
    #[tool(description = "Set clip color index")]
    pub async fn set_clip_color_index(
        &self,
        Parameters(params): Parameters<SetClipColorIndexParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let color_index = params.color_index;
        self.osc
            .send(
                "/live/clip/set/color_index",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(color_index),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} color index set to {color_index}"
        ))
    }

    /// Get clip pitch fine.
    #[tool(description = "Get clip pitch fine (-50 to 50 cents)")]
    pub async fn get_clip_pitch_fine(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let cents: f32 = self
            .osc
            .query(
                "/live/clip/get/pitch_fine",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} pitch fine: {cents} cents"
        ))
    }

    /// Set clip pitch fine.
    #[tool(description = "Set clip pitch fine (-50 to 50 cents)")]
    pub async fn set_clip_pitch_fine(
        &self,
        Parameters(params): Parameters<SetClipPitchFineParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let cents = params.cents;
        self.osc
            .send(
                "/live/clip/set/pitch_fine",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Float(cents),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} pitch fine set to {cents} cents"
        ))
    }

    /// Get clip RAM mode state (audio clips only).
    #[tool(description = "Check if RAM mode is enabled (audio clips only)")]
    pub async fn get_clip_ram_mode(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/ram_mode",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} RAM mode: {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set clip RAM mode state (audio clips only).
    #[tool(description = "Enable or disable RAM mode (audio clips only)")]
    pub async fn set_clip_ram_mode(
        &self,
        Parameters(params): Parameters<SetClipRamModeParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/clip/set/ram_mode",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(slot as i32),
                    OscType::Int(if enabled { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} RAM mode {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Check if clip is overdubbing.
    #[tool(description = "Check if clip is currently overdubbing")]
    pub async fn is_clip_overdubbing(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/is_overdubbing",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let is_overdubbing = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} is {}overdubbing",
            if is_overdubbing { "" } else { "not " }
        ))
    }

    /// Check if clip will record on start.
    #[tool(description = "Check if clip will record when started")]
    pub async fn will_clip_record_on_start(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/will_record_on_start",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let will_record = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} {}record on start",
            if will_record { "will " } else { "will not " }
        ))
    }

    /// Get clip sample length (audio clips only).
    #[tool(description = "Get the sample length of an audio clip")]
    pub async fn get_clip_sample_length(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let length: f32 = self
            .osc
            .query(
                "/live/clip/get/sample_length",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} sample length: {length}"
        ))
    }

    /// Check if clip has groove.
    #[tool(description = "Check if clip has groove applied")]
    pub async fn has_clip_groove(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let result: i32 = self
            .osc
            .query(
                "/live/clip/get/has_groove",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Clip at track {track}, slot {slot} {}groove",
            if has { "has " } else { "does not have " }
        ))
    }

    /// Get clip gain display string.
    #[tool(description = "Get the clip gain as a display string")]
    pub async fn get_clip_gain_display(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let slot = params.slot;
        let display: String = self
            .osc
            .query(
                "/live/clip/get/gain_display_string",
                vec![OscType::Int(track as i32), OscType::Int(slot as i32)],
            )
            .await?;
        Ok(format!(
            "Clip at track {track}, slot {slot} gain: {display}"
        ))
    }
}
