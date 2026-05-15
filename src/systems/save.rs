use std::{fs::{File, create_dir_all}, io::{Read, Write}, path::PathBuf, time::Duration};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce, Key};
use crate::schema::{save_file::*, types_and_states::*};
use crate::content::world::sunlit_nursery::*;
use crate::systems::simulation::monitor_window_settings;
use bevy::{prelude::*, window::{WindowFocused, PrimaryWindow}, app::AppExit};
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
    cit_storege: Res<CountItemType>,
    save_slot_inv: Res<SaveSlotInv>,
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

    auto_save_system(&up_storege, &global_storege, &eco_storege, &cit_storege, &save_slot_inv, &world);

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
    cit_storege: &CountItemType,
    save_slot_inv: &SaveSlotInv,
    world: &State<CurrentWorld>,
) {
    let contener = SaveDataContainer {
        up_storege: up_storege.clone(),
        global_storege: global_storege.clone(),
        eco_storege: eco_storege.clone(),
        cit_storege: cit_storege.clone(),
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

    let contener: SaveDataContainer = ron::de::from_bytes(&decrypted_bytes)?;

    Ok(contener)
}

fn fix_upgrade_references(inv: &mut UpgradeStorege) {
    let clean_reference = UpgradeStorege::default();

    for (coords, clean_upgrade) in clean_reference.global.iter() {
        inv.global.entry(*coords).or_insert_with(|| clean_upgrade.clone());
    };

    for (coords, clean_upgrade) in clean_reference.sunlit_nursery.iter() {
        inv.sunlit_nursery.entry(*coords).or_insert_with(|| clean_upgrade.clone());
    };

    for (coords, loaded_upgrade) in inv.global.iter_mut() {
        if let Some(clean_upgrade) = clean_reference.global.get(coords) {
            loaded_upgrade.name = clean_upgrade.name;
            loaded_upgrade.description = clean_upgrade.description;
            loaded_upgrade.levels = clean_upgrade.levels;
            loaded_upgrade.dependencies = clean_upgrade.dependencies;
        };
    };

    for (coords, loaded_upgrade) in inv.sunlit_nursery.iter_mut() {
        if let Some(clean_upgrade) = clean_reference.sunlit_nursery.get(coords) {
            loaded_upgrade.name = clean_upgrade.name;
            loaded_upgrade.description = clean_upgrade.description;
            loaded_upgrade.levels = clean_upgrade.levels;
            loaded_upgrade.dependencies = clean_upgrade.dependencies;
        };
    };
}

fn get_plant_static_price(plant: TypePlant) -> &'static [f64] {
    match plant {
        TypePlant::Tomato => PL_TOMATO.price,
        TypePlant::Cucumber => PL_CUCUMBER.price,
        TypePlant::Corn => PL_CORN.price,
        TypePlant::Pumpkin => PL_PUMPKIN.price,
    }
}

fn fix_inventory_references(inv: &mut GlobalInventory) {
    for slot_state in inv.sunlit_nursery_inv.iter_mut() {
        if let SlotState::Occupied(plant) = slot_state {
            plant.price = get_plant_static_price(plant.species_id);
        };
    };
}

pub fn final_load_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut up_storege: ResMut<UpgradeStorege>,
    mut global_storege: ResMut<GlobalInventory>,
    mut eco_storege: ResMut<Economy>,
    mut cit_storege: ResMut<CountItemType>,
    mut world: ResMut<NextState<CurrentWorld>>,
    save_slot_inv: Res<SaveSlotInv>,
) {
    let Some(i) = save_slot_inv.active_slot else { return; };

    let Ok(mut contener) = load_game_from_disk(i) else { info!("Error open save"); return; };

    fix_upgrade_references(&mut contener.up_storege);

    fix_inventory_references(&mut contener.global_storege);

    *up_storege = contener.up_storege;
    *global_storege = contener.global_storege;
    *eco_storege = contener.eco_storege;
    *cit_storege = contener.cit_storege;
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