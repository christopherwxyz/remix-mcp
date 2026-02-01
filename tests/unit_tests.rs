//! Unit tests with snapshot testing (insta) and parameterized tests (rstest).

use insta::{assert_json_snapshot, assert_snapshot};
use rosc::{OscMessage, OscType};
use rstest::rstest;

// ============================================================================
// OSC Message Builder Tests (rstest parameterized)
// ============================================================================

/// Test OSC message argument construction with various types.
#[rstest]
#[case::integer(vec![OscType::Int(42)], "[Int(42)]")]
#[case::float(vec![OscType::Float(std::f32::consts::PI)], "[Float(3.1415927)]")]
#[case::string(vec![OscType::String("hello".into())], "[String(\"hello\")]")]
#[case::bool_true(vec![OscType::Bool(true)], "[Bool(true)]")]
#[case::bool_false(vec![OscType::Bool(false)], "[Bool(false)]")]
#[case::mixed(
    vec![OscType::Int(1), OscType::Float(2.0), OscType::String("three".into())],
    "[Int(1), Float(2.0), String(\"three\")]"
)]
fn test_osc_type_formatting(#[case] args: Vec<OscType>, #[case] expected: &str) {
    assert_eq!(format!("{args:?}"), expected);
}

/// Test track indices are valid.
#[rstest]
#[case::track_0(0)]
#[case::track_1(1)]
#[case::track_127(127)]
fn test_valid_track_indices(#[case] track: i32) {
    assert!(track >= 0);
    assert!(track < 256); // Reasonable upper bound
}

/// Test scene indices are valid.
#[rstest]
#[case::scene_0(0)]
#[case::scene_1(1)]
#[case::scene_99(99)]
fn test_valid_scene_indices(#[case] scene: i32) {
    assert!(scene >= 0);
}

/// Test tempo values in valid range.
#[rstest]
#[case::slow(20.0, true)]
#[case::normal(120.0, true)]
#[case::fast(200.0, true)]
#[case::max(999.0, true)]
#[case::too_slow(19.9, false)]
#[case::too_fast(1000.0, false)]
fn test_tempo_validation(#[case] tempo: f32, #[case] valid: bool) {
    let is_valid = (20.0..=999.0).contains(&tempo);
    assert_eq!(is_valid, valid);
}

/// Test MIDI note values.
#[rstest]
#[case::c_minus_2(0, true)]
#[case::middle_c(60, true)]
#[case::c8(108, true)]
#[case::g9(127, true)]
#[case::invalid_high(128, false)]
#[case::invalid_negative(-1, false)]
fn test_midi_note_validation(#[case] note: i32, #[case] valid: bool) {
    let is_valid = (0..=127).contains(&note);
    assert_eq!(is_valid, valid);
}

/// Test velocity values.
#[rstest]
#[case::silent(0, true)]
#[case::soft(32, true)]
#[case::medium(64, true)]
#[case::loud(100, true)]
#[case::max(127, true)]
#[case::invalid(128, false)]
fn test_velocity_validation(#[case] velocity: i32, #[case] valid: bool) {
    let is_valid = (0..=127).contains(&velocity);
    assert_eq!(is_valid, valid);
}

// ============================================================================
// Snapshot Tests for OSC Message Formats
// ============================================================================

/// Snapshot test for tempo query message format.
#[test]
fn test_tempo_query_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/song/get/tempo".to_string(),
        args: vec![],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for set tempo message format.
#[test]
fn test_set_tempo_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/song/set/tempo".to_string(),
        args: vec![OscType::Float(128.0)],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for track volume message format.
#[test]
fn test_track_volume_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/track/set/volume".to_string(),
        args: vec![OscType::Int(0), OscType::Float(0.85)],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for fire clip message format.
#[test]
fn test_fire_clip_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/clip_slot/fire".to_string(),
        args: vec![OscType::Int(0), OscType::Int(0)],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for add MIDI notes message format.
