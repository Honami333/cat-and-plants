use bevy::prelude::*;
use crate::schema::types_and_states::*;


// Механика роста
pub fn plant_growth(
    mut inv: ResMut<GlobalInventory>,
    upgrade_storege: Res<UpgradeStorege>,
) {
    let invetories = [
        &mut inv.sunlit_nursery_inv,
    ];

    let mut selective_breeding_upgrade = 1.0;

    for upgrade in upgrade_storege.global.iter() {
        if upgrade.id == UpgradeUID::SelectiveBreeding && upgrade.current_level > 0 {
            selective_breeding_upgrade = upgrade.levels[upgrade.current_level - 1].value;
        };
    };

    for inventory in invetories {
    for slot in inventory.iter_mut() {
    if let SlotState::Occupied(plant) = slot && plant.growth_score < (plant.growth_thereshold / selective_breeding_upgrade) {
        plant.growth_score += plant.growth_rate;

        let growth_pct = plant.growth_score / (plant.growth_thereshold / selective_breeding_upgrade);

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
    }}}
}