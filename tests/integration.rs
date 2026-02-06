//! Integration tests for Ableton MCP Server.
//!
//! These tests require Ableton Live with `AbletonOSC` to be running.
//!
//! Run with:
//!   cargo test --test integration -- --ignored --test-threads=1
//!
//! Note: `--test-threads=1` is recommended to avoid race conditions when
//! multiple tests interact with the same Ableton Live instance.
//!
//! For a quick smoke test:
//!   cargo test --test integration -- --ignored --test-threads=1 test_basic

use remix_mcp::osc::OscClient;
use rosc::{OscPacket, OscType};
use tokio::time::{Duration, sleep};

/// Create a test client bound to an ephemeral port.
async fn create_test_client() -> OscClient {
    OscClient::new().await.expect("Failed to create OSC client")
}

/// Extract strings from OSC packets.
fn extract_strings_from_packets(packets: Vec<OscPacket>) -> Vec<String> {
    packets
        .into_iter()
        .filter_map(|packet| match packet {
            OscPacket::Message(msg) => Some(msg.args),
            OscPacket::Bundle(_) => None,
        })
        .flatten()
        .filter_map(|arg| match arg {
            OscType::String(s) => Some(s),
            _ => None,
        })
        .collect()
}

/// Extract a boolean from OSC response at a given index.
/// `AbletonOSC` clip responses have format: (`track_id`, `scene_id`, value)
fn extract_bool_at_index(packets: Vec<OscPacket>, index: usize) -> Option<bool> {
    packets.into_iter().find_map(|packet| match packet {
        OscPacket::Message(msg) => msg.args.get(index).and_then(|arg| match arg {
            OscType::Bool(v) => Some(*v),
            OscType::Int(v) => Some(*v != 0),
            _ => None,
        }),
        OscPacket::Bundle(_) => None,
    })
}

// ============================================================================
// Basic Connection Tests
// ============================================================================

/// Test that we can create an OSC client (uses ephemeral port).
#[tokio::test]
async fn test_osc_client_creation() {
    let client = OscClient::new().await;
    assert!(
        client.is_ok(),
        "Failed to create OSC client: {:?}",
        client.err()
    );
}

/// Test connection to Ableton Live (requires Ableton to be running).
#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_basic_ableton_connection() {
    let client = create_test_client().await;
    let connected = client
        .test_connection()
        .await
        .expect("Connection test failed");
    assert!(connected, "Ableton Live is not responding");
}

