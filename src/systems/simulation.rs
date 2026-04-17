use bevy::prelude::*;
use crate::schema::{types_and_states::*};


// Механика роста
pub fn plant_growth(
    current_world: Res<CurrentWorld>,
    mut inv: ResMut<GlobalInventory>,
) {
    let inventory = current_world.get_inv_mut(&mut inv);
    
    for slot in inventory.iter_mut() {
    if let SlotState::Occupied(plant) = slot && plant.growth_score < plant.growth_thereshold {
        plant.growth_score += 1;

        let growth_pct: f32 = plant.growth_score as f32 / plant.growth_thereshold as f32;

        match (plant.state, growth_pct) {
            (PlantStateGrowth::Seed(PlantStateUpdate::Idle), p) if p >= 0.25 && p < 0.50 => {
                plant.state = PlantStateGrowth::Seed(PlantStateUpdate::Growth);
            },
            (PlantStateGrowth::Sprout(PlantStateUpdate::Idle), p) if p >= 0.50 && p < 0.75 => {
                plant.state = PlantStateGrowth::Sprout(PlantStateUpdate::Growth);
            },
            (PlantStateGrowth::Sapling(PlantStateUpdate::Idle), p) if p >= 0.75 && p <= 1.0 => {
                plant.state = PlantStateGrowth::Sapling(PlantStateUpdate::Growth);
            },
            _ => {}
        }
    }}
}