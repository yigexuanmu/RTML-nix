use std::sync::LazyLock;
use std::sync::Mutex;

const MACHINE_ID_FILE: &str = "machine-id";

static INITIALIZED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

fn ensure_init() {
    let mut init = INITIALIZED.lock().unwrap();
    if !*init {
        let dir = dirs_next::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("RTML");
        std::fs::create_dir_all(&dir).ok();
        terracotta::init_lib(dir.join(MACHINE_ID_FILE));
        *init = true;
    }
}

pub fn get_state() -> serde_json::Value {
    ensure_init();
    terracotta::controller::get_state()
}

pub fn start_host(player_name: &str) {
    ensure_init();
    terracotta::controller::set_scanning(None, Some(player_name.to_owned()), Vec::new());
}

pub fn start_join(room_code: &str) -> bool {
    ensure_init();
    let room = match terracotta::rooms::Room::from(room_code) {
        Some(r) => r,
        None => return false,
    };
    terracotta::controller::set_guesting(room, None, Vec::new())
}

pub fn stop() {
    ensure_init();
    terracotta::controller::set_waiting();
}