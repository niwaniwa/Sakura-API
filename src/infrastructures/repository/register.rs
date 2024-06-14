use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::domain::repository::register::RegisterRepository;

// Static HashMap with RwLock for thread-safe access
static HASHMAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

// Static flag for register mode
static IS_REGISTER_MODE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

// Mutex for synchronization
static RESET_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn card_insert(key: String, value: String, ttl: u64) {
    let mut map = HASHMAP.write().unwrap();
    map.insert(key.clone(), value);

    let key_clone = key.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(ttl)).await;
        let _guard = RESET_MUTEX.lock().await;
        let mut map = HASHMAP.write().unwrap();
        map.remove(&key_clone);
    });
}

fn card_get(key: &str) -> Option<String> {
    let map = HASHMAP.read().unwrap();
    map.get(key).cloned()
}

fn set_register_mode(value: bool, ttl: u64) {
    IS_REGISTER_MODE.store(value, Ordering::SeqCst);

    if value {
        tokio::spawn(async move {
            sleep(Duration::from_secs(ttl)).await;
            let _guard = RESET_MUTEX.lock().await;
            IS_REGISTER_MODE.store(false, Ordering::SeqCst);
        });
    }
}

pub struct RegisterRepositoryImpl {}

impl RegisterRepository for RegisterRepositoryImpl {
    fn register(&self) {
        set_register_mode(true, 60);
    }

    fn get_card(&self) -> String {
        if let Some(value) = card_get("card") {
            value
        } else {
            "".to_string()
        }
    }

    fn is_register_mode(&self) -> bool {
        IS_REGISTER_MODE.load(Ordering::SeqCst)
    }

    fn add_card(&self, card: String) {
        card_insert("card".to_string(), card, 60);
    }
}
