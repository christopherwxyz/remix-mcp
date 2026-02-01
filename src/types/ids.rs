//! Newtype wrappers for various IDs in Ableton Live.

use serde::{Deserialize, Serialize};

/// Track index (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackId(pub u32);

impl From<TrackId> for i32 {
    fn from(id: TrackId) -> Self {
        id.0 as i32
    }
}

impl From<u32> for TrackId {
    fn from(v: u32) -> Self {
        TrackId(v)
    }
}

/// Clip slot index (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipSlotId(pub u32);

impl From<ClipSlotId> for i32 {
    fn from(id: ClipSlotId) -> Self {
        id.0 as i32
    }
}

impl From<u32> for ClipSlotId {
    fn from(v: u32) -> Self {
        ClipSlotId(v)
    }
}

/// Scene index (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneId(pub u32);

impl From<SceneId> for i32 {
    fn from(id: SceneId) -> Self {
        id.0 as i32
    }
}

impl From<u32> for SceneId {
    fn from(v: u32) -> Self {
        SceneId(v)
    }
}

/// Device index (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceId(pub u32);

impl From<DeviceId> for i32 {
    fn from(id: DeviceId) -> Self {
        id.0 as i32
    }
}

impl From<u32> for DeviceId {
    fn from(v: u32) -> Self {
        DeviceId(v)
    }
}

/// Parameter index (0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterId(pub u32);

impl From<ParameterId> for i32 {
    fn from(id: ParameterId) -> Self {
        id.0 as i32
    }
}

impl From<u32> for ParameterId {
    fn from(v: u32) -> Self {
        ParameterId(v)
    }
}
