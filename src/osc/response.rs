//! OSC response parsing utilities.

use rosc::{OscPacket, OscType};

use crate::error::Error;

/// Trait for types that can be parsed from an OSC response.
pub trait FromOsc: Sized {
    fn from_osc(packet: OscPacket) -> Result<Self, Error>;
}

/// Implementation for raw OSC argument list.
impl FromOsc for Vec<OscType> {
    fn from_osc(packet: OscPacket) -> Result<Self, Error> {
        match packet {
            OscPacket::Message(msg) => Ok(msg.args),
            OscPacket::Bundle(bundle) => {
                // Flatten bundle into a single list of args from first message
                for content in bundle.content {
                    if let OscPacket::Message(msg) = content {
                        return Ok(msg.args);
                    }
                }
                Err(Error::InvalidResponse("Empty bundle".into()))
            }
        }
    }
}

/// Implementation for single float value.
/// Handles responses like `[Int(0), Float(0.5)]` by taking the last float.
impl FromOsc for f32 {
    fn from_osc(packet: OscPacket) -> Result<Self, Error> {
        let args = Vec::<OscType>::from_osc(packet)?;
        // Find the last float in the response (skips index arguments)
        for arg in args.iter().rev() {
            match arg {
                OscType::Float(v) => return Ok(*v),
                OscType::Int(_) if args.len() == 1 => {
                    // Single int can be converted to float
                    if let OscType::Int(v) = arg {
                        return Ok(*v as f32);
                    }
                }
                _ => {}
            }
        }
        // Fallback to first argument
        match args.first() {
            Some(OscType::Float(v)) => Ok(*v),
            Some(OscType::Int(v)) => Ok(*v as f32),
            Some(other) => Err(Error::InvalidResponse(format!(
                "Expected float, got {other:?}"
            ))),
            None => Err(Error::InvalidResponse("No arguments in response".into())),
        }
    }
}

/// Implementation for single integer value.
/// Handles responses like `[Int(0), Int(1)]` by taking the last int.
impl FromOsc for i32 {
    fn from_osc(packet: OscPacket) -> Result<Self, Error> {
        let args = Vec::<OscType>::from_osc(packet)?;
        // For single argument, just return it
        if args.len() == 1 {
            match args.first() {
                Some(OscType::Int(v)) => return Ok(*v),
                Some(OscType::Float(v)) => return Ok(*v as i32),
                Some(other) => {
                    return Err(Error::InvalidResponse(format!(
                        "Expected int, got {other:?}"
                    )));
                }
                None => return Err(Error::InvalidResponse("No arguments in response".into())),
            }
        }
        // For multiple arguments, take the last int
        for arg in args.iter().rev() {
            if let OscType::Int(v) = arg {
                return Ok(*v);
            }
        }
        // Fallback to first argument
        match args.first() {
            Some(OscType::Int(v)) => Ok(*v),
            Some(OscType::Float(v)) => Ok(*v as i32),
            Some(other) => Err(Error::InvalidResponse(format!(
                "Expected int, got {other:?}"
            ))),
            None => Err(Error::InvalidResponse("No arguments in response".into())),
        }
    }
}

/// Implementation for single string value.
/// Handles responses like `[Int(0), String("name")]` by finding the last string.
impl FromOsc for String {
    fn from_osc(packet: OscPacket) -> Result<Self, Error> {
        let args = Vec::<OscType>::from_osc(packet)?;
        // Find the last string in the response (skips index arguments)
        for arg in args.iter().rev() {
            if let OscType::String(v) = arg {
                return Ok(v.clone());
            }
        }
        match args.first() {
            Some(other) => Err(Error::InvalidResponse(format!(
                "Expected string, got {other:?}"
            ))),
            None => Err(Error::InvalidResponse("No arguments in response".into())),
        }
    }
}

/// Implementation for boolean value (handles both Bool and Int types).
impl FromOsc for bool {
    fn from_osc(packet: OscPacket) -> Result<Self, Error> {
        let args = Vec::<OscType>::from_osc(packet)?;
        match args.first() {
            Some(OscType::Bool(v)) => Ok(*v),
            Some(OscType::Int(v)) => Ok(*v != 0),
            Some(other) => Err(Error::InvalidResponse(format!(
                "Expected bool, got {other:?}"
            ))),
            None => Err(Error::InvalidResponse("No arguments in response".into())),
        }
    }
}

/// Helper to extract a specific type from args at an index.
#[allow(dead_code)]
pub fn get_float(args: &[OscType], index: usize) -> Option<f32> {
    match args.get(index) {
        Some(OscType::Float(v)) => Some(*v),
        Some(OscType::Int(v)) => Some(*v as f32),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn get_int(args: &[OscType], index: usize) -> Option<i32> {
    match args.get(index) {
        Some(OscType::Int(v)) => Some(*v),
        Some(OscType::Float(v)) => Some(*v as i32),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn get_string(args: &[OscType], index: usize) -> Option<String> {
    match args.get(index) {
        Some(OscType::String(v)) => Some(v.clone()),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn get_bool(args: &[OscType], index: usize) -> Option<bool> {
    get_int(args, index).map(|v| v != 0)
}
