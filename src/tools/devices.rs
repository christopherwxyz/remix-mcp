//! Device and parameter control tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    DeviceInfo, DeviceParams, GetParameterValueStringParams, ParameterInfo, ParameterStructure,
    SetAllDeviceParametersParams, SetDeviceEnabledParams, SetDeviceParameterParams, TrackParams,
};

#[tool_router(router = devices_router, vis = "pub")]
impl AbletonServer {
    /// List all devices on a track.
    #[tool(description = "List all devices on a track")]
    pub async fn list_devices(
        &self,
        Parameters(params): Parameters<TrackParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let count: i32 = self
            .osc
            .query(
                "/live/track/get/num_devices",
                vec![OscType::Int(track as i32)],
            )
            .await?;

        let mut devices = Vec::new();
        for i in 0..count {
            let args = vec![OscType::Int(track as i32), OscType::Int(i)];

            let name: String = self
                .osc
                .query("/live/device/get/name", args.clone())
                .await
                .unwrap_or_else(|_| format!("Device {}", i + 1));

            let class_name: String = self
                .osc
                .query("/live/device/get/class_name", args.clone())
                .await
                .unwrap_or_else(|_| "Unknown".to_string());

            devices.push(DeviceInfo {
                index: i as u32,
                name,
                class_name,
            });
        }

        Ok(serde_json::to_string_pretty(&devices).unwrap_or_else(|_| "[]".into()))
    }

    /// Get all parameters for a device.
    #[tool(description = "Get all parameters for a device")]
    pub async fn get_device_parameters(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let count: i32 = self
            .osc
            .query(
                "/live/device/get/num_parameters",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;

        let mut parameters = Vec::new();
        for i in 0..count {
            let args = vec![
                OscType::Int(track as i32),
                OscType::Int(device as i32),
                OscType::Int(i),
            ];

            let name: String = self
                .osc
                .query("/live/device/get/parameter/name", args.clone())
                .await
                .unwrap_or_else(|_| format!("Param {}", i + 1));

            let value: f32 = self
                .osc
                .query("/live/device/get/parameter/value", args.clone())
                .await
                .unwrap_or(0.0);

            let min: f32 = self
                .osc
                .query("/live/device/get/parameter/min", args.clone())
                .await
                .unwrap_or(0.0);

            let max: f32 = self
                .osc
                .query("/live/device/get/parameter/max", args.clone())
                .await
                .unwrap_or(1.0);

            parameters.push(ParameterInfo {
                index: i as u32,
                name,
                value,
                min,
                max,
            });
        }

        Ok(serde_json::to_string_pretty(&parameters).unwrap_or_else(|_| "[]".into()))
    }

    /// Set a device parameter value.
    #[tool(description = "Set a device parameter value")]
    pub async fn set_device_parameter(
        &self,
        Parameters(params): Parameters<SetDeviceParameterParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let param = params.param;
        let value = params.value;
        self.osc
            .send(
                "/live/device/set/parameter/value",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(device as i32),
                    OscType::Int(param as i32),
                    OscType::Float(value),
                ],
            )
            .await?;
        Ok(format!(
            "Set parameter {param} on device {device} (track {track}) to {value}"
        ))
    }

