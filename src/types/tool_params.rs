//! Parameter structs for MCP tools using `#[tool]` macro.
//!
//! Each struct represents the input parameters for a tool and derives:
//! - `Deserialize` for parsing JSON input
//! - `JsonSchema` for generating the tool's input schema

use schemars::JsonSchema;
use serde::Deserialize;

use crate::types::MidiNote;

// =============================================================================
// Transport Parameters
// =============================================================================

/// Parameters for `set_tempo` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTempoParams {
    /// Tempo in beats per minute (20-999).
    #[schemars(description = "Tempo in beats per minute (20-999)")]
    pub bpm: f32,
}

/// Parameters for `set_time` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimeParams {
    /// Position in beats.
    #[schemars(description = "Position in beats")]
    pub beats: f32,
}

/// Parameters for `set_metronome` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetMetronomeParams {
    /// Whether to enable the metronome.
    #[schemars(description = "Whether to enable the metronome")]
    pub enabled: bool,
}

// =============================================================================
// Track Parameters
// =============================================================================

/// Parameters for tools that only require a track index.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TrackParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
}

/// Parameters for `set_track_volume` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackVolumeParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Volume level (0.0 to 1.0).
    #[schemars(description = "Volume level (0.0 to 1.0)")]
    pub volume: f32,
}

/// Parameters for `set_track_pan` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackPanParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Pan position (-1.0 left to 1.0 right).
    #[schemars(description = "Pan position (-1.0 left to 1.0 right)")]
    pub pan: f32,
}

/// Parameters for `mute_track` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MuteTrackParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Whether to mute the track.
    #[schemars(description = "Whether to mute the track")]
    pub mute: bool,
}

/// Parameters for `solo_track` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SoloTrackParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Whether to solo the track.
    #[schemars(description = "Whether to solo the track")]
    pub solo: bool,
}

/// Parameters for `arm_track` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArmTrackParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Whether to arm the track.
    #[schemars(description = "Whether to arm the track")]
    pub arm: bool,
}

/// Parameters for `create_midi_track` and `create_audio_track` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTrackParams {
    /// Optional index to insert the track at.
    #[schemars(description = "Optional index to insert the track at")]
    pub index: Option<i32>,
}

/// Parameters for `set_track_name` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackNameParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// New name for the track.
    #[schemars(description = "New name for the track")]
    pub name: String,
}

/// Parameters for `set_track_send` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackSendParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Send index (0-based).
    #[schemars(description = "Send index (0-based)")]
    pub send: u32,
    /// Send level (0.0 to 1.0).
    #[schemars(description = "Send level (0.0 to 1.0)")]
    pub level: f32,
}

/// Parameters for `get_track_send` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTrackSendParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Send index (0-based).
    #[schemars(description = "Send index (0-based)")]
    pub send: u32,
}

/// Parameters for `set_track_color` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackColorParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// RGB color as integer.
    #[schemars(description = "RGB color as integer")]
    pub color: i32,
}

/// Parameters for `set_track_monitoring` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackMonitoringParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Monitoring state (0=In, 1=Auto, 2=Off).
    #[schemars(description = "Monitoring state (0=In, 1=Auto, 2=Off)")]
    pub state: i32,
}

/// Parameters for `set_track_fold_state` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackFoldStateParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Whether to fold the track.
    #[schemars(description = "Whether to fold the track")]
    pub folded: bool,
}

/// Parameters for `set_track_input_routing_type` and `set_track_output_routing_type` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackRoutingTypeParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Routing type name.
    #[schemars(description = "Routing type name")]
    pub routing_type: String,
}

/// Parameters for `set_track_input_routing_channel` and `set_track_output_routing_channel` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTrackRoutingChannelParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Routing channel name.
    #[schemars(description = "Routing channel name")]
    pub channel: String,
}

// =============================================================================
// Clip Parameters
// =============================================================================

/// Parameters for tools that require track and slot indices.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClipSlotParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
}

/// Parameters for `set_clip_name` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipNameParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// New name for the clip.
    #[schemars(description = "New name for the clip")]
    pub name: String,
}

/// Parameters for `create_clip` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateClipParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Length of the clip in beats.
    #[schemars(description = "Length of the clip in beats")]
    pub length: f32,
}

/// Parameters for setting clip loop start or end.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLoopPointParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Position in beats.
    #[schemars(description = "Position in beats")]
    pub position: f32,
}

/// Parameters for `add_clip_notes` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddClipNotesParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// MIDI notes to add.
    #[schemars(description = "MIDI notes to add")]
    pub notes: Vec<MidiNote>,
}

