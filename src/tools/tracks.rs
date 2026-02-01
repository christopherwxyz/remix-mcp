//! Track control tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::OscPacket;
use rosc::OscType;

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    ArmTrackParams, ArrangementClipInfo, ClipSlotParams, CreateTrackParams, GetTrackSendParams,
    MuteTrackParams, RoutingOptions, SetTrackColorParams, SetTrackFoldStateParams,
    SetTrackMonitoringParams, SetTrackNameParams, SetTrackPanParams, SetTrackRoutingChannelParams,
    SetTrackRoutingTypeParams, SetTrackSendParams, SetTrackVolumeParams, SoloTrackParams,
    TrackCapabilities, TrackInfo, TrackParams,
};

#[tool_router(router = tracks_router, vis = "pub")]
impl AbletonServer {
    /// Get list of all tracks.
    #[tool(description = "Get list of all tracks with their properties")]
    pub async fn list_tracks(&self) -> Result<String, Error> {
        // Get track count first
        let count: i32 = self.osc.query("/live/song/get/num_tracks", vec![]).await?;

        let mut tracks = Vec::new();
        for i in 0..count {
            let args = vec![OscType::Int(i)];

            // Query track properties
            let name: String = self
                .osc
                .query("/live/track/get/name", args.clone())
                .await
                .unwrap_or_else(|_| format!("Track {}", i + 1));

            let armed: bool = self
                .osc
                .query("/live/track/get/arm", args.clone())
                .await
                .unwrap_or(false);

            let muted: bool = self
                .osc
                .query("/live/track/get/mute", args.clone())
                .await
                .unwrap_or(false);

            let soloed: bool = self
                .osc
                .query("/live/track/get/solo", args.clone())
                .await
                .unwrap_or(false);

            let volume: f32 = self
                .osc
                .query("/live/track/get/volume", args.clone())
                .await
                .unwrap_or(0.85);

            let pan: f32 = self
                .osc
                .query("/live/track/get/panning", args.clone())
                .await
                .unwrap_or(0.0);

            tracks.push(TrackInfo {
                index: i as u32,
                name,
                armed,
                muted,
                soloed,
                volume,
                pan,
            });
        }

        Ok(serde_json::to_string_pretty(&tracks).unwrap_or_else(|_| format!("{tracks:?}")))
    }

    /// Get information about a specific track.
    #[tool(description = "Get information about a specific track (index 0-based)")]
    pub async fn get_track(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let args = vec![OscType::Int(params.track as i32)];

        let name: String = self
            .osc
            .query("/live/track/get/name", args.clone())
            .await
            .unwrap_or_else(|_| format!("Track {}", params.track + 1));

        let armed: bool = self
            .osc
            .query("/live/track/get/arm", args.clone())
            .await
            .unwrap_or(false);

        let muted: bool = self
            .osc
            .query("/live/track/get/mute", args.clone())
            .await
            .unwrap_or(false);

        let soloed: bool = self
            .osc
            .query("/live/track/get/solo", args.clone())
            .await
            .unwrap_or(false);

        let volume: f32 = self
            .osc
            .query("/live/track/get/volume", args.clone())
            .await
            .unwrap_or(0.85);

        let pan: f32 = self
            .osc
            .query("/live/track/get/panning", args.clone())
            .await
            .unwrap_or(0.0);

        let track = TrackInfo {
            index: params.track,
            name,
            armed,
            muted,
            soloed,
            volume,
            pan,
        };

        Ok(serde_json::to_string_pretty(&track).unwrap_or_else(|_| format!("{track:?}")))
    }

    /// Set track volume.
    #[tool(description = "Set track volume (0.0 to 1.0)")]
    pub async fn set_track_volume(
        &self,
        Parameters(params): Parameters<SetTrackVolumeParams>,
    ) -> Result<String, Error> {
        if !(0.0..=1.0).contains(&params.volume) {
            return Err(Error::InvalidParameter(
                "Volume must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/track/set/volume",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Float(params.volume),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} volume set to {}",
            params.track, params.volume
        ))
    }