// ============================================================================
// Transport Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_basic_get_tempo() {
    let client = create_test_client().await;
    let tempo: f32 = client
        .query("/live/song/get/tempo", vec![])
        .await
        .expect("Failed to get tempo");
    assert!(tempo > 0.0, "Tempo should be positive");
    assert!(
        (20.0..=999.0).contains(&tempo),
        "Tempo should be in valid range"
    );
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_is_playing() {
    let client = create_test_client().await;
    let is_playing: bool = client
        .query("/live/song/get/is_playing", vec![])
        .await
        .expect("Failed to get is_playing");
    // Just verify we got a bool response
    let _ = is_playing;
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_current_song_time() {
    let client = create_test_client().await;
    let time: f32 = client
        .query("/live/song/get/current_song_time", vec![])
        .await
        .expect("Failed to get current_song_time");
    assert!(time >= 0.0, "Current song time should be non-negative");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_metronome() {
    let client = create_test_client().await;
    let metronome: bool = client
        .query("/live/song/get/metronome", vec![])
        .await
        .expect("Failed to get metronome");
    // Just verify we got a bool response
    let _ = metronome;
}

// ============================================================================
// Song Advanced Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_song_length() {
    let client = create_test_client().await;
    let length: f32 = client
        .query("/live/song/get/song_length", vec![])
        .await
        .expect("Failed to get song_length");
    assert!(length >= 0.0, "Song length should be non-negative");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_can_undo() {
    let client = create_test_client().await;
    let can_undo: bool = client
        .query("/live/song/get/can_undo", vec![])
        .await
        .expect("Failed to get can_undo");
    // Just verify we got a bool response
    let _ = can_undo;
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_can_redo() {
    let client = create_test_client().await;
    let can_redo: bool = client
        .query("/live/song/get/can_redo", vec![])
        .await
        .expect("Failed to get can_redo");
    // Just verify we got a bool response
    let _ = can_redo;
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_signature() {
    let client = create_test_client().await;

    let numerator: i32 = client
        .query("/live/song/get/signature_numerator", vec![])
        .await
        .expect("Failed to get signature_numerator");
    assert!(numerator > 0, "Numerator should be positive");

    let denominator: i32 = client
        .query("/live/song/get/signature_denominator", vec![])
        .await
        .expect("Failed to get signature_denominator");
    assert!(denominator > 0, "Denominator should be positive");
    // Common time signatures have denominators that are powers of 2
    assert!(
        [1, 2, 4, 8, 16, 32].contains(&denominator),
        "Denominator should be a power of 2"
    );
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_groove_amount() {
    let client = create_test_client().await;
    let groove: f32 = client
        .query("/live/song/get/groove_amount", vec![])
        .await
        .expect("Failed to get groove_amount");
    assert!(
        (0.0..=1.0).contains(&groove),
        "Groove amount should be 0.0 to 1.0"
    );
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_root_note() {
    let client = create_test_client().await;
    let root_note: i32 = client
        .query("/live/song/get/root_note", vec![])
        .await
        .expect("Failed to get root_note");
    assert!((0..=11).contains(&root_note), "Root note should be 0-11");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_scale_name() {
    let client = create_test_client().await;
    let scale: String = client
        .query("/live/song/get/scale_name", vec![])
        .await
        .expect("Failed to get scale_name");
    assert!(!scale.is_empty(), "Scale name should not be empty");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_loop_settings() {
    let client = create_test_client().await;

    let loop_enabled: bool = client
        .query("/live/song/get/loop", vec![])
        .await
        .expect("Failed to get loop");
    let _ = loop_enabled;

    let loop_start: f32 = client
        .query("/live/song/get/loop_start", vec![])
        .await
        .expect("Failed to get loop_start");
    assert!(loop_start >= 0.0, "Loop start should be non-negative");

    let loop_length: f32 = client
        .query("/live/song/get/loop_length", vec![])
        .await
        .expect("Failed to get loop_length");
    assert!(loop_length > 0.0, "Loop length should be positive");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_quantization() {
    let client = create_test_client().await;
    let quant: i32 = client
        .query("/live/song/get/clip_trigger_quantization", vec![])
        .await
        .expect("Failed to get clip_trigger_quantization");
    // Quantization values: 0=None, 1=8 Bars, ... up to about 13
    assert!(quant >= 0, "Quantization should be non-negative");
}

// ============================================================================
// Track Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_num_tracks() {
    let client = create_test_client().await;
    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");
    assert!(num_tracks >= 0, "Number of tracks should be non-negative");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_name() {
    let client = create_test_client().await;

    // First check if there are any tracks
    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let name: String = client
            .query("/live/track/get/name", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track name");
        // Name can be empty but query should succeed
        assert!(name.len() < 1000, "Track name should be reasonable length");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_volume() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let volume: f32 = client
            .query("/live/track/get/volume", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track volume");
        assert!((0.0..=1.0).contains(&volume), "Volume should be 0.0 to 1.0");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_panning() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let pan: f32 = client
            .query("/live/track/get/panning", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track panning");
        assert!((-1.0..=1.0).contains(&pan), "Panning should be -1.0 to 1.0");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_mute_solo_arm() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let mute: i32 = client
            .query("/live/track/get/mute", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track mute");
        assert!(mute == 0 || mute == 1, "Mute should be 0 or 1");

        let solo: i32 = client
            .query("/live/track/get/solo", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track solo");
        assert!(solo == 0 || solo == 1, "Solo should be 0 or 1");

        // Arm may fail if track can't be armed, so we just check the query works
        let _arm_result: Result<i32, _> = client
            .query("/live/track/get/arm", vec![OscType::Int(0)])
            .await;
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_color() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let color: i32 = client
            .query("/live/track/get/color", vec![OscType::Int(0)])
            .await
            .expect("Failed to get track color");
        // Color is an RGB integer
        assert!(color >= 0, "Color should be non-negative");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_monitoring() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let monitoring: i32 = client
            .query(
                "/live/track/get/current_monitoring_state",
                vec![OscType::Int(0)],
            )
            .await
            .expect("Failed to get monitoring state");
        // 0=In, 1=Auto, 2=Off
        assert!((0..=2).contains(&monitoring), "Monitoring should be 0-2");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_capabilities() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        // Test various capability queries
        let can_be_armed: i32 = client
            .query("/live/track/get/can_be_armed", vec![OscType::Int(0)])
            .await
            .expect("Failed to get can_be_armed");
        assert!(
            can_be_armed == 0 || can_be_armed == 1,
            "can_be_armed should be 0 or 1"
        );

        let is_foldable: i32 = client
            .query("/live/track/get/is_foldable", vec![OscType::Int(0)])
            .await
            .expect("Failed to get is_foldable");
        assert!(
            is_foldable == 0 || is_foldable == 1,
            "is_foldable should be 0 or 1"
        );

        let is_grouped: i32 = client
            .query("/live/track/get/is_grouped", vec![OscType::Int(0)])
            .await
            .expect("Failed to get is_grouped");
        assert!(
            is_grouped == 0 || is_grouped == 1,
            "is_grouped should be 0 or 1"
        );

        let is_visible: i32 = client
            .query("/live/track/get/is_visible", vec![OscType::Int(0)])
            .await
            .expect("Failed to get is_visible");
        assert!(
            is_visible == 0 || is_visible == 1,
            "is_visible should be 0 or 1"
        );
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_io_capabilities() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let has_audio_input: i32 = client
            .query("/live/track/get/has_audio_input", vec![OscType::Int(0)])
            .await
            .expect("Failed to get has_audio_input");
        assert!(
            has_audio_input == 0 || has_audio_input == 1,
            "has_audio_input should be 0 or 1"
        );

        let has_audio_output: i32 = client
            .query("/live/track/get/has_audio_output", vec![OscType::Int(0)])
            .await
            .expect("Failed to get has_audio_output");
        assert!(
            has_audio_output == 0 || has_audio_output == 1,
            "has_audio_output should be 0 or 1"
        );

        let has_midi_input: i32 = client
            .query("/live/track/get/has_midi_input", vec![OscType::Int(0)])
            .await
            .expect("Failed to get has_midi_input");
        assert!(
            has_midi_input == 0 || has_midi_input == 1,
            "has_midi_input should be 0 or 1"
        );

        let has_midi_output: i32 = client
            .query("/live/track/get/has_midi_output", vec![OscType::Int(0)])
            .await
            .expect("Failed to get has_midi_output");
        assert!(
            has_midi_output == 0 || has_midi_output == 1,
            "has_midi_output should be 0 or 1"
        );
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_routing() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let input_type: String = client
            .query("/live/track/get/input_routing_type", vec![OscType::Int(0)])
            .await
            .expect("Failed to get input_routing_type");
        assert!(
            !input_type.is_empty(),
            "Input routing type should not be empty"
        );

        let output_type: String = client
            .query("/live/track/get/output_routing_type", vec![OscType::Int(0)])
            .await
            .expect("Failed to get output_routing_type");
        assert!(
            !output_type.is_empty(),
            "Output routing type should not be empty"
        );
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_meter() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let meter: f32 = client
            .query("/live/track/get/output_meter_level", vec![OscType::Int(0)])
            .await
            .expect("Failed to get output_meter_level");
        assert!(meter >= 0.0, "Meter level should be non-negative");
    }
}

// ============================================================================
// Scene Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_num_scenes() {
    let client = create_test_client().await;
    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");
    assert!(num_scenes >= 0, "Number of scenes should be non-negative");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_scene_properties() {
    let client = create_test_client().await;

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_scenes > 0 {
        let name: String = client
            .query("/live/scene/get/name", vec![OscType::Int(0)])
            .await
            .expect("Failed to get scene name");
        assert!(name.len() < 1000, "Scene name should be reasonable length");

        let color: i32 = client
            .query("/live/scene/get/color", vec![OscType::Int(0)])
            .await
            .expect("Failed to get scene color");
        assert!(color >= 0, "Scene color should be non-negative");
    }
}

// ============================================================================
// Clip Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_clip_slot_has_clip() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_tracks > 0 && num_scenes > 0 {
        let has_clip: i32 = client
            .query(
                "/live/clip_slot/get/has_clip",
                vec![OscType::Int(0), OscType::Int(0)],
            )
            .await
            .expect("Failed to get has_clip");
        assert!(has_clip == 0 || has_clip == 1, "has_clip should be 0 or 1");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_clip_properties() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_tracks > 0 && num_scenes > 0 {
        // Find a clip
        for track in 0..num_tracks.min(8) {
            for scene in 0..num_scenes.min(8) {
                // Check if slot has a clip (handle bool or int response)
                let has_clip: bool = client
                    .query(
                        "/live/clip_slot/get/has_clip",
                        vec![OscType::Int(track), OscType::Int(scene)],
                    )
                    .await
                    .unwrap_or(false);

                if has_clip {
                    // Try to get clip properties - if any fail, try next clip
                    let Ok(name): Result<String, _> = client
                        .query(
                            "/live/clip/get/name",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    assert!(name.len() < 1000, "Clip name should be reasonable length");

                    let Ok(length): Result<f32, _> = client
                        .query(
                            "/live/clip/get/length",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    assert!(length > 0.0, "Clip length should be positive");

                    let Ok(is_playing): Result<bool, _> = client
                        .query(
                            "/live/clip/get/is_playing",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    let _ = is_playing;

                    let Ok(looping): Result<bool, _> = client
                        .query(
                            "/live/clip/get/looping",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    let _ = looping;

                    return; // Found and tested a clip
                }
            }
        }
        // No clips found - that's okay, test passes
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_clip_type_detection() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_tracks > 0 && num_scenes > 0 {
        // Find a clip
        for track in 0..num_tracks.min(8) {
            for scene in 0..num_scenes.min(8) {
                let has_clip: bool = client
                    .query(
                        "/live/clip_slot/get/has_clip",
                        vec![OscType::Int(track), OscType::Int(scene)],
                    )
                    .await
                    .unwrap_or(false);

                if has_clip {
                    // AbletonOSC returns (track_id, scene_id, value) for clip queries
                    // We need to extract the boolean from index 2
                    let midi_response = client
                        .query_all(
                            "/live/clip/get/is_midi_clip",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                        .unwrap_or_default();

                    let audio_response = client
                        .query_all(
                            "/live/clip/get/is_audio_clip",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                        .unwrap_or_default();

                    let Some(is_midi) = extract_bool_at_index(midi_response, 2) else {
                        continue;
                    };
                    let Some(is_audio) = extract_bool_at_index(audio_response, 2) else {
                        continue;
                    };

                    // A clip must be either MIDI or audio, not both
                    assert!(
                        (is_midi && !is_audio) || (!is_midi && is_audio),
                        "Clip at track {track}, scene {scene} has is_midi={is_midi}, is_audio={is_audio}"
                    );

                    return; // Found and tested a clip
                }
            }
        }
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_clip_loop_properties() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_tracks > 0 && num_scenes > 0 {
        // Find a clip
        for track in 0..num_tracks.min(8) {
            for scene in 0..num_scenes.min(8) {
                let has_clip: bool = client
                    .query(
                        "/live/clip_slot/get/has_clip",
                        vec![OscType::Int(track), OscType::Int(scene)],
                    )
                    .await
                    .unwrap_or(false);

                if has_clip {
                    let Ok(loop_start): Result<f32, _> = client
                        .query(
                            "/live/clip/get/loop_start",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    assert!(loop_start >= 0.0, "Loop start should be non-negative");

                    let Ok(loop_end): Result<f32, _> = client
                        .query(
                            "/live/clip/get/loop_end",
                            vec![OscType::Int(track), OscType::Int(scene)],
                        )
                        .await
                    else {
                        continue;
                    };
                    assert!(
                        loop_end > loop_start,
                        "Loop end should be greater than loop start"
                    );

                    return;
                }
            }
        }
    }
}

// ============================================================================
// Device Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_num_devices() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let num_devices: i32 = client
            .query("/live/track/get/num_devices", vec![OscType::Int(0)])
            .await
            .expect("Failed to get num_devices");
        assert!(num_devices >= 0, "Number of devices should be non-negative");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_device_properties() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        for track in 0..num_tracks.min(4) {
            let num_devices: i32 = client
                .query("/live/track/get/num_devices", vec![OscType::Int(track)])
                .await
                .unwrap_or(0);

            if num_devices > 0 {
                let name: String = client
                    .query(
                        "/live/device/get/name",
                        vec![OscType::Int(track), OscType::Int(0)],
                    )
                    .await
                    .expect("Failed to get device name");
                assert!(name.len() < 1000, "Device name should be reasonable length");

                let class_name: String = client
                    .query(
                        "/live/device/get/class_name",
                        vec![OscType::Int(track), OscType::Int(0)],
                    )
                    .await
                    .expect("Failed to get device class_name");
                assert!(!class_name.is_empty(), "Class name should not be empty");

                let device_type: i32 = client
                    .query(
                        "/live/device/get/type",
                        vec![OscType::Int(track), OscType::Int(0)],
                    )
                    .await
                    .expect("Failed to get device type");
                // 0=audio_effect, 1=instrument, 2=midi_effect
                assert!((0..=2).contains(&device_type), "Device type should be 0-2");

                return; // Found and tested a device
            }
        }
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_device_parameters() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        for track in 0..num_tracks.min(4) {
            let num_devices: i32 = client
                .query("/live/track/get/num_devices", vec![OscType::Int(track)])
                .await
                .unwrap_or(0);

            if num_devices > 0 {
                let num_params: i32 = client
                    .query(
                        "/live/device/get/num_parameters",
                        vec![OscType::Int(track), OscType::Int(0)],
                    )
                    .await
                    .expect("Failed to get num_parameters");
                assert!(
                    num_params >= 0,
                    "Number of parameters should be non-negative"
                );

                if num_params > 0 {
                    // Test getting parameter name and value
                    let param_name: String = client
                        .query(
                            "/live/device/get/parameter/name",
                            vec![OscType::Int(track), OscType::Int(0), OscType::Int(0)],
                        )
                        .await
                        .expect("Failed to get parameter name");
                    assert!(!param_name.is_empty(), "Parameter name should not be empty");

                    let param_value: f32 = client
                        .query(
                            "/live/device/get/parameter/value",
                            vec![OscType::Int(track), OscType::Int(0), OscType::Int(0)],
                        )
                        .await
                        .expect("Failed to get parameter value");
                    // Value can be any float
                    assert!(param_value.is_finite(), "Parameter value should be finite");
                }

                return;
            }
        }
    }
}

// ============================================================================
// View/Selection Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_selected_track() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let selected: i32 = client
            .query("/live/view/get/selected_track", vec![])
            .await
            .expect("Failed to get selected_track");
        assert!(
            selected >= 0 && selected < num_tracks,
            "Selected track should be in valid range"
        );
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_selected_scene() {
    let client = create_test_client().await;

    let num_scenes: i32 = client
        .query("/live/song/get/num_scenes", vec![])
        .await
        .expect("Failed to get num_scenes");

    if num_scenes > 0 {
        let selected: i32 = client
            .query("/live/view/get/selected_scene", vec![])
            .await
            .expect("Failed to get selected_scene");
        assert!(
            selected >= 0 && selected < num_scenes,
            "Selected scene should be in valid range"
        );
    }
}

// ============================================================================
// Application Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_version() {
    let client = create_test_client().await;
    // Version is returned as [major, minor] integers
    let args: Vec<OscType> = client
        .query("/live/application/get/version", vec![])
        .await
        .expect("Failed to get version");
    // Extract major version (first int)
    let major = match args.first() {
        Some(OscType::Int(v)) => *v,
        _ => panic!("Expected major version int"),
    };
    // Ableton Live versions are typically 10, 11, 12, etc.
    assert!(major >= 10, "Major version should be at least 10");
}

// ============================================================================
// Cue Point Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_cue_points() {
    let client = create_test_client().await;

    // This may return an empty list or cue points
    let result = client.query_all("/live/song/get/cue_points", vec![]).await;

    // Query should succeed even if there are no cue points
    assert!(result.is_ok(), "Cue points query should succeed");
}

// ============================================================================
// Bulk Operation Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_track_names_bulk() {
    let client = create_test_client().await;

    let result = client.query_all("/live/song/get/track_names", vec![]).await;
    assert!(result.is_ok(), "Track names bulk query should succeed");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_scene_names_bulk() {
    let client = create_test_client().await;

    let result = client.query_all("/live/song/get/scenes/name", vec![]).await;
    assert!(result.is_ok(), "Scene names bulk query should succeed");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_clip_names_bulk() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let result = client
            .query_all("/live/track/get/clips/name", vec![OscType::Int(0)])
            .await;
        assert!(result.is_ok(), "Clip names bulk query should succeed");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_device_names_bulk() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let result = client
            .query_all("/live/track/get/devices/name", vec![OscType::Int(0)])
            .await;
        assert!(result.is_ok(), "Device names bulk query should succeed");
    }
}

// ============================================================================
// Routing Options Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_available_routing_types() {
    let client = create_test_client().await;

    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    if num_tracks > 0 {
        let input_types = client
            .query_all(
                "/live/track/get/available_input_routing_types",
                vec![OscType::Int(0)],
            )
            .await;
        assert!(
            input_types.is_ok(),
            "Available input routing types query should succeed"
        );

        let output_types = client
            .query_all(
                "/live/track/get/available_output_routing_types",
                vec![OscType::Int(0)],
            )
            .await;
        assert!(
            output_types.is_ok(),
            "Available output routing types query should succeed"
        );
    }
}

// ============================================================================
// Atlanta Hip Hop Twinkle Twinkle Test
// ============================================================================

/// Creates an Atlanta hip hop version of Twinkle Twinkle Little Star
/// with trap drums, 808 bass, and the melody.
#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_create_atlanta_twinkle() {
    let client = create_test_client().await;

    // Stop any current playback
    client
        .send("/live/song/stop_playing", vec![])
        .await
        .expect("Failed to stop playback");
    sleep(Duration::from_millis(200)).await;

    // Set tempo to 140 BPM (Atlanta trap tempo)
    client
        .send("/live/song/set/tempo", vec![OscType::Float(140.0)])
        .await
        .expect("Failed to set tempo");

    // Get current track count
    let initial_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .expect("Failed to get num_tracks");

    // ========== TRACK 1: MELODY (Drift Synth) ==========
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create melody track");
    sleep(Duration::from_millis(300)).await;

    let melody_track = initial_tracks; // New track is at the end

    // Name the track
    client
        .send(
            "/live/track/set/name",
            vec![
                OscType::Int(melody_track),
                OscType::String("ATL Twinkle".into()),
            ],
        )
        .await
        .expect("Failed to name melody track");

    // Select track and load Drift synth
    client
        .send(
            "/live/view/set/selected_track",
            vec![OscType::Int(melody_track)],
        )
        .await
        .expect("Failed to select melody track");
    sleep(Duration::from_millis(200)).await;

    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .expect("Failed to load default instrument");
    sleep(Duration::from_millis(800)).await;

    // Create 16-bar clip (64 beats at 4/4)
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![
                OscType::Int(melody_track),
                OscType::Int(0),
                OscType::Float(64.0),
            ],
        )
        .await
        .expect("Failed to create melody clip");
    sleep(Duration::from_millis(200)).await;

    // Twinkle Twinkle melody adapted for Atlanta trap
    // Original: C C G G A A G | F F E E D D C
    // Using C4=60, D4=62, E4=64, F4=65, G4=67, A4=69
    // Adding syncopation and trap-style rhythms
    let melody_notes: Vec<(i32, f32, f32, i32)> = vec![
        // Bar 1-2: "Twinkle twinkle" - syncopated
        (60, 0.0, 0.75, 100), // C
        (60, 1.0, 0.5, 90),   // C (short)
        (67, 2.0, 0.75, 100), // G
        (67, 3.0, 0.5, 85),   // G
        (69, 4.0, 0.75, 110), // A (accent)
        (69, 5.0, 0.5, 90),   // A
        (67, 6.0, 1.5, 95),   // G (held)
        // Bar 3-4: "Little star"
        (65, 8.0, 0.75, 100),  // F
        (65, 9.0, 0.5, 85),    // F
        (64, 10.0, 0.75, 95),  // E
        (64, 11.0, 0.5, 85),   // E
        (62, 12.0, 0.75, 100), // D
        (62, 13.0, 0.5, 85),   // D
        (60, 14.0, 1.5, 95),   // C (held)
        // Bar 5-6: "How I wonder"
        (67, 16.0, 0.75, 95), // G
        (67, 17.0, 0.5, 85),  // G
        (65, 18.0, 0.75, 90), // F
        (65, 19.0, 0.5, 80),  // F
        (64, 20.0, 0.75, 95), // E
        (64, 21.0, 0.5, 85),  // E
        (62, 22.0, 1.5, 90),  // D (held)
        // Bar 7-8: "What you are"
        (67, 24.0, 0.75, 95), // G
        (67, 25.0, 0.5, 85),  // G
        (65, 26.0, 0.75, 90), // F
        (65, 27.0, 0.5, 80),  // F
        (64, 28.0, 0.75, 95), // E
        (64, 29.0, 0.5, 85),  // E
        (62, 30.0, 1.5, 90),  // D (held)
        // Bar 9-16: Repeat with variations
        (60, 32.0, 0.75, 105), // C (louder second verse)
        (60, 33.0, 0.5, 95),
        (67, 34.0, 0.75, 105),
        (67, 35.0, 0.5, 90),
        (69, 36.0, 0.75, 115), // A (big accent)
        (69, 37.0, 0.5, 95),
        (67, 38.0, 1.5, 100),
        (65, 40.0, 0.75, 105),
        (65, 41.0, 0.5, 90),
        (64, 42.0, 0.75, 100),
        (64, 43.0, 0.5, 90),
        (62, 44.0, 0.75, 105),
        (62, 45.0, 0.5, 90),
        (60, 46.0, 2.0, 100), // Final C (longer)
    ];

    // Build melody notes args
    let mut melody_args = vec![OscType::Int(melody_track), OscType::Int(0)];
    for (pitch, start, dur, vel) in &melody_notes {
        melody_args.push(OscType::Int(*pitch));
        melody_args.push(OscType::Float(*start));
        melody_args.push(OscType::Float(*dur));
        melody_args.push(OscType::Int(*vel));
        melody_args.push(OscType::Int(0)); // not muted
    }

    client
        .send("/live/clip/add/notes", melody_args)
        .await
        .expect("Failed to add melody notes");

    // Name the clip
    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(melody_track),
                OscType::Int(0),
                OscType::String("ATL Twinkle Melody".into()),
            ],
        )
        .await
        .expect("Failed to name melody clip");

    // ========== TRACK 2: 808 BASS ==========
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create bass track");
    sleep(Duration::from_millis(300)).await;

    let bass_track = initial_tracks + 1;

    client
        .send(
            "/live/track/set/name",
            vec![OscType::Int(bass_track), OscType::String("808 Bass".into())],
        )
        .await
        .expect("Failed to name bass track");

    // Select and load instrument for bass
    client
        .send(
            "/live/view/set/selected_track",
            vec![OscType::Int(bass_track)],
        )
        .await
        .expect("Failed to select bass track");
    sleep(Duration::from_millis(200)).await;

    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .expect("Failed to load bass instrument");
    sleep(Duration::from_millis(800)).await;

    // Create bass clip
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![
                OscType::Int(bass_track),
                OscType::Int(0),
                OscType::Float(64.0),
            ],
        )
        .await
        .expect("Failed to create bass clip");
    sleep(Duration::from_millis(200)).await;

    // 808 bass following root notes (C2=36, D2=38, E2=40, F2=41, G2=43, A2=45)
    // Deep sub bass with long sustain on downbeats
    let bass_notes: Vec<(i32, f32, f32, i32)> = vec![
        // Following melody root notes
        (36, 0.0, 3.5, 120),  // C (bars 1-2)
        (43, 4.0, 3.5, 115),  // G (bars 2)
        (41, 8.0, 3.5, 120),  // F (bars 3-4)
        (36, 14.0, 1.5, 110), // C
        (43, 16.0, 3.5, 115), // G (bars 5-6)
        (38, 22.0, 1.5, 110), // D
        (43, 24.0, 3.5, 115), // G (bars 7-8)
        (38, 30.0, 1.5, 110), // D
        // Second half
        (36, 32.0, 3.5, 125), // C (louder)
        (43, 36.0, 3.5, 120), // G
        (41, 40.0, 3.5, 125), // F
        (36, 46.0, 2.0, 120), // Final C
    ];

    let mut bass_args = vec![OscType::Int(bass_track), OscType::Int(0)];
    for (pitch, start, dur, vel) in &bass_notes {
        bass_args.push(OscType::Int(*pitch));
        bass_args.push(OscType::Float(*start));
        bass_args.push(OscType::Float(*dur));
        bass_args.push(OscType::Int(*vel));
        bass_args.push(OscType::Int(0));
    }

    client
        .send("/live/clip/add/notes", bass_args)
        .await
        .expect("Failed to add bass notes");

    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(bass_track),
                OscType::Int(0),
                OscType::String("808 Sub".into()),
            ],
        )
        .await
        .expect("Failed to name bass clip");

    // ========== TRACK 3: HI-HATS (Triplet rolls) ==========
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create hihat track");
    sleep(Duration::from_millis(300)).await;

    let hihat_track = initial_tracks + 2;

    client
        .send(
            "/live/track/set/name",
            vec![
                OscType::Int(hihat_track),
                OscType::String("Trap Hats".into()),
            ],
        )
        .await
        .expect("Failed to name hihat track");

    // Select and load instrument
    client
        .send(
            "/live/view/set/selected_track",
            vec![OscType::Int(hihat_track)],
        )
        .await
        .expect("Failed to select hihat track");
    sleep(Duration::from_millis(200)).await;

    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .expect("Failed to load hihat instrument");
    sleep(Duration::from_millis(800)).await;

    // Create hihat clip
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![
                OscType::Int(hihat_track),
                OscType::Int(0),
                OscType::Float(64.0),
            ],
        )
        .await
        .expect("Failed to create hihat clip");
    sleep(Duration::from_millis(200)).await;

    // Rolling triplet hi-hats - signature Atlanta trap sound
    // F#4 (66) for closed hat, using triplet subdivisions
    let mut hihat_notes: Vec<(i32, f32, f32, i32)> = Vec::new();

    // Generate rolling triplet pattern for 16 bars (64 beats)
    for bar in 0..16 {
        let bar_start = bar as f32 * 4.0;

        // Basic hi-hat on every 8th note with triplet rolls
        for beat in 0..4 {
            let beat_start = bar_start + beat as f32;

            // Main 8th notes
            hihat_notes.push((66, beat_start, 0.125, 90));
            hihat_notes.push((66, beat_start + 0.5, 0.125, 80));

            // Add triplet roll on beats 2 and 4 (signature trap pattern)
            if beat == 1 || beat == 3 {
                // Triplet subdivision (3 notes in space of 2)
                let triplet_spacing = 0.166_667; // 1/6 of a beat
                hihat_notes.push((66, beat_start + 0.25, 0.1, 70));
                hihat_notes.push((66, beat_start + 0.25 + triplet_spacing, 0.1, 60));
                hihat_notes.push((66, triplet_spacing.mul_add(2.0, beat_start + 0.25), 0.1, 65));
            }
        }
    }

    let mut hihat_args = vec![OscType::Int(hihat_track), OscType::Int(0)];
    for (pitch, start, dur, vel) in &hihat_notes {
        hihat_args.push(OscType::Int(*pitch));
        hihat_args.push(OscType::Float(*start));
        hihat_args.push(OscType::Float(*dur));
        hihat_args.push(OscType::Int(*vel));
        hihat_args.push(OscType::Int(0));
    }

    client
        .send("/live/clip/add/notes", hihat_args)
        .await
        .expect("Failed to add hihat notes");

    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(hihat_track),
                OscType::Int(0),
                OscType::String("Trap Hats".into()),
            ],
        )
        .await
        .expect("Failed to name hihat clip");

    // ========== TRACK 4: SNARE/CLAP ==========
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create snare track");
    sleep(Duration::from_millis(300)).await;

    let snare_track = initial_tracks + 3;

    client
        .send(
            "/live/track/set/name",
            vec![
                OscType::Int(snare_track),
                OscType::String("Snare/Clap".into()),
            ],
        )
        .await
        .expect("Failed to name snare track");

    client
        .send(
            "/live/view/set/selected_track",
            vec![OscType::Int(snare_track)],
        )
        .await
        .expect("Failed to select snare track");
    sleep(Duration::from_millis(200)).await;

    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .expect("Failed to load snare instrument");
    sleep(Duration::from_millis(800)).await;

    // Create snare clip
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![
                OscType::Int(snare_track),
                OscType::Int(0),
                OscType::Float(64.0),
            ],
        )
        .await
        .expect("Failed to create snare clip");
    sleep(Duration::from_millis(200)).await;

    // Snare on beats 2 and 4 - classic trap pattern
    // D4 (62) for snare sound
    let mut snare_notes: Vec<(i32, f32, f32, i32)> = Vec::new();

    for bar in 0..16 {
        let bar_start = bar as f32 * 4.0;
        // Beat 2
        snare_notes.push((62, bar_start + 1.0, 0.25, 110));
        // Beat 4
        snare_notes.push((62, bar_start + 3.0, 0.25, 115));
    }

    let mut snare_args = vec![OscType::Int(snare_track), OscType::Int(0)];
    for (pitch, start, dur, vel) in &snare_notes {
        snare_args.push(OscType::Int(*pitch));
        snare_args.push(OscType::Float(*start));
        snare_args.push(OscType::Float(*dur));
        snare_args.push(OscType::Int(*vel));
        snare_args.push(OscType::Int(0));
    }

    client
        .send("/live/clip/add/notes", snare_args)
        .await
        .expect("Failed to add snare notes");

    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(snare_track),
                OscType::Int(0),
                OscType::String("Trap Snare".into()),
            ],
        )
        .await
        .expect("Failed to name snare clip");

    // ========== FIRE ALL CLIPS AND PLAY ==========
    sleep(Duration::from_millis(300)).await;

    // Fire all clips at scene 0
    client
        .send("/live/scene/fire", vec![OscType::Int(0)])
        .await
        .expect("Failed to fire scene");

    // Start playback
    client
        .send("/live/song/start_playing", vec![])
        .await
        .expect("Failed to start playback");

    println!("Atlanta Hip Hop Twinkle Twinkle Little Star created and playing!");
    println!("  - Tempo: 140 BPM");
    println!("  - Track {melody_track}: ATL Twinkle (Melody)");
    println!("  - Track {bass_track}: 808 Bass");
    println!("  - Track {hihat_track}: Trap Hats (rolling triplets)");
    println!("  - Track {snare_track}: Snare/Clap (beats 2 & 4)");
}

// ============================================================================
// Browser Tests
// ============================================================================

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_audio_effects() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_audio_effects", vec![])
        .await
        .expect("Failed to list audio effects");

    let effects = extract_strings_from_packets(packets);

    println!("Found {} audio effect categories:", effects.len());
    for effect in &effects {
        println!("  - {effect}");
    }

    assert!(
        !effects.is_empty(),
        "Should find at least some audio effects"
    );
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_midi_effects() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_midi_effects", vec![])
        .await
        .expect("Failed to list MIDI effects");

    let effects = extract_strings_from_packets(packets);

    println!("Found {} MIDI effect categories:", effects.len());
    for effect in &effects {
        println!("  - {effect}");
    }

    assert!(
        !effects.is_empty(),
        "Should find at least some MIDI effects"
    );
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_sounds() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_sounds", vec![])
        .await
        .expect("Failed to list sounds");

    let sounds = extract_strings_from_packets(packets);

    println!("Found {} sound categories:", sounds.len());
    for sound in &sounds {
        println!("  - {sound}");
    }

    // Sounds may be empty if no packs installed, so just verify the call worked
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_samples() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_samples", vec![])
        .await
        .expect("Failed to list samples");

    let samples = extract_strings_from_packets(packets);

    println!("Found {} sample categories:", samples.len());
    for sample in &samples {
        println!("  - {sample}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_clips() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_clips", vec![])
        .await
        .expect("Failed to list clips");

    let clips = extract_strings_from_packets(packets);

    println!("Found {} clip categories:", clips.len());
    for clip in &clips {
        println!("  - {clip}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_plugins() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_plugins", vec![])
        .await
        .expect("Failed to list plugins");

    let plugins = extract_strings_from_packets(packets);

    println!("Found {} plugin categories:", plugins.len());
    for plugin in &plugins {
        println!("  - {plugin}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_max_devices() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_max_devices", vec![])
        .await
        .expect("Failed to list Max devices");

    let devices = extract_strings_from_packets(packets);

    println!("Found {} Max for Live device categories:", devices.len());
    for device in &devices {
        println!("  - {device}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_list_user_presets() {
    let client = create_test_client().await;
    let packets = client
        .query_all("/live/browser/list_user_presets", vec![])
        .await
        .expect("Failed to list user presets");

    let presets = extract_strings_from_packets(packets);

    println!("Found {} user library items:", presets.len());
    for preset in &presets {
        println!("  - {preset}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_browse_instruments() {
    let client = create_test_client().await;
    let packets = client
        .query_all(
            "/live/browser/browse",
            vec![OscType::String("instruments".into())],
        )
        .await
        .expect("Failed to browse instruments");

    let items = extract_strings_from_packets(packets);

    println!("Found {} items in instruments:", items.len());
    for item in &items {
        println!("  - {item}");
    }

    assert!(!items.is_empty(), "Should find at least some instruments");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_browse_audio_effects() {
    let client = create_test_client().await;
    let packets = client
        .query_all(
            "/live/browser/browse",
            vec![OscType::String("audio_effects".into())],
        )
        .await
        .expect("Failed to browse audio effects");

    let items = extract_strings_from_packets(packets);

    println!("Found {} items in audio_effects:", items.len());
    for item in &items {
        println!("  - {item}");
    }

    assert!(!items.is_empty(), "Should find at least some audio effects");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_search_browser() {
    let client = create_test_client().await;
    let packets = client
        .query_all(
            "/live/browser/search",
            vec![OscType::String("reverb".into())],
        )
        .await
        .expect("Failed to search browser");

    let results = extract_strings_from_packets(packets);

    println!("Found {} search results for 'reverb':", results.len() / 2);
    // Results come as category, name pairs
    for chunk in results.chunks(2) {
        if let [cat, name] = chunk {
            println!("  - [{cat}] {name}");
        }
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_load_default_audio_effect() {
    let client = create_test_client().await;

    // First create a MIDI track to load the effect onto
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create track");

    sleep(Duration::from_millis(300)).await;

    // Load default audio effect (Reverb)
    client
        .send("/live/browser/load_default_audio_effect", vec![])
        .await
        .expect("Failed to load default audio effect");

    sleep(Duration::from_millis(500)).await;

    println!("Loaded default audio effect (Reverb) onto new track");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_load_default_midi_effect() {
    let client = create_test_client().await;

    // First create a MIDI track to load the effect onto
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create track");

    sleep(Duration::from_millis(300)).await;

    // Load default MIDI effect (Arpeggiator)
    client
        .send("/live/browser/load_default_midi_effect", vec![])
        .await
        .expect("Failed to load default MIDI effect");

    sleep(Duration::from_millis(500)).await;

    println!("Loaded default MIDI effect (Arpeggiator) onto new track");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_load_audio_effect_by_name() {
    let client = create_test_client().await;

    // First create a MIDI track
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create track");

    sleep(Duration::from_millis(300)).await;

    // Load Delay effect
    client
        .send(
            "/live/browser/load_audio_effect",
            vec![OscType::String("Delay".into())],
        )
        .await
        .expect("Failed to load Delay");

    sleep(Duration::from_millis(500)).await;

    println!("Loaded Delay audio effect onto new track");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_load_midi_effect_by_name() {
    let client = create_test_client().await;

    // First create a MIDI track
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create track");

    sleep(Duration::from_millis(300)).await;

    // Load Chord effect
    client
        .send(
            "/live/browser/load_midi_effect",
            vec![OscType::String("Chord".into())],
        )
        .await
        .expect("Failed to load Chord");

    sleep(Duration::from_millis(500)).await;

    println!("Loaded Chord MIDI effect onto new track");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_preview_and_stop() {
    let client = create_test_client().await;

    // Try to preview a sample (may not find one, but tests the endpoint)
    let _ = client
        .send(
            "/live/browser/preview_sample",
            vec![OscType::String("kick".into())],
        )
        .await;

    sleep(Duration::from_millis(200)).await;

    // Stop preview
    client
        .send("/live/browser/stop_preview", vec![])
        .await
        .expect("Failed to stop preview");

    println!("Preview start/stop test completed");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_get_item_info() {
    let client = create_test_client().await;

    let packets = client
        .query_all(
            "/live/browser/get_item_info",
            vec![
                OscType::String("instruments".into()),
                OscType::String("Drift".into()),
            ],
        )
        .await
        .expect("Failed to get item info");

    let mut info = Vec::new();
    for packet in packets {
        if let rosc::OscPacket::Message(msg) = packet {
            for arg in msg.args {
                match arg {
                    OscType::String(s) => info.push(format!("String: {s}")),
                    OscType::Int(i) => info.push(format!("Int: {i}")),
                    OscType::Bool(b) => info.push(format!("Bool: {b}")),
                    _ => info.push("Other".to_string()),
                }
            }
        }
    }

    println!("Item info for instruments/Drift:");
    for i in &info {
        println!("  {i}");
    }
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_full_browser_workflow() {
    let client = create_test_client().await;

    println!("=== Full Browser Workflow Test ===\n");

    // 1. Create a MIDI track
    println!("1. Creating MIDI track...");
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .expect("Failed to create track");
    sleep(Duration::from_millis(300)).await;

    // 2. Load an instrument
    println!("2. Loading Drift instrument...");
    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .expect("Failed to load instrument");
    sleep(Duration::from_millis(500)).await;

    // 3. Add MIDI effect
    println!("3. Adding Arpeggiator MIDI effect...");
    client
        .send("/live/browser/load_default_midi_effect", vec![])
        .await
        .expect("Failed to load MIDI effect");
    sleep(Duration::from_millis(500)).await;

    // 4. Add audio effect
    println!("4. Adding Reverb audio effect...");
    client
        .send("/live/browser/load_default_audio_effect", vec![])
        .await
        .expect("Failed to load audio effect");
    sleep(Duration::from_millis(500)).await;

    // 5. Add another audio effect
    println!("5. Adding Delay audio effect...");
    client
        .send(
            "/live/browser/load_audio_effect",
            vec![OscType::String("Delay".into())],
        )
        .await
        .expect("Failed to load Delay");
    sleep(Duration::from_millis(500)).await;

    println!("\n=== Workflow Complete ===");
    println!("Created track with: Drift -> Arpeggiator -> Reverb -> Delay");
}

#[tokio::test]
#[ignore = "Requires Ableton Live with AbletonOSC to be running"]
async fn test_create_trap_eenie_meenie() {
    let client = create_test_client().await;

    // Stop playback
    client
        .send("/live/song/stop_playing", vec![])
        .await
        .unwrap();
    sleep(Duration::from_millis(200)).await;

    // Get number of tracks and delete them all
    let num_tracks: i32 = client
        .query("/live/song/get/num_tracks", vec![])
        .await
        .unwrap();
    println!("Cleaning up {num_tracks} tracks...");

    for i in (0..num_tracks).rev() {
        client
            .send("/live/song/delete_track", vec![OscType::Int(i)])
            .await
            .unwrap();
        sleep(Duration::from_millis(50)).await;
    }

    println!("Creating Trap Eenie Meenie Miney Mo...");

    // Set tempo
    client
        .send("/live/song/set/tempo", vec![OscType::Float(140.0)])
        .await
        .unwrap();
    sleep(Duration::from_millis(100)).await;

    // === TRACK 1: MELODY ===
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    client
        .send("/live/browser/load_default_instrument", vec![])
        .await
        .unwrap();
    sleep(Duration::from_millis(500)).await;
    client
        .send(
            "/live/track/set/name",
            vec![OscType::Int(0), OscType::String("Melody".into())],
        )
        .await
        .unwrap();
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![OscType::Int(0), OscType::Int(0), OscType::Float(16.0)],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(200)).await;

    // Melody: Eenie Meenie Miney Mo (C pentatonic)
    let melody: Vec<(i32, f32, f32, i32)> = vec![
        // "Ee-nie mee-nie mi-ney mo"
        (72, 0.0, 0.25, 100),
        (74, 0.5, 0.25, 90),
        (76, 1.0, 0.25, 100),
        (74, 1.5, 0.25, 90),
        (72, 2.0, 0.25, 100),
        (74, 2.5, 0.25, 90),
        (76, 3.0, 0.5, 110),
        // "Catch a tiger by the toe"
        (79, 4.0, 0.25, 100),
        (76, 4.5, 0.25, 90),
        (76, 5.0, 0.25, 100),
        (74, 5.5, 0.25, 90),
        (72, 6.0, 0.25, 100),
        (74, 6.5, 0.25, 90),
        (76, 7.0, 0.5, 110),
        // "If he hollers let him go"
        (72, 8.0, 0.25, 100),
        (74, 8.5, 0.25, 90),
        (76, 9.0, 0.25, 100),
        (79, 9.5, 0.25, 90),
        (76, 10.0, 0.25, 100),
        (74, 10.5, 0.25, 90),
        (72, 11.0, 0.5, 110),
        // "Eenie meenie miney mo"
        (72, 12.0, 0.25, 100),
        (74, 12.5, 0.25, 90),
        (76, 13.0, 0.25, 100),
        (74, 13.5, 0.25, 90),
        (72, 14.0, 0.25, 100),
        (74, 14.5, 0.25, 90),
        (72, 15.0, 1.0, 120),
    ];

    let mut args = vec![OscType::Int(0), OscType::Int(0)];
    for (p, s, d, v) in &melody {
        args.push(OscType::Int(*p));
        args.push(OscType::Float(*s));
        args.push(OscType::Float(*d));
        args.push(OscType::Int(*v));
        args.push(OscType::Int(0));
    }
    client.send("/live/clip/add/notes", args).await.unwrap();
    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(0),
                OscType::Int(0),
                OscType::String("Eenie Meenie".into()),
            ],
        )
        .await
        .unwrap();

    // === TRACK 2: 808 BASS ===
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    client
        .send(
            "/live/browser/load_instrument",
            vec![OscType::String("Drift".into())],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(500)).await;
    client
        .send(
            "/live/track/set/name",
            vec![OscType::Int(1), OscType::String("808 Bass".into())],
        )
        .await
        .unwrap();
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![OscType::Int(1), OscType::Int(0), OscType::Float(16.0)],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(200)).await;

    let bass: Vec<(i32, f32, f32, i32)> = vec![
        (36, 0.0, 0.75, 127),
        (36, 3.0, 0.5, 110),
        (36, 4.0, 0.75, 127),
        (36, 7.0, 0.5, 110),
        (36, 8.0, 0.75, 127),
        (36, 11.0, 0.5, 110),
        (36, 12.0, 0.75, 127),
        (38, 14.0, 0.5, 100),
        (36, 15.0, 0.5, 110),
    ];

    let mut args = vec![OscType::Int(1), OscType::Int(0)];
    for (p, s, d, v) in &bass {
        args.push(OscType::Int(*p));
        args.push(OscType::Float(*s));
        args.push(OscType::Float(*d));
        args.push(OscType::Int(*v));
        args.push(OscType::Int(0));
    }
    client.send("/live/clip/add/notes", args).await.unwrap();
    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(1),
                OscType::Int(0),
                OscType::String("808".into()),
            ],
        )
        .await
        .unwrap();

    // === TRACK 3: HI-HATS ===
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    client
        .send("/live/browser/load_drum_kit", vec![])
        .await
        .unwrap();
    sleep(Duration::from_millis(500)).await;
    client
        .send(
            "/live/track/set/name",
            vec![OscType::Int(2), OscType::String("Trap Hats".into())],
        )
        .await
        .unwrap();
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![OscType::Int(2), OscType::Int(0), OscType::Float(16.0)],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(200)).await;

    let mut hats: Vec<(i32, f32, f32, i32)> = Vec::new();
    for bar in 0..4 {
        let o = bar as f32 * 4.0;
        // Regular 8th notes
        for i in 0..8 {
            hats.push((
                42,
                (i as f32).mul_add(0.5, o),
                0.1,
                if i % 2 == 0 { 100 } else { 80 },
            ));
        }
        // Triplet rolls on odd bars
        if bar % 2 == 1 {
            for i in 0..6 {
                hats.push((42, (i as f32).mul_add(0.166, o + 3.0), 0.08, 90 + (i * 5)));
            }
        }
    }

    let mut args = vec![OscType::Int(2), OscType::Int(0)];
    for (p, s, d, v) in &hats {
        args.push(OscType::Int(*p));
        args.push(OscType::Float(*s));
        args.push(OscType::Float(*d));
        args.push(OscType::Int(*v));
        args.push(OscType::Int(0));
    }
    client.send("/live/clip/add/notes", args).await.unwrap();
    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(2),
                OscType::Int(0),
                OscType::String("Hats".into()),
            ],
        )
        .await
        .unwrap();

    // === TRACK 4: SNARE ===
    client
        .send("/live/song/create_midi_track", vec![OscType::Int(-1)])
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    client
        .send("/live/browser/load_drum_kit", vec![])
        .await
        .unwrap();
    sleep(Duration::from_millis(500)).await;
    client
        .send(
            "/live/track/set/name",
            vec![OscType::Int(3), OscType::String("Snare".into())],
        )
        .await
        .unwrap();
    client
        .send(
            "/live/clip_slot/create_clip",
            vec![OscType::Int(3), OscType::Int(0), OscType::Float(16.0)],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(200)).await;

    let mut snare: Vec<(i32, f32, f32, i32)> = Vec::new();
    for bar in 0..4 {
        let o = bar as f32 * 4.0;
        snare.push((38, o + 1.0, 0.25, 110)); // Beat 2
        snare.push((38, o + 3.0, 0.25, 110)); // Beat 4
    }

    let mut args = vec![OscType::Int(3), OscType::Int(0)];
    for (p, s, d, v) in &snare {
        args.push(OscType::Int(*p));
        args.push(OscType::Float(*s));
        args.push(OscType::Float(*d));
        args.push(OscType::Int(*v));
        args.push(OscType::Int(0));
    }
    client.send("/live/clip/add/notes", args).await.unwrap();
    client
        .send(
            "/live/clip/set/name",
            vec![
                OscType::Int(3),
                OscType::Int(0),
                OscType::String("Snare".into()),
            ],
        )
        .await
        .unwrap();

    // === ADD EFFECTS ===
    client
        .send("/live/view/set/selected_track", vec![OscType::Int(0)])
        .await
        .unwrap();
    sleep(Duration::from_millis(100)).await;
    client
        .send(
            "/live/browser/load_audio_effect",
            vec![OscType::String("Reverb".into())],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;
    client
        .send(
            "/live/browser/load_audio_effect",
            vec![OscType::String("Delay".into())],
        )
        .await
        .unwrap();
    sleep(Duration::from_millis(300)).await;

    // === PLAY ===
    sleep(Duration::from_millis(200)).await;
    client
        .send("/live/scene/fire", vec![OscType::Int(0)])
        .await
        .unwrap();
    client
        .send("/live/song/start_playing", vec![])
        .await
        .unwrap();

    println!("\nTrap Eenie Meenie Miney Mo created and playing!");
    println!("  Tempo: 140 BPM");
    println!("  Track 0: Melody + Reverb + Delay");
    println!("  Track 1: 808 Bass");
    println!("  Track 2: Trap Hi-Hats with rolls");
    println!("  Track 3: Snare on 2 & 4");
}