/// Parameters for `remove_clip_notes` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveClipNotesParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Start time in beats.
    #[schemars(description = "Start time in beats")]
    pub start_time: f32,
    /// End time in beats.
    #[schemars(description = "End time in beats")]
    pub end_time: f32,
    /// Start pitch (0-127).
    #[schemars(description = "Start pitch (0-127)")]
    pub pitch_start: u8,
    /// End pitch (0-127).
    #[schemars(description = "End pitch (0-127)")]
    pub pitch_end: u8,
}

/// Parameters for `set_clip_color` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipColorParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// RGB color as integer.
    #[schemars(description = "RGB color as integer")]
    pub color: i32,
}

/// Parameters for `set_clip_gain` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipGainParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Gain value.
    #[schemars(description = "Gain value")]
    pub gain: f32,
}

/// Parameters for `set_clip_pitch` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipPitchParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Pitch in semitones (-48 to +48).
    #[schemars(description = "Pitch in semitones (-48 to +48)")]
    pub semitones: i32,
}

/// Parameters for `set_clip_warp` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipWarpParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether to enable warping.
    #[schemars(description = "Whether to enable warping")]
    pub enabled: bool,
}

/// Parameters for `set_clip_warp_mode` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipWarpModeParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Warp mode (0=Beats, 1=Tones, 2=Texture, 3=Re-Pitch, 4=Complex, 5=Complex Pro).
    #[schemars(
        description = "Warp mode (0=Beats, 1=Tones, 2=Texture, 3=Re-Pitch, 4=Complex, 5=Complex Pro)"
    )]
    pub mode: i32,
}

/// Parameters for `set_clip_loop_bounds` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLoopBoundsParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Loop start in beats.
    #[schemars(description = "Loop start in beats")]
    pub start: f32,
    /// Loop end in beats.
    #[schemars(description = "Loop end in beats")]
    pub end: f32,
}

/// Parameters for `set_clip_launch_mode` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLaunchModeParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Launch mode (0=Trigger, 1=Gate, 2=Toggle, 3=Repeat).
    #[schemars(description = "Launch mode (0=Trigger, 1=Gate, 2=Toggle, 3=Repeat)")]
    pub mode: i32,
}

/// Parameters for `set_clip_launch_quantization` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLaunchQuantizationParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Quantization value.
    #[schemars(description = "Quantization value")]
    pub quantization: i32,
}

/// Parameters for `duplicate_clip_to` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplicateClipToParams {
    /// Source track index (0-based).
    #[schemars(description = "Source track index (0-based)")]
    pub src_track: u32,
    /// Source clip slot index (0-based).
    #[schemars(description = "Source clip slot index (0-based)")]
    pub src_slot: u32,
    /// Destination track index (0-based).
    #[schemars(description = "Destination track index (0-based)")]
    pub dst_track: u32,
    /// Destination clip slot index (0-based).
    #[schemars(description = "Destination clip slot index (0-based)")]
    pub dst_slot: u32,
}

/// Parameters for `set_clip_slot_has_stop_button` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipSlotHasStopButtonParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether the slot has a stop button.
    #[schemars(description = "Whether the slot has a stop button")]
    pub has_stop_button: bool,
}

/// Parameters for `set_clip_looping` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLoopingParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether looping is enabled.
    #[schemars(description = "Whether looping is enabled")]
    pub looping: bool,
}

/// Parameters for `set_clip_muted` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipMutedParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether the clip is muted.
    #[schemars(description = "Whether the clip is muted")]
    pub muted: bool,
}

/// Parameters for `set_clip_position` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipPositionParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Position in beats.
    #[schemars(description = "Position in beats")]
    pub position: f32,
}

/// Parameters for `set_clip_start_marker` and `set_clip_end_marker` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipMarkerParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Marker position in beats.
    #[schemars(description = "Marker position in beats")]
    pub marker: f32,
}

/// Parameters for `set_clip_legato` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipLegatoParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether legato is enabled.
    #[schemars(description = "Whether legato is enabled")]
    pub legato: bool,
}

/// Parameters for `set_clip_velocity_amount` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipVelocityAmountParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Velocity amount (-1.0 to 1.0).
    #[schemars(description = "Velocity amount (-1.0 to 1.0)")]
    pub amount: f32,
}

/// Parameters for `set_clip_color_index` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipColorIndexParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Color index.
    #[schemars(description = "Color index")]
    pub color_index: i32,
}

/// Parameters for `set_clip_pitch_fine` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipPitchFineParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Pitch fine in cents (-50 to 50).
    #[schemars(description = "Pitch fine in cents (-50 to 50)")]
    pub cents: f32,
}

