//! Parameter structs for MCP tools.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Track information returned from `list_tracks`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub index: u32,
    pub name: String,
    pub armed: bool,
    pub muted: bool,
    pub soloed: bool,
    pub volume: f32,
    pub pan: f32,
}

/// Scene information returned from `list_scenes`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneInfo {
    pub index: u32,
    pub name: String,
}

/// Clip information returned from `get_clip_info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipInfo {
    pub track: u32,
    pub slot: u32,
    pub name: String,
    pub length: f32,
    pub is_playing: bool,
    pub is_recording: bool,
    pub is_triggered: bool,
}

/// Device information returned from `list_devices`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub index: u32,
    pub name: String,
    pub class_name: String,
}

/// Device parameter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub index: u32,
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

/// Song information returned from `get_song_info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongInfo {
    pub tempo: f32,
    pub is_playing: bool,
    pub current_time: f32,
    pub track_count: u32,
    pub scene_count: u32,
}

/// A MIDI note in a clip.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MidiNote {
    /// MIDI pitch (0-127).
    #[schemars(description = "MIDI pitch (0-127)")]
    pub pitch: u8,
    /// Start time in beats.
    #[schemars(description = "Start time in beats")]
    pub start_time: f32,
    /// Duration in beats.
    #[schemars(description = "Duration in beats")]
    pub duration: f32,
    /// Velocity (0-127).
    #[schemars(description = "Velocity (0-127)")]
    pub velocity: u8,
    /// Mute state.
    #[schemars(description = "Whether the note is muted")]
    pub muted: bool,
}

/// A cue point in the arrangement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuePoint {
    /// Cue point ID.
    pub id: u32,
    /// Time position in beats.
    pub time: f32,
    /// Cue point name.
    pub name: String,
}

/// Clip loop bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipLoopBounds {
    pub start: f32,
    pub end: f32,
}

/// Extended clip information with all properties.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipDetailedInfo {
    pub track: u32,
    pub slot: u32,
    pub name: String,
    pub length: f32,
    pub is_playing: bool,
    pub is_recording: bool,
    pub is_triggered: bool,
    // Additional properties
    pub is_midi_clip: bool,
    pub is_audio_clip: bool,
    pub start_time: f32,
    pub end_time: f32,
    pub loop_start: f32,
    pub loop_end: f32,
    pub looping: bool,
    pub muted: bool,
    pub color: i32,
    pub playing_position: f32,
}

/// Track capability information.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackCapabilities {
    pub can_be_armed: bool,
    pub has_audio_input: bool,
    pub has_audio_output: bool,
    pub has_midi_input: bool,
    pub has_midi_output: bool,
    pub is_foldable: bool,
    pub is_grouped: bool,
    pub is_visible: bool,
}

/// Routing options for a track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingOptions {
    pub available_types: Vec<String>,
    pub available_channels: Vec<String>,
    pub current_type: String,
    pub current_channel: String,
}

/// Arrangement clip information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangementClipInfo {
    pub name: String,
    pub length: f32,
    pub start_time: f32,
}

/// Song structure for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongStructure {
    pub tracks: Vec<TrackStructure>,
}

/// Track structure for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackStructure {
    pub index: u32,
    pub name: String,
    pub is_foldable: bool,
    pub group_track: Option<u32>,
    pub clips: Vec<ClipStructure>,
    pub devices: Vec<DeviceStructure>,
}

/// Clip structure for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipStructure {
    pub index: u32,
    pub name: String,
    pub length: f32,
}

/// Device structure for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStructure {
    pub index: u32,
    pub name: String,
    pub class_name: String,
    pub device_type: i32,
    pub parameters: Vec<ParameterStructure>,
}

/// Parameter structure for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterStructure {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub is_quantized: bool,
}

/// Extended song information.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongDetailedInfo {
    pub tempo: f32,
    pub is_playing: bool,
    pub current_time: f32,
    pub song_length: f32,
    pub track_count: u32,
    pub scene_count: u32,
    pub can_undo: bool,
    pub can_redo: bool,
    pub signature_numerator: i32,
    pub signature_denominator: i32,
    pub groove_amount: f32,
    pub metronome: bool,
    pub loop_enabled: bool,
    pub loop_start: f32,
    pub loop_length: f32,
    pub root_note: i32,
    pub scale_name: String,
}
