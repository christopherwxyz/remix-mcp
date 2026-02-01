//! Ableton MCP Server - Control Ableton Live via OSC over MCP.
//!
//! This library provides an MCP (Model Context Protocol) server that allows
//! AI assistants to control Ableton Live through the `AbletonOSC` Remote Script.

pub mod error;
pub mod installer;
pub mod osc;
pub mod server;
pub mod tools;
pub mod types;

pub use error::Error;
pub use installer::InstallStatus;
pub use server::AbletonServer;