/// Parameters for `set_clip_ram_mode` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetClipRamModeParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
    /// Whether RAM mode is enabled.
    #[schemars(description = "Whether RAM mode is enabled")]
    pub enabled: bool,
}

// =============================================================================
// Scene Parameters
// =============================================================================

/// Parameters for tools that only require a scene index.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SceneParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
}

/// Parameters for `create_scene` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateSceneParams {
    /// Optional index to insert the scene at.
    #[schemars(description = "Optional index to insert the scene at")]
    pub index: Option<i32>,
}

/// Parameters for `set_scene_name` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneNameParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// New name for the scene.
    #[schemars(description = "New name for the scene")]
    pub name: String,
}

/// Parameters for `set_scene_color` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneColorParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// RGB color as integer.
    #[schemars(description = "RGB color as integer")]
    pub color: i32,
}

/// Parameters for `set_scene_tempo` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneTempoParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// Scene tempo in BPM.
    #[schemars(description = "Scene tempo in BPM")]
    pub tempo: f32,
}

/// Parameters for `set_scene_tempo_enabled` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneTempoEnabledParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// Whether scene tempo is enabled.
    #[schemars(description = "Whether scene tempo is enabled")]
    pub enabled: bool,
}

/// Parameters for `set_scene_time_signature` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneTimeSignatureParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// Time signature numerator.
    #[schemars(description = "Time signature numerator")]
    pub numerator: i32,
    /// Time signature denominator.
    #[schemars(description = "Time signature denominator")]
    pub denominator: i32,
}

/// Parameters for `set_scene_time_sig_enabled` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSceneTimeSigEnabledParams {
    /// Scene index (0-based).
    #[schemars(description = "Scene index (0-based)")]
    pub scene: u32,
    /// Whether scene time signature is enabled.
    #[schemars(description = "Whether scene time signature is enabled")]
    pub enabled: bool,
}

// =============================================================================
// Device Parameters
// =============================================================================

/// Parameters for tools that require track and device indices.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeviceParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
}

/// Parameters for `set_device_parameter` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetDeviceParameterParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
    /// Parameter index (0-based).
    #[schemars(description = "Parameter index (0-based)")]
    pub param: u32,
    /// Parameter value.
    #[schemars(description = "Parameter value")]
    pub value: f32,
}

/// Parameters for `set_device_enabled` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetDeviceEnabledParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
    /// Whether to enable the device.
    #[schemars(description = "Whether to enable the device")]
    pub enabled: bool,
}

/// Parameters for `get_parameter_value_string` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetParameterValueStringParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
    /// Parameter index (0-based).
    #[schemars(description = "Parameter index (0-based)")]
    pub param: u32,
}

/// Parameters for `set_all_device_parameters` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetAllDeviceParametersParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
    /// Array of parameter values.
    #[schemars(description = "Array of parameter values")]
    pub values: Vec<f32>,
}

// =============================================================================
// Song Parameters
// =============================================================================

/// Parameters for `set_loop_start` and `set_loop_length` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetLoopBeatsParams {
    /// Position/length in beats.
    #[schemars(description = "Position/length in beats")]
    pub beats: f32,
}

/// Parameters for `set_loop_enabled` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetLoopEnabledParams {
    /// Whether to enable loop playback.
    #[schemars(description = "Whether to enable loop playback")]
    pub enabled: bool,
}

/// Parameters for `set_quantization` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetQuantizationParams {
    /// Quantization value (0=None, 1=8 Bars, 2=4 Bars, 3=2 Bars, 4=1 Bar, 5=1/2, etc.).
    #[schemars(
        description = "Quantization value (0=None, 1=8 Bars, 2=4 Bars, 3=2 Bars, 4=1 Bar, 5=1/2, etc.)"
    )]
    pub quantization: i32,
}

/// Parameters for `set_groove_amount` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetGrooveAmountParams {
    /// Groove amount (0.0 to 1.0).
    #[schemars(description = "Groove amount (0.0 to 1.0)")]
    pub amount: f32,
}

/// Parameters for `set_signature_numerator` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSignatureNumeratorParams {
    /// Time signature numerator.
    #[schemars(description = "Time signature numerator")]
    pub numerator: i32,
}

/// Parameters for `set_signature_denominator` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSignatureDenominatorParams {
    /// Time signature denominator.
    #[schemars(description = "Time signature denominator")]
    pub denominator: i32,
}

/// Parameters for boolean toggle tools (punch in/out, overdub, session record, etc.).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetEnabledParams {
    /// Whether to enable the feature.
    #[schemars(description = "Whether to enable the feature")]
    pub enabled: bool,
}

/// Parameters for `delete_return_track` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteReturnTrackParams {
    /// Return track index (0-based).
    #[schemars(description = "Return track index (0-based)")]
    pub index: u32,
}

