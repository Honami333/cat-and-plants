use crate::schema::types_and_states::*;
use bevy::prelude::*;

// Механика роста
pub fn plant_growth(mut inv: ResMut<GlobalInventory>, upgrade_storege: Res<UpgradeStorege>) {
    let invetories = [&mut inv.sunlit_nursery_inv];

    let mut up_value =  1.0;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::SelectiveBreeding) {up_value = value};

    for inventory in invetories {
        for slot in inventory.iter_mut() {
            if let SlotState::Occupied(plant) = slot {
                if plant.growth_score < (plant.growth_thereshold / up_value) {
                    plant.growth_score += plant.growth_rate;
                }

                let p = plant.growth_score / (plant.growth_thereshold / up_value);

                plant.state = match p {
                    _ if p <= 0.25 => PlantStateGrowth::Seed,
                    _ if p <= 0.5 => PlantStateGrowth::Sprout,
                    _ if p < 1.0 => PlantStateGrowth::Sapling,
                    _  => PlantStateGrowth::Mature,
                }
            }
        }
    }
}