#[test]
fn test_add_notes_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/clip/add/notes".to_string(),
        args: vec![
            OscType::Int(0),     // track
            OscType::Int(0),     // scene
            OscType::Int(60),    // pitch (C4)
            OscType::Float(0.0), // start time
            OscType::Float(1.0), // duration
            OscType::Int(100),   // velocity
            OscType::Int(0),     // mute
        ],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for browser load instrument message.
#[test]
fn test_load_instrument_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/browser/load_instrument".to_string(),
        args: vec![OscType::String("Drift".to_string())],
    };
    assert_snapshot!(format!("{:?}", msg));
}

/// Snapshot test for browser search message.
#[test]
fn test_browser_search_message_snapshot() {
    let msg = OscMessage {
        addr: "/live/browser/search".to_string(),
        args: vec![OscType::String("bass".to_string())],
    };
    assert_snapshot!(format!("{:?}", msg));
}

// ============================================================================
// JSON Snapshot Tests for Complex Structures
// ============================================================================

/// Snapshot test for a typical OSC response structure.
#[test]
fn test_osc_response_structure_snapshot() {
    let response = serde_json::json!({
        "address": "/live/song/get/tempo",
        "args": [120.0]
    });
    assert_json_snapshot!(response);
}

/// Snapshot test for track info response structure.
#[test]
fn test_track_info_response_snapshot() {
    let response = serde_json::json!({
        "track_index": 0,
        "name": "Lead Synth",
        "volume": 0.85,
        "panning": 0.0,
        "mute": false,
        "solo": false,
        "arm": true,
        "color": 16_750_848
    });
    assert_json_snapshot!(response);
}

/// Snapshot test for clip info response structure.
#[test]
fn test_clip_info_response_snapshot() {
    let response = serde_json::json!({
        "track": 0,
        "scene": 0,
        "name": "Melody A",
        "length": 16.0,
        "loop_start": 0.0,
        "loop_end": 16.0,
        "is_midi": true,
        "is_playing": false
    });
    assert_json_snapshot!(response);
}

/// Snapshot test for MIDI note data structure.
#[test]
fn test_midi_note_snapshot() {
    let notes = serde_json::json!([
        {"pitch": 60, "start": 0.0, "duration": 0.5, "velocity": 100, "mute": false},
        {"pitch": 64, "start": 0.5, "duration": 0.5, "velocity": 90, "mute": false},
        {"pitch": 67, "start": 1.0, "duration": 0.5, "velocity": 95, "mute": false},
    ]);
    assert_json_snapshot!(notes);
}

/// Snapshot test for browser search results structure.
#[test]
fn test_browser_search_results_snapshot() {
    let results = serde_json::json!({
        "query": "reverb",
        "results": [
            {"category": "Audio Effects", "name": "Reverb"},
            {"category": "Audio Effects", "name": "Convolution Reverb"},
            {"category": "Max for Live", "name": "Reverb Tail"},
        ]
    });
    assert_json_snapshot!(results);
}

// ============================================================================
// OSC Address Path Tests
// ============================================================================

#[rstest]
#[case::get_tempo("/live/song/get/tempo")]
#[case::set_tempo("/live/song/set/tempo")]
#[case::start_playing("/live/song/start_playing")]
#[case::stop_playing("/live/song/stop_playing")]
#[case::get_track_volume("/live/track/get/volume")]
#[case::set_track_volume("/live/track/set/volume")]
#[case::fire_clip("/live/clip_slot/fire")]
#[case::create_clip("/live/clip_slot/create_clip")]
#[case::add_notes("/live/clip/add/notes")]
#[case::load_instrument("/live/browser/load_instrument")]
fn test_osc_address_format(#[case] address: &str) {
    assert!(address.starts_with("/live/"));
    assert!(!address.contains(' '));
    assert!(!address.ends_with('/'));
}
