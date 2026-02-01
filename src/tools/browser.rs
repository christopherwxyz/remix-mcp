//! Browser tools for loading instruments, effects, presets, and navigating the browser.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::{OscPacket, OscType};

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    BrowseParams, BrowsePathParams, DeviceParams, GetBrowserItemParams,
    ListWithOptionalCategoryParams, LoadByNameParams, LoadDrumKitParams, LoadUserPresetParams,
    SearchBrowserParams,
};

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

/// Format a list of items for display, with a header and empty message.
fn format_list(items: &[String], header: &str, empty_msg: &str) -> String {
    if items.is_empty() {
        empty_msg.to_string()
    } else {
        format!("{header}\n{}", items.join("\n"))
    }
}

#[tool_router(router = browser_router, vis = "pub")]
impl AbletonServer {
    // =========================================================================
    // Instruments
    // =========================================================================

    /// Load the default instrument (Drift synth) onto the selected track.
    #[tool(description = "Load the default instrument (Drift synth) onto the selected track")]
    pub async fn load_default_instrument(&self) -> Result<String, Error> {
        self.osc
            .send("/live/browser/load_default_instrument", vec![])
            .await?;
        Ok("Loaded default instrument (Drift synth)".to_string())
    }

    /// Load an instrument by name onto the selected track.
    #[tool(description = "Load an instrument by name onto the selected track")]
    pub async fn load_instrument(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_instrument",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded instrument: {name}"))
    }

    /// Load a drum kit onto the selected track.
    #[tool(description = "Load a drum kit onto the selected track")]
    pub async fn load_drum_kit(
        &self,
        Parameters(params): Parameters<LoadDrumKitParams>,
    ) -> Result<String, Error> {
        let args = params
            .name
            .as_ref()
            .map(|n| vec![OscType::String(n.clone())])
            .unwrap_or_default();
        self.osc.send("/live/browser/load_drum_kit", args).await?;
        Ok(params.name.as_ref().map_or_else(
            || "Loaded default drum kit".to_string(),
            |n| format!("Loaded drum kit: {n}"),
        ))
    }

    // =========================================================================
    // Audio & MIDI Effects
    // =========================================================================

    /// Load an audio effect by name onto the selected track.
    #[tool(description = "Load an audio effect by name onto the selected track")]
    pub async fn load_audio_effect(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_audio_effect",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded audio effect: {name}"))
    }

    /// Load a MIDI effect by name onto the selected track.
    #[tool(description = "Load a MIDI effect by name onto the selected track")]
    pub async fn load_midi_effect(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_midi_effect",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded MIDI effect: {name}"))
    }

    /// Load the default audio effect (Reverb) onto the selected track.
    #[tool(description = "Load the default audio effect (Reverb) onto the selected track")]
    pub async fn load_default_audio_effect(&self) -> Result<String, Error> {
        self.osc
            .send("/live/browser/load_default_audio_effect", vec![])
            .await?;
        Ok("Loaded default audio effect (Reverb)".to_string())
    }

    /// Load the default MIDI effect (Arpeggiator) onto the selected track.
    #[tool(description = "Load the default MIDI effect (Arpeggiator) onto the selected track")]
    pub async fn load_default_midi_effect(&self) -> Result<String, Error> {
        self.osc
            .send("/live/browser/load_default_midi_effect", vec![])
            .await?;
        Ok("Loaded default MIDI effect (Arpeggiator)".to_string())
    }

    /// List available audio effects.
    #[tool(description = "List available audio effects")]
    pub async fn list_audio_effects(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/browser/list_audio_effects", vec![])
            .await
            .unwrap_or_default();
        let effects = extract_strings_from_packets(packets);
        Ok(format_list(
            &effects,
            "Audio effects:",
            "No audio effects found",
        ))
    }

    /// List available MIDI effects.
    #[tool(description = "List available MIDI effects")]
    pub async fn list_midi_effects(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/browser/list_midi_effects", vec![])
            .await
            .unwrap_or_default();
        let effects = extract_strings_from_packets(packets);
        Ok(format_list(
            &effects,
            "MIDI effects:",
            "No MIDI effects found",
        ))
    }

    // =========================================================================
    // Sounds & Presets
    // =========================================================================

    /// Load a sound preset by name onto the selected track.
    #[tool(description = "Load a sound preset by name onto the selected track")]
    pub async fn load_sound(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_sound",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded sound: {name}"))
    }

    /// List available sound presets.
    #[tool(description = "List available sound presets")]
    pub async fn list_sounds(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/browser/list_sounds", vec![])
            .await
            .unwrap_or_default();
        let sounds = extract_strings_from_packets(packets);
        Ok(format_list(&sounds, "Sound categories:", "No sounds found"))
    }

    // =========================================================================
    // Samples & Clips
    // =========================================================================

    /// Load a sample by name onto the selected track (via Simpler).
    #[tool(description = "Load a sample by name onto the selected track (via Simpler)")]
    pub async fn load_sample(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_sample",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded sample: {name}"))
    }

    /// Load a clip by name.
    #[tool(description = "Load a clip by name")]
    pub async fn load_clip(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_clip",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded clip: {name}"))
    }

    /// List available samples, optionally filtered by category.
    #[tool(description = "List available samples, optionally filtered by category")]
    pub async fn list_samples(
        &self,
        Parameters(params): Parameters<ListWithOptionalCategoryParams>,
    ) -> Result<String, Error> {
        let args = params
            .category
            .as_ref()
            .map(|c| vec![OscType::String(c.clone())])
            .unwrap_or_default();
        let packets = self
            .osc
            .query_all("/live/browser/list_samples", args)
            .await
            .unwrap_or_default();
        let samples = extract_strings_from_packets(packets);
        let header = params.category.as_ref().map_or_else(
            || "Sample categories:".to_string(),
            |c| format!("Samples in '{c}':"),
        );
        Ok(format_list(&samples, &header, "No samples found"))
    }

    /// List available clips, optionally filtered by category.
    #[tool(description = "List available clips, optionally filtered by category")]
    pub async fn list_clips(
        &self,
        Parameters(params): Parameters<ListWithOptionalCategoryParams>,
    ) -> Result<String, Error> {
        let args = params
            .category
            .as_ref()
            .map(|c| vec![OscType::String(c.clone())])
            .unwrap_or_default();
        let packets = self
            .osc
            .query_all("/live/browser/list_clips", args)
            .await
            .unwrap_or_default();
        let clips = extract_strings_from_packets(packets);
        let header = params.category.as_ref().map_or_else(
            || "Clip categories:".to_string(),
            |c| format!("Clips in '{c}':"),
        );
        Ok(format_list(&clips, &header, "No clips found"))
    }

    // =========================================================================
    // Plugins & Max4Live
    // =========================================================================

    /// Load a VST/AU plugin by name onto the selected track.
    #[tool(description = "Load a VST/AU plugin by name onto the selected track")]
    pub async fn load_plugin(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_plugin",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded plugin: {name}"))
    }

    /// Load a Max for Live device by name onto the selected track.
    #[tool(description = "Load a Max for Live device by name onto the selected track")]
    pub async fn load_max_device(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/load_max_device",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Loaded Max for Live device: {name}"))
    }

    /// List available VST/AU plugins.
    #[tool(description = "List available VST/AU plugins")]
    pub async fn list_plugins(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/browser/list_plugins", vec![])
            .await
            .unwrap_or_default();
        let plugins = extract_strings_from_packets(packets);
        Ok(format_list(&plugins, "Plugins:", "No plugins found"))
    }

    /// List available Max for Live devices.
    #[tool(description = "List available Max for Live devices")]
    pub async fn list_max_devices(&self) -> Result<String, Error> {
        let packets = self
            .osc
            .query_all("/live/browser/list_max_devices", vec![])
            .await
            .unwrap_or_default();
        let devices = extract_strings_from_packets(packets);
        Ok(format_list(
            &devices,
            "Max for Live devices:",
            "No Max for Live devices found",
        ))
    }

    // =========================================================================
    // Browser Navigation
    // =========================================================================

    /// Browse a top-level browser category.
    #[tool(description = "Browse a top-level browser category")]
    pub async fn browse(
        &self,
        Parameters(params): Parameters<BrowseParams>,
    ) -> Result<String, Error> {
        let category = params.category;
        let packets = self
            .osc
            .query_all(
                "/live/browser/browse",
                vec![OscType::String(category.clone())],
            )
            .await
            .unwrap_or_default();
        let items = extract_strings_from_packets(packets);
        Ok(format_list(
            &items,
            &format!("Items in '{category}':"),
            &format!("No items found in category: {category}"),
        ))
    }

    /// Browse a specific path in the browser.
    #[tool(description = "Browse a specific path in the browser")]
    pub async fn browse_path(
        &self,
        Parameters(params): Parameters<BrowsePathParams>,
    ) -> Result<String, Error> {
        let category = params.category;
        let path = params.path;
        let packets = self
            .osc
            .query_all(
                "/live/browser/browse_path",
                vec![
                    OscType::String(category.clone()),
                    OscType::String(path.clone()),
                ],
            )
            .await
            .unwrap_or_default();
        let items = extract_strings_from_packets(packets);
        Ok(format_list(
            &items,
            &format!("Items at '{category}/{path}':"),
            &format!("No items found at path: {category}/{path}"),
        ))
    }

    /// Search for items across the browser.
    #[tool(description = "Search for items across the browser")]
    pub async fn search_browser(
        &self,
        Parameters(params): Parameters<SearchBrowserParams>,
    ) -> Result<String, Error> {
        let query = params.query;
        let packets = self
            .osc
            .query_all("/live/browser/search", vec![OscType::String(query.clone())])
            .await
            .unwrap_or_default();
        let results = extract_strings_from_packets(packets);
        if results.is_empty() {
            return Ok(format!("No results found for: {query}"));
        }

        // Results come as alternating category, name pairs
        let formatted: Vec<String> = results
            .chunks(2)
            .filter_map(|chunk| match chunk {
                [cat, name] => Some(format!("[{cat}] {name}")),
                _ => None,
            })
            .collect();

        Ok(format!(
            "Search results for '{query}':\n{}",
            formatted.join("\n")
        ))
    }

    /// Get information about a browser item.
    #[tool(description = "Get information about a browser item")]
    pub async fn get_browser_item(
        &self,
        Parameters(params): Parameters<GetBrowserItemParams>,
    ) -> Result<String, Error> {
        let category = params.category;
        let name = params.name;
        let packets = self
            .osc
            .query_all(
                "/live/browser/get_item_info",
                vec![
                    OscType::String(category.clone()),
                    OscType::String(name.clone()),
                ],
            )
            .await
            .unwrap_or_default();
        let info = extract_strings_from_packets(packets);
        if info.is_empty() {
            Ok(format!("Item not found: {category}/{name}"))
        } else {
            let item_name = info
                .first()
                .map(std::string::String::as_str)
                .unwrap_or("Unknown");
            Ok(format!(
                "Browser item: {item_name}\nCategory: {category}\nRaw info: {info:?}"
            ))
        }
    }

    // =========================================================================
    // User Library
    // =========================================================================

    /// List presets in the user library, optionally filtered by category.
    #[tool(description = "List presets in the user library, optionally filtered by category")]
    pub async fn list_user_presets(
        &self,
        Parameters(params): Parameters<ListWithOptionalCategoryParams>,
    ) -> Result<String, Error> {
        let args = params
            .category
            .as_ref()
            .map(|c| vec![OscType::String(c.clone())])
            .unwrap_or_default();
        let packets = self
            .osc
            .query_all("/live/browser/list_user_presets", args)
            .await
            .unwrap_or_default();
        let presets = extract_strings_from_packets(packets);
        let header = params.category.as_ref().map_or_else(
            || "User library:".to_string(),
            |c| format!("User presets in '{c}':"),
        );
        Ok(format_list(&presets, &header, "No user presets found"))
    }

    /// Load a preset from the user library.
    #[tool(description = "Load a preset from the user library")]
    pub async fn load_user_preset(
        &self,
        Parameters(params): Parameters<LoadUserPresetParams>,
    ) -> Result<String, Error> {
        let path = params.path;
        self.osc
            .send(
                "/live/browser/load_user_preset",
                vec![OscType::String(path.clone())],
            )
            .await?;
        Ok(format!("Loaded user preset: {path}"))
    }

    // =========================================================================
    // Hotswap & Preview
    // =========================================================================

    /// Enter hotswap mode for a specific device.
    #[tool(description = "Enter hotswap mode for a specific device")]
    pub async fn hotswap_start(
        &self,
        Parameters(params): Parameters<DeviceParams>,
    ) -> Result<String, Error> {
        let track = params.track;
        let device = params.device;
        self.osc
            .send(
                "/live/browser/hotswap_start",
                vec![OscType::Int(track as i32), OscType::Int(device as i32)],
            )
            .await?;
        Ok(format!(
            "Started hotswap for track {track}, device {device}"
        ))
    }

    /// Load an item via hotswap mode.
    #[tool(description = "Load an item via hotswap mode")]
    pub async fn hotswap_load(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/hotswap_load",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Hotswap loaded: {name}"))
    }

    /// Preview a sample before loading.
    #[tool(description = "Preview a sample before loading")]
    pub async fn preview_sample(
        &self,
        Parameters(params): Parameters<LoadByNameParams>,
    ) -> Result<String, Error> {
        let name = params.name;
        self.osc
            .send(
                "/live/browser/preview_sample",
                vec![OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Previewing sample: {name}"))
    }

    /// Stop sample preview playback.
    #[tool(description = "Stop sample preview playback")]
    pub async fn stop_preview(&self) -> Result<String, Error> {
        self.osc.send("/live/browser/stop_preview", vec![]).await?;
        Ok("Stopped preview".to_string())
    }
}
