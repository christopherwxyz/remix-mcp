//! Scene management tools.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use rosc::OscType;

use crate::error::Error;
use crate::server::AbletonServer;
use crate::types::{
    CreateSceneParams, SceneInfo, SceneParams, SetSceneColorParams, SetSceneNameParams,
    SetSceneTempoEnabledParams, SetSceneTempoParams, SetSceneTimeSigEnabledParams,
    SetSceneTimeSignatureParams,
};

#[tool_router(router = scenes_router, vis = "pub")]
impl AbletonServer {
    /// List all scenes in the song.
    #[tool(description = "List all scenes in the song")]
    pub async fn list_scenes(&self) -> Result<String, Error> {
        let count: i32 = self.osc.query("/live/song/get/num_scenes", vec![]).await?;

        let mut scenes = Vec::new();
        for i in 0..count {
            let name: String = self
                .osc
                .query("/live/scene/get/name", vec![OscType::Int(i)])
                .await
                .unwrap_or_else(|_| format!("Scene {}", i + 1));

            scenes.push(SceneInfo {
                index: i as u32,
                name,
            });
        }

        Ok(serde_json::to_string_pretty(&scenes).unwrap_or_else(|_| "[]".into()))
    }

    /// Fire (trigger) a scene by index.
    #[tool(description = "Fire (trigger) a scene by index")]
    pub async fn fire_scene(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        self.osc
            .send("/live/scene/fire", vec![OscType::Int(scene as i32)])
            .await?;
        Ok(format!("Fired scene {scene}"))
    }

    /// Create a new scene at an optional index.
    #[tool(description = "Create a new scene at an optional index")]
    pub async fn create_scene(
        &self,
        Parameters(params): Parameters<CreateSceneParams>,
    ) -> Result<String, Error> {
        let args = match params.index {
            Some(i) => vec![OscType::Int(i)],
            None => vec![],
        };
        self.osc.send("/live/song/create_scene", args).await?;
        Ok(match params.index {
            Some(i) => format!("Created scene at index {i}"),
            None => "Created new scene".to_string(),
        })
    }

    /// Delete a scene by index.
    #[tool(description = "Delete a scene by index")]
    pub async fn delete_scene(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        self.osc
            .send("/live/song/delete_scene", vec![OscType::Int(scene as i32)])
            .await?;
        Ok(format!("Deleted scene {scene}"))
    }

