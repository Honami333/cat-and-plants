use crate::schema::types_and_states::*;
use bevy::prelude::*;

// Механика роста
pub fn plant_growth(mut inv: ResMut<GlobalInventory>, upgrade_storege: Res<UpgradeStorege>) {
    let invetories = [&mut inv.sunlit_nursery_inv];

    let selective_breeding_upgrade =
        upgrade_storege.get_global_modifier(UpgradeUID::SelectiveBreeding);

    for inventory in invetories {
        for slot in inventory.iter_mut() {
            if let SlotState::Occupied(plant) = slot
                && plant.growth_score < (plant.growth_thereshold / selective_breeding_upgrade)
            {
                plant.growth_score += plant.growth_rate;

                let growth_pct =
                    plant.growth_score / (plant.growth_thereshold / selective_breeding_upgrade);

                match growth_pct {
                    p if p >= 0.25 && p < 0.50 => {
                        plant.state = PlantStateGrowth::Sprout;
                    }
                    p if p >= 0.50 && p < 0.75 => {
                        plant.state = PlantStateGrowth::Sapling;
                    }
                    p if p >= 0.75 && p <= 1.0 => {
                        plant.state = PlantStateGrowth::Mature;
                    }
                    _ => {}
                }
            }
        }
    }
}
