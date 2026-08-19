use rodio::MixerDeviceSink;
use rodio::DeviceSinkBuilder;
use rodio::Player;
use rodio::Decoder;

use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use std::fs::File;
use std::io::Read;

use crate::schema::global_settings::GlobalSettings;


#[derive(Resource)]
pub struct AuidioSystem {
    handle: Option<MixerDeviceSink>,
    music_player: Option<Player>,

    sfx_map: HashMap<AudioId, Vec<u8>>,
    music_map: HashMap<AudioId, Vec<u8>>,
}

impl Default for AuidioSystem {
    fn default() -> Self {
        let handle = DeviceSinkBuilder::open_default_sink();

        let mut music_player = None;

        if let Ok(ok_handle) = &handle {
            music_player = Some(Player::connect_new(ok_handle.mixer()));
        };

        Self {
            handle: handle.ok(),
            music_player,
            sfx_map: HashMap::new(),
            music_map: HashMap::new(),
        }
    }
}

pub struct AudioId(pub usize);

impl AudioId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get_id(&self) -> usize {
        self.0
    }
}

pub fn setup(
    mut auidio_system: ResMut<AuidioSystem>,
    settings: Res<GlobalSettings>,
) {
    let Ok(sfx) = File::open("") else { return; };

    let Ok(decode_sfx) = Decoder::try_from(sfx) else { return; };
}