    /// Duplicate a scene by index.
    #[tool(description = "Duplicate a scene by index")]
    pub async fn duplicate_scene(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        self.osc
            .send(
                "/live/song/duplicate_scene",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        Ok(format!("Duplicated scene {scene}"))
    }

    /// Set a scene's name.
    #[tool(description = "Set a scene's name")]
    pub async fn set_scene_name(
        &self,
        Parameters(params): Parameters<SetSceneNameParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let name = params.name.clone();
        self.osc
            .send(
                "/live/scene/set/name",
                vec![OscType::Int(scene as i32), OscType::String(name.clone())],
            )
            .await?;
        Ok(format!("Scene {scene} renamed to \"{name}\""))
    }

    /// Get a scene's color (RGB integer).
    #[tool(description = "Get a scene's color (RGB integer)")]
    pub async fn get_scene_color(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let color: i32 = self
            .osc
            .query("/live/scene/get/color", vec![OscType::Int(scene as i32)])
            .await?;
        Ok(format!("Scene {scene} color: {color}"))
    }

    /// Set a scene's color (RGB integer).
    #[tool(description = "Set a scene's color (RGB integer)")]
    pub async fn set_scene_color(
        &self,
        Parameters(params): Parameters<SetSceneColorParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let color = params.color;
        self.osc
            .send(
                "/live/scene/set/color",
                vec![OscType::Int(scene as i32), OscType::Int(color)],
            )
            .await?;
        Ok(format!("Scene {scene} color set to {color}"))
    }

    /// Get a scene's tempo.
    #[tool(description = "Get a scene's tempo")]
    pub async fn get_scene_tempo(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let tempo: f32 = self
            .osc
            .query("/live/scene/get/tempo", vec![OscType::Int(scene as i32)])
            .await?;
        Ok(format!("Scene {scene} tempo: {tempo} BPM"))
    }

    /// Set a scene's tempo.
    #[tool(description = "Set a scene's tempo")]
    pub async fn set_scene_tempo(
        &self,
        Parameters(params): Parameters<SetSceneTempoParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let tempo = params.tempo;
        self.osc
            .send(
                "/live/scene/set/tempo",
                vec![OscType::Int(scene as i32), OscType::Float(tempo)],
            )
            .await?;
        Ok(format!("Scene {scene} tempo set to {tempo} BPM"))
    }

    /// Get whether a scene's tempo is enabled.
    #[tool(description = "Get whether a scene's tempo is enabled")]
    pub async fn get_scene_tempo_enabled(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let result: i32 = self
            .osc
            .query(
                "/live/scene/get/tempo_enabled",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Scene {scene} tempo is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set whether a scene's tempo is enabled.
    #[tool(description = "Set whether a scene's tempo is enabled")]
    pub async fn set_scene_tempo_enabled(
        &self,
        Parameters(params): Parameters<SetSceneTempoEnabledParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/scene/set/tempo_enabled",
                vec![
                    OscType::Int(scene as i32),
                    OscType::Int(if enabled { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Scene {scene} tempo {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Get a scene's time signature numerator.
    #[tool(description = "Get a scene's time signature numerator")]
    pub async fn get_scene_time_sig_numerator(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let numerator: i32 = self
            .osc
            .query(
                "/live/scene/get/time_signature_numerator",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        Ok(format!(
            "Scene {scene} time signature numerator: {numerator}"
        ))
    }

    /// Get a scene's time signature denominator.
    #[tool(description = "Get a scene's time signature denominator")]
    pub async fn get_scene_time_sig_denominator(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let denominator: i32 = self
            .osc
            .query(
                "/live/scene/get/time_signature_denominator",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        Ok(format!(
            "Scene {scene} time signature denominator: {denominator}"
        ))
    }

    /// Set a scene's time signature.
    #[tool(description = "Set a scene's time signature")]
    pub async fn set_scene_time_signature(
        &self,
        Parameters(params): Parameters<SetSceneTimeSignatureParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let numerator = params.numerator;
        let denominator = params.denominator;
        self.osc
            .send(
                "/live/scene/set/time_signature_numerator",
                vec![OscType::Int(scene as i32), OscType::Int(numerator)],
            )
            .await?;
        self.osc
            .send(
                "/live/scene/set/time_signature_denominator",
                vec![OscType::Int(scene as i32), OscType::Int(denominator)],
            )
            .await?;
        Ok(format!(
            "Scene {scene} time signature set to {numerator}/{denominator}"
        ))
    }

    /// Get whether a scene's time signature is enabled.
    #[tool(description = "Get whether a scene's time signature is enabled")]
    pub async fn get_scene_time_sig_enabled(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let result: i32 = self
            .osc
            .query(
                "/live/scene/get/time_signature_enabled",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        let enabled = result != 0;
        Ok(format!(
            "Scene {scene} time signature is {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Set whether a scene's time signature is enabled.
    #[tool(description = "Set whether a scene's time signature is enabled")]
    pub async fn set_scene_time_sig_enabled(
        &self,
        Parameters(params): Parameters<SetSceneTimeSigEnabledParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let enabled = params.enabled;
        self.osc
            .send(
                "/live/scene/set/time_signature_enabled",
                vec![
                    OscType::Int(scene as i32),
                    OscType::Int(if enabled { 1 } else { 0 }),
                ],
            )
            .await?;
        Ok(format!(
            "Scene {scene} time signature {}",
            if enabled { "enabled" } else { "disabled" }
        ))
    }

    /// Check if a scene is triggered/playing.
    #[tool(description = "Check if a scene is triggered/playing")]
    pub async fn is_scene_triggered(
        &self,
        Parameters(params): Parameters<SceneParams>,
    ) -> Result<String, Error> {
        let scene = params.scene;
        let result: i32 = self
            .osc
            .query(
                "/live/scene/get/is_triggered",
                vec![OscType::Int(scene as i32)],
            )
            .await?;
        let triggered = result != 0;
        Ok(format!(
            "Scene {scene} is {}",
            if triggered {
                "triggered"
            } else {
                "not triggered"
            }
        ))
    }

    /// Fire the currently selected scene.
    #[tool(description = "Fire the currently selected scene")]
    pub async fn fire_selected_scene(&self) -> Result<String, Error> {
        self.osc.send("/live/scene/fire_selected", vec![]).await?;
        Ok("Fired selected scene".to_string())
    }
}
