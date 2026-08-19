
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use bevy::prelude::*;
use crate::systems::locales::Language;


#[derive(Resource, Clone, Copy, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum ScreenMode  {
    #[strum(serialize = "screen-windowed")] Windowed,
    #[strum(serialize = "screen-fullscreen")] Fullscreen,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalSettings {
    pub fps: MaxFPS,
    pub display: DisplaySettings,
    pub shader: ShaderSettings,
    pub autosave_interval: f64,
    pub language: Language,
    pub audio: Audio,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub screen_mode: ScreenMode,
    pub resolution: [f32; 2],
    pub max_display: [f32; 2],
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct ShaderSettings {
    pub light_shaders: bool,
    pub dust_particles: bool,
    pub dust_amount: f32,
    pub breeze_shaders: bool,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct MaxFPS {
    pub limit: bool,
    pub max_fps: f64,
    pub foces_fps: f64,
    pub unfoces_fps: f64,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct Audio {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            fps: MaxFPS {
                limit: true,
                max_fps: 60.0,
                foces_fps: 60.0,
                unfoces_fps: 5.0,
            },

            display: DisplaySettings {
                screen_mode: ScreenMode::Fullscreen,
                resolution: [1920.0, 1080.0],
                max_display: [0.0, 0.0],
            },

            shader: ShaderSettings {
                light_shaders: true,
                dust_particles: true,
                dust_amount: 0.5,
                breeze_shaders: true,
            },

            autosave_interval: 180.0,

            language: Language::En,

            audio: Audio {
                master_volume: 100.0,
                sfx_volume: 65.0,
                music_volume: 80.0
            }
        }
    }
}

