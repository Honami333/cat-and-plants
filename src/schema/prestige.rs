
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use super::common::CurrentWorld;
use super::economy_inventory::ResourceType;
use std::borrow::Borrow;


#[derive(Resource, Clone, Copy, Default,  Serialize, Deserialize, Debug)]
pub struct PrestigeRoom {
    pub sunlit_nursery: usize,
    pub shadow_greenhouse: usize,
}

impl PrestigeRoom {
    pub fn get_room<W>(&self, current_world: W) -> Option<usize> 
        where 
            W: Borrow<CurrentWorld>
        {

        match current_world.borrow() {
            CurrentWorld::SunlitNursery => Some(self.sunlit_nursery),
            CurrentWorld::ShadowGreenhouse => Some(self.shadow_greenhouse),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_mut_room(&mut self, current_world: &State<CurrentWorld>) -> Option<&mut usize> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery),
            CurrentWorld::ShadowGreenhouse => Some(&mut self.shadow_greenhouse),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_sparks_res(&self, current_world: &State<CurrentWorld>) -> Option<ResourceType>{
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(ResourceType::SunSparks),
            CurrentWorld::ShadowGreenhouse => Some(ResourceType::PhotoSparks),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn first_prestige(&self) -> bool {
        let rooms = [self.sunlit_nursery, self.shadow_greenhouse];

        for room in rooms {
            if room > 0 { return true };
        };

        false
    }

    pub fn get_all_prestige(&self) -> usize {
        let rooms = [self.sunlit_nursery, self.shadow_greenhouse];

        let mut prestige_count = 0;

        for room in rooms {
            prestige_count += room;
        };

        prestige_count
    }
}