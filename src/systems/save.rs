use std::{fs::{File, create_dir_all}, io::{Read, Write}, path::PathBuf, time::Duration};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce, Key};
use crate::schema::{save_file::*, global_settings::*, common::*, global_inventory::*, item_type_info::*, economy_inventory::*, prestige::*, upgrade_storege::*};
use crate::content::world::sunlit_nursery::*;
use super::simulation::monitor_window_settings;
use bevy::{app::AppExit, prelude::*, window::{PrimaryWindow, WindowFocused}};
use directories::ProjectDirs;

const ENCRYPTION_KEY: &[u8; 32] = b"h7X9vK2mN4pQ1rT6wZ8xY0zC3bV5nM7q";
const CRYPTO_NONCE: &[u8; 12] = b"aB4kL9wP2xZ1";


pub fn event_save_system(    
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut exit_events: MessageReader<AppExit>,
    mut focus_events: MessageReader<WindowFocused>,
    mut timer: Local<Timer>,
    mut settings: ResMut<GlobalSettings>,
    time: Res<Time>,
    up_storege: Res<UpgradeStorege>,
    global_storege: Res<GlobalInventory>,
    eco_storege: Res<Economy>,
    iti_storege: Res<ItemTypeInfo>,
    save_slot_inv: Res<SaveSlotInv>,
    pristige_room: Res<PrestigeRoom>,
    world: Res<State<CurrentWorld>>,
) {
    let mut need_save = false;

    timer.set_duration(Duration::from_secs_f64(settings.autosave_interval));

    timer.tick(time.delta());

    if timer.is_finished() {
        need_save = true;
        timer.reset();
    };

    if exit_events.read().count() > 0 {
        need_save = true;
    };

    for event in focus_events.read() {
        if !event.focused {
            need_save = true;
        };
    };

    if !need_save { return; };

    auto_save_system(&up_storege, &global_storege, &eco_storege, &iti_storege, &save_slot_inv, &world, &pristige_room);

    let setting_save_path = get_setting_path();

    
    let Ok(mut window) = window_query.single_mut() else { return; };
    
    if let Err(e) = setting_save(&mut window, &mut settings, setting_save_path) {
        error!("Critical autosave error: {}", e);
    } else {
        info!("Autosave completed successfully!");
    }
}

pub fn auto_save_system(
    up_storege: &UpgradeStorege,
    global_storege: &GlobalInventory,
    eco_storege: &Economy,
    iti_storege: &ItemTypeInfo,
    save_slot_inv: &SaveSlotInv,
    world: &State<CurrentWorld>,
    pristige_room: &PrestigeRoom
) {
    let contener = SaveDataContainer {
        up_storege: up_storege.clone(),
        global_storege: global_storege.clone(),
        eco_storege: eco_storege.clone(),
        iti_storege: iti_storege.clone(),
        prestige: pristige_room.clone(),
        world: **world,
    };


    if let Err(e) = save_game_to_disk(&contener, save_slot_inv.active_slot) {
        error!("Critical autosave error: {}", e);
    } else {
        info!("Autosave completed successfully!");
    }
}