/// Parameters for `jump_by` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct JumpByParams {
    /// Beats to jump by (positive or negative).
    #[schemars(description = "Beats to jump by (positive or negative)")]
    pub beats: f32,
}

/// Parameters for `set_root_note` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetRootNoteParams {
    /// Root note (0-11, where 0=C).
    #[schemars(description = "Root note (0-11, where 0=C, 1=C#, ..., 11=B)")]
    pub root_note: i32,
}

/// Parameters for `set_scale_name` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetScaleNameParams {
    /// Scale name (e.g., "Major", "Minor", "Dorian").
    #[schemars(description = "Scale name (e.g., 'Major', 'Minor', 'Dorian')")]
    pub scale_name: String,
}

/// Parameters for `set_current_time` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCurrentTimeParams {
    /// Time position in beats.
    #[schemars(description = "Time position in beats")]
    pub time: f32,
}

// =============================================================================
// View/Selection Parameters
// =============================================================================

/// Parameters for `set_selected_clip` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSelectedClipParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Clip slot index (0-based).
    #[schemars(description = "Clip slot index (0-based)")]
    pub slot: u32,
}

/// Parameters for `set_selected_device` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSelectedDeviceParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
}

// =============================================================================
// Cue Point Parameters
// =============================================================================

/// Parameters for `jump_to_cue_point` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct JumpToCuePointParams {
    /// Cue point index (0-based).
    #[schemars(description = "Cue point index (0-based)")]
    pub index: u32,
}

/// Parameters for `set_cue_point_name` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCuePointNameParams {
    /// Cue point index (0-based).
    #[schemars(description = "Cue point index (0-based)")]
    pub index: u32,
    /// New name for the cue point.
    #[schemars(description = "New name for the cue point")]
    pub name: String,
}

// =============================================================================
// Browser Parameters
// =============================================================================

/// Parameters for tools that require a name (instrument, effect, sound, etc.).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadByNameParams {
    /// Name of the item to load.
    #[schemars(description = "Name of the item to load")]
    pub name: String,
}

/// Parameters for `load_drum_kit` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadDrumKitParams {
    /// Optional drum kit name (loads default if not specified).
    #[schemars(description = "Optional drum kit name (loads default if not specified)")]
    pub name: Option<String>,
}

/// Parameters for `list_samples` and `list_clips` tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListWithOptionalCategoryParams {
    /// Optional category to filter items.
    #[schemars(description = "Optional category to filter items")]
    pub category: Option<String>,
}

/// Parameters for `browse` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowseParams {
    /// Category to browse: instruments, drums, sounds, effects, etc.
    #[schemars(
        description = "Category to browse (instruments, drums, sounds, audio_effects, midi_effects, max_for_live, plugins, clips, samples, packs, user_library)"
    )]
    pub category: String,
}

/// Parameters for `browse_path` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrowsePathParams {
    /// Category to browse.
    #[schemars(description = "Category to browse")]
    pub category: String,
    /// Path within the category.
    #[schemars(description = "Path within the category")]
    pub path: String,
}

/// Parameters for `search_browser` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchBrowserParams {
    /// Search query.
    #[schemars(description = "Search query")]
    pub query: String,
}

/// Parameters for `get_browser_item` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetBrowserItemParams {
    /// Category containing the item.
    #[schemars(description = "Category containing the item")]
    pub category: String,
    /// Name of the item.
    #[schemars(description = "Name of the item")]
    pub name: String,
}

/// Parameters for `load_user_preset` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadUserPresetParams {
    /// Path to the preset in user library.
    #[schemars(description = "Path to the preset in user library")]
    pub path: String,
}

/// Parameters for `hotswap_start` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HotswapStartParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
}

// =============================================================================
// Application Parameters
// =============================================================================

/// Parameters for `show_message` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShowMessageParams {
    /// Message to display in Ableton's status bar.
    #[schemars(description = "Message to display in Ableton's status bar")]
    pub message: String,
}

// =============================================================================
// MIDI Map Parameters
// =============================================================================

/// Parameters for `map_midi_cc` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapMidiCcParams {
    /// Track index (0-based).
    #[schemars(description = "Track index (0-based)")]
    pub track: u32,
    /// Device index (0-based).
    #[schemars(description = "Device index (0-based)")]
    pub device: u32,
    /// Parameter index (0-based).
    #[schemars(description = "Parameter index (0-based)")]
    pub parameter: u32,
    /// MIDI channel (0-15).
    #[schemars(description = "MIDI channel (0-15)")]
    pub channel: u32,
    /// MIDI CC number (0-127).
    #[schemars(description = "MIDI CC number (0-127)")]
    pub cc: u32,
}
