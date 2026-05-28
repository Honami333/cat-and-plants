use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use strum_macros::Display;
use crate::content::world::sunlit_nursery::*;
use super::economy_inventory::ResourceType;


pub trait MapStore<T> {
    fn get_for_world (&self, world: &State<CurrentWorld>) -> Option<&T>;
    fn get_for_world_mut (&mut self, _world: &State<CurrentWorld>) -> Option<&mut T> {
        None
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    // Стадии загрузки
    #[default]
    Loading,
    Menu,
    LoadGame,
    Playing,
}

#[derive(States, Hash, Resource, Default, Clone, Copy, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
pub enum CurrentWorld {
    #[default]
    #[strum(serialize = "world-warm-paws")] WarmPawsPorch,
    #[strum(serialize = "world-sunlit-nursery")] SunlitNursery,
}

impl CurrentWorld {
    pub fn get_prestige_cost(&self) -> Option<&[(ResourceType, f64)]> {
        match self {
            CurrentWorld::SunlitNursery => Some(SN_FIRST_PRESTIGE_COST.cost),
            CurrentWorld::WarmPawsPorch => None
        }
    }

    pub fn get_cost(&self, res: ResourceType, pr_room: usize) -> Option<f64> {
        let Some(res_cost) = self.get_prestige_cost() else { return None; };

        let Some((_, cost)) = res_cost.iter().find(|(r, _)| *r == res) else { return None; };

        if matches!(res, ResourceType::CatHappiness) {
            Some(*cost * (pr_room as f64 + 1.0).powf(1.8))
        } else {
            Some(*cost * (pr_room as f64 + 1.0).powf(1.2))
        }
    }
}

#[derive(Resource, Default)]
pub struct WorldScale(pub Vec2);