fn save_game_to_disk(
    contener: &SaveDataContainer,
    active_slot: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_bytes = ron::to_string(contener)?.into_bytes();

    let key = Key::<Aes256Gcm>::from_slice(ENCRYPTION_KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(CRYPTO_NONCE);

    let Some(i) = active_slot else { return Err(From::from("Attempted to save without an active slot!"))};

    let encrypted_bytes = cipher
        .encrypt(nonce, raw_bytes.as_slice())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    let mut file = File::create(get_save_path(i))?;
    file.write_all(&encrypted_bytes)?;

    Ok(())
}

pub fn get_save_path(i: usize) -> PathBuf {
    let file_name = format!("slot_{}.dat", i + 1);

    if let Some(proj_dirs) = ProjectDirs::from("com", "Honami", "CatAndPlants") {
        let save_dir = proj_dirs.data_dir();

        let _ = create_dir_all(save_dir);

        return save_dir.join(file_name);
    }

    PathBuf::from(file_name)
}

fn load_game_from_disk(i: usize) -> Result<SaveDataContainer, Box<dyn std::error::Error>> {
    let save_path = get_save_path(i);

    let mut file = File::open(save_path)?;
    let mut encrypted_bytes = Vec::new();
    file.read_to_end(&mut encrypted_bytes)?;

    let key = Key::<Aes256Gcm>::from_slice(ENCRYPTION_KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(CRYPTO_NONCE);

    let decrypted_bytes  = cipher
        .decrypt(nonce, encrypted_bytes.as_slice())
        .map_err(|e| format!("Decryption error: {:?}", e))?;

    let parsed: ron::Value = ron::de::from_bytes(&decrypted_bytes)
         .map_err(|e| format!("RON dynamic parse error: {:?}", e))?;

    let mut contener: SaveDataContainer = SaveDataContainer::default();

    let get_field = |value: &ron::Value, key: &str| -> Option<ron::Value> {
        let ron::Value::Map(map) = value else { return None; };
        map.iter()
            .find(|(k, _)| *k == &ron::Value::String(key.to_string()))
            .map(|(_, v)| v.clone())
    };

    if let Some(prestige) = get_field(&parsed, "prestige") {
        if let Some(ron::Value::Number(ron::value::Number::Integer(int_val))) = get_field(&prestige, "sunlit_nursery") {
            contener.prestige.sunlit_nursery = int_val as usize;
        };
    };

    if let Ok(full_container) = ron::de::from_bytes::<SaveDataContainer>(&decrypted_bytes) {
        contener.eco_storege = full_container.eco_storege;
    };

    if let Ok(full_container) = ron::de::from_bytes::<SaveDataContainer>(&decrypted_bytes) {
        contener.up_storege = full_container.up_storege;
    };

    if let Ok(full_container) = ron::de::from_bytes::<SaveDataContainer>(&decrypted_bytes) {
        contener.iti_storege = full_container.iti_storege;
    };

    if let Ok(full_container) = ron::de::from_bytes::<SaveDataContainer>(&decrypted_bytes) {
        contener.global_storege = full_container.global_storege;
    };

    if let Ok(full_container) = ron::de::from_bytes::<SaveDataContainer>(&decrypted_bytes) {
        contener.world = full_container.world;
    };

    Ok(contener)
}

fn fix_upgrade_references(inv: &mut UpgradeStorege) {
    let mut clean_reference = UpgradeStorege::default();

    for (_, loaded_upgrade) in inv.sparcks.iter() {
        let Some((_, upgrade)) = clean_reference.sparcks
            .iter_mut() 
            .find(|(_, u)| u.id == loaded_upgrade.id) else { continue; };

            upgrade.current_level = loaded_upgrade.current_level;
            upgrade.texture_stage = loaded_upgrade.texture_stage;
    };

    for (_, loaded_upgrade) in inv.global.iter() {
        let Some((_, upgrade)) = clean_reference.global
            .iter_mut() 
            .find(|(_, u)| u.id == loaded_upgrade.id) else { continue; };

            upgrade.current_level = loaded_upgrade.current_level;
            upgrade.texture_stage = loaded_upgrade.texture_stage;
    };

    for (_, loaded_upgrade) in inv.sunlit_nursery.iter() {
        let Some((_, upgrade)) = clean_reference.sunlit_nursery
            .iter_mut() 
            .find(|(_, u)| u.id == loaded_upgrade.id) else { continue; };

            upgrade.current_level = loaded_upgrade.current_level;
            upgrade.texture_stage = loaded_upgrade.texture_stage;
    };

    inv.sparcks = clean_reference.sparcks;
    inv.global = clean_reference.global;
    inv.sunlit_nursery = clean_reference.sunlit_nursery;
}

fn get_plant_static_price(plant: TypePlant) -> &'static [f64] {
    match plant {
        TypePlant::Tomato => PL_TOMATO.price,
        TypePlant::Cucumber => PL_CUCUMBER.price,
        TypePlant::Corn => PL_CORN.price,
        TypePlant::Pumpkin => PL_PUMPKIN.price,
    }
}

fn fix_economy_references(eco: &mut Economy) {
    let mut clean_reference = Economy::default();

    for (res_type_loaded, count_loaded) in eco.vault.iter() {
        clean_reference.vault.insert(*res_type_loaded, *count_loaded);
    };

    *eco = clean_reference;
}

fn fix_inventory_references(inv: &mut GlobalInventory) {
    for i in 0..16 {
        let Some(slot_state) = inv.sunlit_nursery_inv.get_mut(&i) else { continue; };

        if let SlotState::Occupied(plant) = slot_state {
            plant.price = get_plant_static_price(plant.species_id);
        };
    };
}

fn fix_iti_inventory(
    iti_storege: &mut ItemTypeInfo
) {
    let mut clean_reference = ItemTypeInfo::default();

    for (type_plant_loaded, plant_ability_map_loaded) in iti_storege.sn_plant_ability.iter() {
        let Some(plant_ability_map) = clean_reference.sn_plant_ability.get_mut(type_plant_loaded) else { continue; };

        for (plant_ability_id_loaded, plant_ability_case_loaded) in plant_ability_map_loaded.iter() {
            let Some(plant_ability_case) = plant_ability_map.get_mut(plant_ability_id_loaded) else { continue; };

            for (i, plant_ability_value_loaded) in plant_ability_case_loaded.iter().enumerate() {
                let Some(plant_ability_value) = plant_ability_case.get_mut(i) else { continue; };

                *plant_ability_value = *plant_ability_value_loaded;
            };
        };
    };
    for (type_loaded, count_loaded) in iti_storege.item_count_inv.iter() {
        let Some(count) = clean_reference.item_count_inv.get_mut(type_loaded) else { continue; };

        *count = *count_loaded;
    };

    *iti_storege = clean_reference;
}


pub fn final_load_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut up_storege: ResMut<UpgradeStorege>,
    mut global_storege: ResMut<GlobalInventory>,
    mut eco_storege: ResMut<Economy>,
    mut iti_storege: ResMut<ItemTypeInfo>,
    mut prestige: ResMut<PrestigeRoom>,
    mut world: ResMut<NextState<CurrentWorld>>,
    save_slot_inv: Res<SaveSlotInv>,
) {
    let Some(i) = save_slot_inv.active_slot else { return; };

    let Ok(mut contener) = load_game_from_disk(i) else { info!("Error open save"); return; };

    fix_upgrade_references(&mut contener.up_storege);

    fix_inventory_references(&mut contener.global_storege);

    fix_iti_inventory(&mut contener.iti_storege);

    fix_economy_references(&mut contener.eco_storege);

    *up_storege = contener.up_storege;
    *global_storege = contener.global_storege;
    *eco_storege = contener.eco_storege;
    *iti_storege = contener.iti_storege;
    *prestige = contener.prestige;
    world.set(contener.world);

    next_state.set(GameState::Playing);
}

pub fn setting_save(
    window: &mut Window,
    settings: &mut GlobalSettings,
    path: PathBuf
) -> Result<()>{
    let mut file = File::create(path)?;

    let byte = ron::to_string(settings)?.into_bytes();
    file.write_all(&byte)?;

    monitor_window_settings(window, settings,);

    Ok(())
}

fn setting_load(path: PathBuf) -> Result<GlobalSettings, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    let new_settings = ron::de::from_bytes(&bytes)?;

    Ok(new_settings)
}

pub fn get_setting_path() -> PathBuf {
    let file_name = format!("setting.dat");

    if let Some(proj_dirs) = ProjectDirs::from("com", "Honami", "CatAndPlants") {
        let save_dir = proj_dirs.data_dir();

        let _ = create_dir_all(save_dir);

        return save_dir.join(file_name);
    }

    PathBuf::from(file_name)
}

pub fn save_setting_maneger(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<GlobalSettings>,
) {
    let path = get_setting_path();

    *settings = if let Ok(load) = setting_load(path) { load } else { GlobalSettings::default() };

    let Ok(mut window) = window_query.single_mut() else { return; };
    
    monitor_window_settings(&mut window, &mut settings,);
}