    /// Set track panning.
    #[tool(description = "Set track pan position (-1.0 left to 1.0 right)")]
    pub async fn set_track_pan(
        &self,
        Parameters(params): Parameters<SetTrackPanParams>,
    ) -> Result<String, Error> {
        if !(-1.0..=1.0).contains(&params.pan) {
            return Err(Error::InvalidParameter(
                "Pan must be between -1.0 and 1.0".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/track/set/panning",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Float(params.pan),
                ],
            )
            .await?;
        Ok(format!("Track {} pan set to {}", params.track, params.pan))
    }

    /// Mute or unmute a track.
    #[tool(description = "Mute or unmute a track")]
    pub async fn mute_track(
        &self,
        Parameters(params): Parameters<MuteTrackParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/mute",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(if params.mute { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} {}",
            params.track,
            if params.mute { "muted" } else { "unmuted" }
        ))
    }

    /// Solo or unsolo a track.
    #[tool(description = "Solo or unsolo a track")]
    pub async fn solo_track(
        &self,
        Parameters(params): Parameters<SoloTrackParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/solo",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(if params.solo { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} {}",
            params.track,
            if params.solo { "soloed" } else { "unsoloed" }
        ))
    }

    /// Arm or disarm a track for recording.
    #[tool(description = "Arm or disarm a track for recording")]
    pub async fn arm_track(
        &self,
        Parameters(params): Parameters<ArmTrackParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/arm",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(if params.arm { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} {}",
            params.track,
            if params.arm { "armed" } else { "disarmed" }
        ))
    }

    /// Create a new MIDI track.
    #[tool(description = "Create a new MIDI track (optionally at a specific index)")]
    pub async fn create_midi_track(
        &self,
        Parameters(params): Parameters<CreateTrackParams>,
    ) -> Result<String, Error> {
        let args = match params.index {
            Some(i) => vec![OscType::Int(i)],
            None => vec![],
        };
        self.osc.send("/live/song/create_midi_track", args).await?;
        Ok(match params.index {
            Some(i) => format!("Created MIDI track at index {i}"),
            None => "Created new MIDI track".to_string(),
        })
    }

    /// Create a new audio track.
    #[tool(description = "Create a new audio track (optionally at a specific index)")]
    pub async fn create_audio_track(
        &self,
        Parameters(params): Parameters<CreateTrackParams>,
    ) -> Result<String, Error> {
        let args = match params.index {
            Some(i) => vec![OscType::Int(i)],
            None => vec![],
        };
        self.osc.send("/live/song/create_audio_track", args).await?;
        Ok(match params.index {
            Some(i) => format!("Created audio track at index {i}"),
            None => "Created new audio track".to_string(),
        })
    }

    /// Delete a track.
    #[tool(description = "Delete a track by index")]
    pub async fn delete_track(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/song/delete_track",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!("Deleted track {}", params.track))
    }

    /// Set track name.
    #[tool(description = "Set the name of a track")]
    pub async fn set_track_name(
        &self,
        Parameters(params): Parameters<SetTrackNameParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/name",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::String(params.name.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} renamed to \"{}\"",
            params.track, params.name
        ))
    }

    /// Get track send level.
    #[tool(description = "Get track send level (0.0 to 1.0)")]
    pub async fn get_track_send(
        &self,
        Parameters(params): Parameters<GetTrackSendParams>,
    ) -> Result<String, Error> {
        let level: f32 = self
            .osc
            .query(
                "/live/track/get/send",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(params.send as i32),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} send {} level: {level}",
            params.track, params.send
        ))
    }

    /// Set track send level.
    #[tool(description = "Set track send level (0.0 to 1.0)")]
    pub async fn set_track_send(
        &self,
        Parameters(params): Parameters<SetTrackSendParams>,
    ) -> Result<String, Error> {
        if !(0.0..=1.0).contains(&params.level) {
            return Err(Error::InvalidParameter(
                "Send level must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/track/set/send",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(params.send as i32),
                    OscType::Float(params.level),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} send {} set to {}",
            params.track, params.send, params.level
        ))
    }

    /// Get track color.
    #[tool(description = "Get track color (RGB integer)")]
    pub async fn get_track_color(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let color: i32 = self
            .osc
            .query(
                "/live/track/get/color",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!("Track {} color: {color}", params.track))
    }

    /// Set track color.
    #[tool(description = "Set track color (RGB integer)")]
    pub async fn set_track_color(
        &self,
        Parameters(params): Parameters<SetTrackColorParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/color",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(params.color),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} color set to {}",
            params.track, params.color
        ))
    }

    /// Get track monitoring state.
    #[tool(description = "Get track monitoring state (0=In, 1=Auto, 2=Off)")]
    pub async fn get_track_monitoring(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let state: i32 = self
            .osc
            .query(
                "/live/track/get/current_monitoring_state",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let state_name = match state {
            0 => "In",
            1 => "Auto",
            2 => "Off",
            _ => "Unknown",
        };
        Ok(format!(
            "Track {} monitoring: {state_name} ({state})",
            params.track
        ))
    }

    /// Set track monitoring state.
    #[tool(description = "Set track monitoring state (0=In, 1=Auto, 2=Off)")]
    pub async fn set_track_monitoring(
        &self,
        Parameters(params): Parameters<SetTrackMonitoringParams>,
    ) -> Result<String, Error> {
        if !(0..=2).contains(&params.state) {
            return Err(Error::InvalidParameter(
                "Monitoring state must be 0 (In), 1 (Auto), or 2 (Off)".to_string(),
            ));
        }
        self.osc
            .send(
                "/live/track/set/current_monitoring_state",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(params.state),
                ],
            )
            .await?;
        let state_name = match params.state {
            0 => "In",
            1 => "Auto",
            2 => "Off",
            _ => "Unknown",
        };
        Ok(format!(
            "Track {} monitoring set to {state_name}",
            params.track
        ))
    }

    /// Get track output meter level.
    #[tool(description = "Get track output meter level (0.0 to 1.0)")]
    pub async fn get_track_output_meter(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let level: f32 = self
            .osc
            .query(
                "/live/track/get/output_meter_level",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!("Track {} output meter: {level}", params.track))
    }

    /// Get track output meter left channel.
    #[tool(description = "Get track output meter left channel (0.0 to 1.0)")]
    pub async fn get_track_output_meter_left(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let level: f32 = self
            .osc
            .query(
                "/live/track/get/output_meter_left",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!(
            "Track {} output meter (left): {level}",
            params.track
        ))
    }

    /// Get track output meter right channel.
    #[tool(description = "Get track output meter right channel (0.0 to 1.0)")]
    pub async fn get_track_output_meter_right(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let level: f32 = self
            .osc
            .query(
                "/live/track/get/output_meter_right",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!(
            "Track {} output meter (right): {level}",
            params.track
        ))
    }

    /// Get track fold state.
    #[tool(description = "Get track fold state (for group tracks)")]
    pub async fn get_track_fold_state(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/fold_state",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let folded = result != 0;
        Ok(format!(
            "Track {} is {}",
            params.track,
            if folded { "folded" } else { "unfolded" }
        ))
    }

    /// Set track fold state.
    #[tool(description = "Set track fold state (for group tracks)")]
    pub async fn set_track_fold_state(
        &self,
        Parameters(params): Parameters<SetTrackFoldStateParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/fold_state",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(if params.folded { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} {}",
            params.track,
            if params.folded { "folded" } else { "unfolded" }
        ))
    }

    /// Check if track is foldable.
    #[tool(description = "Check if track is foldable (is a group track)")]
    pub async fn is_track_foldable(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/is_foldable",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let foldable = result != 0;
        Ok(format!(
            "Track {} is {}foldable",
            params.track,
            if foldable { "" } else { "not " }
        ))
    }

    /// Check if track is grouped.
    #[tool(description = "Check if track is grouped (is inside a group)")]
    pub async fn is_track_grouped(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/is_grouped",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let grouped = result != 0;
        Ok(format!(
            "Track {} is {}grouped",
            params.track,
            if grouped { "" } else { "not " }
        ))
    }

    /// Stop all clips on a track.
    #[tool(description = "Stop all clips on a track")]
    pub async fn stop_track_clips(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/stop_all_clips",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        Ok(format!("Stopped all clips on track {}", params.track))
    }

    /// Get the currently playing clip slot index.
    #[tool(description = "Get the currently playing clip slot index on a track (-1 if none)")]
    pub async fn get_playing_slot(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let slot: i32 = self
            .osc
            .query(
                "/live/track/get/playing_slot_index",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        if slot < 0 {
            Ok(format!("Track {} has no playing clip", params.track))
        } else {
            Ok(format!("Track {} playing slot: {slot}", params.track))
        }
    }

    /// Get the triggered/fired clip slot index.
    #[tool(description = "Get the triggered/fired clip slot index on a track (-1 if none)")]
    pub async fn get_fired_slot(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let slot: i32 = self
            .osc
            .query(
                "/live/track/get/fired_slot_index",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        if slot < 0 {
            Ok(format!("Track {} has no fired clip", params.track))
        } else {
            Ok(format!("Track {} fired slot: {slot}", params.track))
        }
    }

    /// Get track input routing type.
    #[tool(description = "Get track input routing type")]
    pub async fn get_track_input_routing_type(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let routing = self.query_track_input_routing_type(params.track).await?;
        Ok(format!(
            "Track {} input routing type: {routing}",
            params.track
        ))
    }

    /// Set track input routing type.
    #[tool(description = "Set track input routing type")]
    pub async fn set_track_input_routing_type(
        &self,
        Parameters(params): Parameters<SetTrackRoutingTypeParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/input_routing_type",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::String(params.routing_type.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} input routing type set to {}",
            params.track, params.routing_type
        ))
    }

    /// Get track input routing channel.
    #[tool(description = "Get track input routing channel")]
    pub async fn get_track_input_routing_channel(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let channel = self.query_track_input_routing_channel(params.track).await?;
        Ok(format!(
            "Track {} input routing channel: {channel}",
            params.track
        ))
    }

    /// Set track input routing channel.
    #[tool(description = "Set track input routing channel")]
    pub async fn set_track_input_routing_channel(
        &self,
        Parameters(params): Parameters<SetTrackRoutingChannelParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/input_routing_channel",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::String(params.channel.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} input routing channel set to {}",
            params.track, params.channel
        ))
    }

    /// Get track output routing type.
    #[tool(description = "Get track output routing type")]
    pub async fn get_track_output_routing_type(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let routing = self.query_track_output_routing_type(params.track).await?;
        Ok(format!(
            "Track {} output routing type: {routing}",
            params.track
        ))
    }

    /// Set track output routing type.
    #[tool(description = "Set track output routing type")]
    pub async fn set_track_output_routing_type(
        &self,
        Parameters(params): Parameters<SetTrackRoutingTypeParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/output_routing_type",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::String(params.routing_type.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} output routing type set to {}",
            params.track, params.routing_type
        ))
    }

    /// Get track output routing channel.
    #[tool(description = "Get track output routing channel")]
    pub async fn get_track_output_routing_channel(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let channel = self
            .query_track_output_routing_channel(params.track)
            .await?;
        Ok(format!(
            "Track {} output routing channel: {channel}",
            params.track
        ))
    }

    /// Set track output routing channel.
    #[tool(description = "Set track output routing channel")]
    pub async fn set_track_output_routing_channel(
        &self,
        Parameters(params): Parameters<SetTrackRoutingChannelParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/set/output_routing_channel",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::String(params.channel.clone()),
                ],
            )
            .await?;
        Ok(format!(
            "Track {} output routing channel set to {}",
            params.track, params.channel
        ))
    }

    /// Get track capabilities.
    #[tool(description = "Get track capabilities (can be armed, has audio/MIDI I/O, etc.)")]
    pub async fn get_track_capabilities(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let args = vec![OscType::Int(params.track as i32)];

        let can_be_armed: bool = self
            .osc
            .query("/live/track/get/can_be_armed", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let has_audio_input: bool = self
            .osc
            .query("/live/track/get/has_audio_input", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let has_audio_output: bool = self
            .osc
            .query("/live/track/get/has_audio_output", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let has_midi_input: bool = self
            .osc
            .query("/live/track/get/has_midi_input", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let has_midi_output: bool = self
            .osc
            .query("/live/track/get/has_midi_output", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let is_foldable: bool = self
            .osc
            .query("/live/track/get/is_foldable", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let is_grouped: bool = self
            .osc
            .query("/live/track/get/is_grouped", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(false);

        let is_visible: bool = self
            .osc
            .query("/live/track/get/is_visible", args.clone())
            .await
            .map(|v: i32| v != 0)
            .unwrap_or(true);

        let caps = TrackCapabilities {
            can_be_armed,
            has_audio_input,
            has_audio_output,
            has_midi_input,
            has_midi_output,
            is_foldable,
            is_grouped,
            is_visible,
        };

        Ok(serde_json::to_string_pretty(&caps).unwrap_or_else(|_| format!("{caps:?}")))
    }

    /// Check if track can be armed.
    #[tool(description = "Check if track can be armed for recording")]
    pub async fn can_track_be_armed(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/can_be_armed",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let can_arm = result != 0;
        Ok(format!(
            "Track {} {}be armed",
            params.track,
            if can_arm { "can " } else { "cannot " }
        ))
    }

    /// Check if track has audio input.
    #[tool(description = "Check if track has audio input")]
    pub async fn has_track_audio_input(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/has_audio_input",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Track {} {}have audio input",
            params.track,
            if has { "does " } else { "does not " }
        ))
    }

    /// Check if track has audio output.
    #[tool(description = "Check if track has audio output")]
    pub async fn has_track_audio_output(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/has_audio_output",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Track {} {}have audio output",
            params.track,
            if has { "does " } else { "does not " }
        ))
    }

    /// Check if track has MIDI input.
    #[tool(description = "Check if track has MIDI input")]
    pub async fn has_track_midi_input(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/has_midi_input",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Track {} {}have MIDI input",
            params.track,
            if has { "does " } else { "does not " }
        ))
    }

    /// Check if track has MIDI output.
    #[tool(description = "Check if track has MIDI output")]
    pub async fn has_track_midi_output(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/has_midi_output",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let has = result != 0;
        Ok(format!(
            "Track {} {}have MIDI output",
            params.track,
            if has { "does " } else { "does not " }
        ))
    }

    /// Check if track is visible.
    #[tool(description = "Check if track is visible")]
    pub async fn is_track_visible(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let result: i32 = self
            .osc
            .query(
                "/live/track/get/is_visible",
                vec![OscType::Int(params.track as i32)],
            )
            .await?;
        let visible = result != 0;
        Ok(format!(
            "Track {} is {}visible",
            params.track,
            if visible { "" } else { "not " }
        ))
    }

    /// Get available input routing types.
    #[tool(description = "Get available input routing types for a track")]
    pub async fn get_available_input_routing_types(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let types = self
            .query_available_input_routing_types(params.track)
            .await?;
        Ok(serde_json::to_string_pretty(&types).unwrap_or_else(|_| format!("{types:?}")))
    }

    /// Get available input routing channels.
    #[tool(description = "Get available input routing channels for a track")]
    pub async fn get_available_input_routing_channels(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let channels = self
            .query_available_input_routing_channels(params.track)
            .await?;
        Ok(serde_json::to_string_pretty(&channels).unwrap_or_else(|_| format!("{channels:?}")))
    }

    /// Get available output routing types.
    #[tool(description = "Get available output routing types for a track")]
    pub async fn get_available_output_routing_types(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let types = self
            .query_available_output_routing_types(params.track)
            .await?;
        Ok(serde_json::to_string_pretty(&types).unwrap_or_else(|_| format!("{types:?}")))
    }

    /// Get available output routing channels.
    #[tool(description = "Get available output routing channels for a track")]
    pub async fn get_available_output_routing_channels(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let channels = self
            .query_available_output_routing_channels(params.track)
            .await?;
        Ok(serde_json::to_string_pretty(&channels).unwrap_or_else(|_| format!("{channels:?}")))
    }

    /// Get complete input routing options.
    #[tool(
        description = "Get complete input routing options for a track (available types, channels, and current settings)"
    )]
    pub async fn get_input_routing_options(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let available_types = self
            .query_available_input_routing_types(params.track)
            .await
            .unwrap_or_default();
        let available_channels = self
            .query_available_input_routing_channels(params.track)
            .await
            .unwrap_or_default();
        let current_type = self
            .query_track_input_routing_type(params.track)
            .await
            .unwrap_or_default();
        let current_channel = self
            .query_track_input_routing_channel(params.track)
            .await
            .unwrap_or_default();

        let options = RoutingOptions {
            available_types,
            available_channels,
            current_type,
            current_channel,
        };
        Ok(serde_json::to_string_pretty(&options).unwrap_or_else(|_| format!("{options:?}")))
    }

    /// Get complete output routing options.
    #[tool(
        description = "Get complete output routing options for a track (available types, channels, and current settings)"
    )]
    pub async fn get_output_routing_options(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let available_types = self
            .query_available_output_routing_types(params.track)
            .await
            .unwrap_or_default();
        let available_channels = self
            .query_available_output_routing_channels(params.track)
            .await
            .unwrap_or_default();
        let current_type = self
            .query_track_output_routing_type(params.track)
            .await
            .unwrap_or_default();
        let current_channel = self
            .query_track_output_routing_channel(params.track)
            .await
            .unwrap_or_default();

        let options = RoutingOptions {
            available_types,
            available_channels,
            current_type,
            current_channel,
        };
        Ok(serde_json::to_string_pretty(&options).unwrap_or_else(|_| format!("{options:?}")))
    }

    /// Get all clip names on a track.
    #[tool(description = "Get all clip names on a track (null for empty slots)")]
    pub async fn get_track_clip_names(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/clips/name",
                vec![OscType::Int(params.track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut names = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    match arg {
                        OscType::String(s) => names.push(Some(s)),
                        OscType::Nil => names.push(None),
                        _ => {}
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| format!("{names:?}")))
    }

    /// Get all clip lengths on a track.
    #[tool(description = "Get all clip lengths on a track (null for empty slots)")]
    pub async fn get_track_clip_lengths(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/clips/length",
                vec![OscType::Int(params.track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut lengths: Vec<Option<f32>> = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    match arg {
                        OscType::Float(f) => lengths.push(Some(f)),
                        OscType::Nil => lengths.push(None),
                        _ => {}
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&lengths).unwrap_or_else(|_| format!("{lengths:?}")))
    }

    /// Get all clip colors on a track.
    #[tool(description = "Get all clip colors on a track (null for empty slots)")]
    pub async fn get_track_clip_colors(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/clips/color",
                vec![OscType::Int(params.track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut colors: Vec<Option<i32>> = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    match arg {
                        OscType::Int(i) => colors.push(Some(i)),
                        OscType::Nil => colors.push(None),
                        _ => {}
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&colors).unwrap_or_else(|_| format!("{colors:?}")))
    }

    /// Get arrangement clip names.
    #[tool(description = "Get arrangement clip names for a track")]
    pub async fn get_arrangement_clip_names(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let names = self.query_arrangement_clip_names(params.track).await?;
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| format!("{names:?}")))
    }

    /// Get arrangement clip lengths.
    #[tool(description = "Get arrangement clip lengths for a track")]
    pub async fn get_arrangement_clip_lengths(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let lengths = self.query_arrangement_clip_lengths(params.track).await?;
        Ok(serde_json::to_string_pretty(&lengths).unwrap_or_else(|_| format!("{lengths:?}")))
    }

    /// Get arrangement clip start times.
    #[tool(description = "Get arrangement clip start times for a track")]
    pub async fn get_arrangement_clip_start_times(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let times = self
            .query_arrangement_clip_start_times(params.track)
            .await?;
        Ok(serde_json::to_string_pretty(&times).unwrap_or_else(|_| format!("{times:?}")))
    }

    /// Get all arrangement clips.
    #[tool(description = "Get all arrangement clips for a track (name, length, start_time)")]
    pub async fn get_arrangement_clips(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let names = self.query_arrangement_clip_names(params.track).await?;
        let lengths = self.query_arrangement_clip_lengths(params.track).await?;
        let start_times = self
            .query_arrangement_clip_start_times(params.track)
            .await?;

        let mut clips = Vec::new();
        for i in 0..names.len().min(lengths.len()).min(start_times.len()) {
            clips.push(ArrangementClipInfo {
                name: names[i].clone(),
                length: lengths[i],
                start_time: start_times[i],
            });
        }
        Ok(serde_json::to_string_pretty(&clips).unwrap_or_else(|_| format!("{clips:?}")))
    }

    /// Get all device names on a track.
    #[tool(description = "Get all device names on a track")]
    pub async fn get_device_names(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/devices/name",
                vec![OscType::Int(params.track as i32)],
            )
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
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| format!("{names:?}")))
    }

    /// Get all device types on a track.
    #[tool(
        description = "Get all device types on a track (0=audio effect, 1=instrument, 2=midi effect)"
    )]
    pub async fn get_device_types(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/devices/type",
                vec![OscType::Int(params.track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut types = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Int(i) = arg {
                        types.push(i);
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&types).unwrap_or_else(|_| format!("{types:?}")))
    }

    /// Get all device class names on a track.
    #[tool(description = "Get all device class names on a track")]
    pub async fn get_device_class_names(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/devices/class_name",
                vec![OscType::Int(params.track as i32)],
            )
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
        Ok(serde_json::to_string_pretty(&names).unwrap_or_else(|_| format!("{names:?}")))
    }

    /// Delete a clip on a track.
    #[tool(description = "Delete a clip on a track at the specified slot")]
    pub async fn delete_track_clip(
        &self,
        Parameters(params): Parameters<ClipSlotParams>,
    ) -> Result<String, Error> {
        self.osc
            .send(
                "/live/track/delete_clip",
                vec![
                    OscType::Int(params.track as i32),
                    OscType::Int(params.slot as i32),
                ],
            )
            .await?;
        Ok(format!(
            "Deleted clip at track {}, slot {}",
            params.track, params.slot
        ))
    }

    // ========== Helper methods for internal use ==========

    /// Query track input routing type.
    async fn query_track_input_routing_type(&self, track: u32) -> Result<String, Error> {
        self.osc
            .query(
                "/live/track/get/input_routing_type",
                vec![OscType::Int(track as i32)],
            )
            .await
    }

    /// Query track input routing channel.
    async fn query_track_input_routing_channel(&self, track: u32) -> Result<String, Error> {
        self.osc
            .query(
                "/live/track/get/input_routing_channel",
                vec![OscType::Int(track as i32)],
            )
            .await
    }

    /// Query track output routing type.
    async fn query_track_output_routing_type(&self, track: u32) -> Result<String, Error> {
        self.osc
            .query(
                "/live/track/get/output_routing_type",
                vec![OscType::Int(track as i32)],
            )
            .await
    }

    /// Query track output routing channel.
    async fn query_track_output_routing_channel(&self, track: u32) -> Result<String, Error> {
        self.osc
            .query(
                "/live/track/get/output_routing_channel",
                vec![OscType::Int(track as i32)],
            )
            .await
    }

    /// Query available input routing types for a track.
    async fn query_available_input_routing_types(&self, track: u32) -> Result<Vec<String>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/available_input_routing_types",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut types = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        types.push(s);
                    }
                }
            }
        }
        Ok(types)
    }

    /// Query available input routing channels for a track.
    async fn query_available_input_routing_channels(
        &self,
        track: u32,
    ) -> Result<Vec<String>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/available_input_routing_channels",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut channels = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        channels.push(s);
                    }
                }
            }
        }
        Ok(channels)
    }

    /// Query available output routing types for a track.
    async fn query_available_output_routing_types(&self, track: u32) -> Result<Vec<String>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/available_output_routing_types",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut types = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        types.push(s);
                    }
                }
            }
        }
        Ok(types)
    }

    /// Query available output routing channels for a track.
    async fn query_available_output_routing_channels(
        &self,
        track: u32,
    ) -> Result<Vec<String>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/available_output_routing_channels",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut channels = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        channels.push(s);
                    }
                }
            }
        }
        Ok(channels)
    }

    /// Query arrangement clip names for a track.
    async fn query_arrangement_clip_names(&self, track: u32) -> Result<Vec<String>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/arrangement_clips/name",
                vec![OscType::Int(track as i32)],
            )
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
        Ok(names)
    }

    /// Query arrangement clip lengths for a track.
    async fn query_arrangement_clip_lengths(&self, track: u32) -> Result<Vec<f32>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/arrangement_clips/length",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut lengths = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Float(f) = arg {
                        lengths.push(f);
                    }
                }
            }
        }
        Ok(lengths)
    }

    /// Query arrangement clip start times for a track.
    async fn query_arrangement_clip_start_times(&self, track: u32) -> Result<Vec<f32>, Error> {
        let packets = self
            .osc
            .query_all(
                "/live/track/get/arrangement_clips/start_time",
                vec![OscType::Int(track as i32)],
            )
            .await
            .unwrap_or_default();

        let mut times = Vec::new();
        for packet in packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Float(f) = arg {
                        times.push(f);
                    }
                }
            }
        }
        Ok(times)
    }
}