    /// Enable or disable a device.
    #[tool(description = "Enable or disable a device")]
    pub async fn set_device_enabled(
        &self,
        Parameters(params): Parameters<SetDeviceEnabledParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/device/set/is_enabled",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(device as i32),
                    OscType::Int(if enabled { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Device {device} on track {track} {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Delete a device from a track.
    #[tool(description = "Delete a device from a track")]
    pub async fn delete_device(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        self.osc
            .send(
                "/live/track/delete_device",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;
        Ok(format!("Deleted device {device} from track {track}"))
    }

    /// Get device type (0 = audio effect, 1 = instrument, 2 = midi effect).
    #[tool(description = "Get device type (0 = audio effect, 1 = instrument, 2 = midi effect)")]
    pub async fn get_device_type(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let device_type: i32 = self
            .osc
            .query(
                "/live/device/get/type",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;
        let type_name = match device_type {
            0 => "audio effect",
            1 => "instrument",
            2 => "midi effect",
            _ => "unknown",
        };
        Ok(format!(
            "Device {device} on track {track} is a {type_name} (type {device_type})"
        ))
    }

    /// Get human-readable parameter value string.
    #[tool(description = "Get human-readable parameter value string")]
    pub async fn get_parameter_value_string(
        &self,
        Parameters(params): Parameters<GetParameterValueStringParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let param = params.param;
        let value_string: String = self
            .osc
            .query(
                "/live/device/get/parameter/value_string",
                vec![
                    OscType::Int(track as i32),
                    OscType::Int(device as i32),
                    OscType::Int(param as i32),
                ],
            )
            .await?;
        Ok(format!(
            "Parameter {param} on device {device} (track {track}): {value_string}"
        ))
    }

    /// Check if device can have chains (e.g., racks).
    #[tool(description = "Check if device can have chains (e.g., racks)")]
    pub async fn can_device_have_chains(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let result: i32 = self
            .osc
            .query(
                "/live/device/get/can_have_chains",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;
        let can_have_chains = result != 0;
        Ok(format!(
            "Device {device} on track {track} {} have chains",
            if can_have_chains { "can" } else { "cannot" }
        ))
    }

    /// Set all parameter values for a device at once.
    #[tool(description = "Set all parameter values for a device at once")]
    pub async fn set_all_device_parameters(
        &self,
        Parameters(params): Parameters<SetAllDeviceParametersParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        let values = params.values;

        let mut args = vec![OscType::Int(track as i32), OscType::Int(device as i32)];
        for value in &values {
            args.push(OscType::Float(*value));
        }

        self.osc
            .send("/live/device/set/parameters/value", args)
            .await?;
        Ok(format!(
            "Set {} parameters on device {device} (track {track})",
            values.len()
        ))
    }

    /// Get detailed information about all device parameters.
    #[tool(description = "Get detailed information about all device parameters")]
    pub async fn get_device_parameters_detailed(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;

        // Get all parameter names
        let names_packets = self
            .osc
            .query_all(
                "/live/device/get/parameters/name",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await
            .unwrap_or_default();

        let mut names = Vec::new();
        for packet in names_packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::String(s) = arg {
                        names.push(s);
                    }
                }
            }
        }

        // Get all parameter values
        let values_packets = self
            .osc
            .query_all(
                "/live/device/get/parameters/value",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await
            .unwrap_or_default();

        let mut values = Vec::new();
        for packet in values_packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Float(f) = arg {
                        values.push(f);
                    }
                }
            }
        }

        // Get all parameter mins
        let mins_packets = self
            .osc
            .query_all(
                "/live/device/get/parameters/min",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await
            .unwrap_or_default();

        let mut mins = Vec::new();
        for packet in mins_packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Float(f) = arg {
                        mins.push(f);
                    }
                }
            }
        }

        // Get all parameter maxs
        let maxs_packets = self
            .osc
            .query_all(
                "/live/device/get/parameters/max",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await
            .unwrap_or_default();

        let mut maxs = Vec::new();
        for packet in maxs_packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    if let OscType::Float(f) = arg {
                        maxs.push(f);
                    }
                }
            }
        }

        // Get all parameter quantized flags
        let quantized_packets = self
            .osc
            .query_all(
                "/live/device/get/parameters/is_quantized",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await
            .unwrap_or_default();

        let mut quantized = Vec::new();
        for packet in quantized_packets {
            if let OscPacket::Message(msg) = packet {
                for arg in msg.args {
                    match arg {
                        OscType::Int(i) => quantized.push(i != 0),
                        OscType::Bool(b) => quantized.push(b),
                        _ => {}
                    }
                }
            }
        }

        // Build parameter structures
        let len = names
            .len()
            .min(values.len())
            .min(mins.len())
            .min(maxs.len())
            .min(quantized.len());

        let mut parameters = Vec::new();
        for i in 0..len {
            parameters.push(ParameterStructure {
                name: names[i].clone(),
                value: values[i],
                min: mins[i],
                max: maxs[i],
                is_quantized: quantized[i],
            });
        }

        Ok(serde_json::to_string_pretty(&parameters).unwrap_or_else(|_| "[]".into()))
    }
}
