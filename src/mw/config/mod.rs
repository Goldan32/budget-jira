use dotenv::dotenv;
use rand::distr::{Alphanumeric, SampleString};
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

#[derive(Debug)]
pub struct AppConfig {
    pub root_dir: PathBuf,
    pub task_db: PathBuf,
    pub work_dir: PathBuf,
}

// TODO: Add some Arc magic to make this a "singleton"
impl AppConfig {
    fn global() -> &'static OnceLock<Arc<AppConfig>> {
        static INSTANCE: OnceLock<Arc<AppConfig>> = OnceLock::new();
        &INSTANCE
    }

    pub fn init() {
        dotenv().ok();
        let root_dir: PathBuf = if let Ok(var) = std::env::var("BJL_ROOT") {
            var.into()
        } else if let Some(dir) = dirs::data_local_dir() {
            dir
        } else {
            // Try to not collide with other users
            let unique_string = Alphanumeric.sample_string(&mut rand::rng(), 6);
            // Assume windows #justworks
            let mut root_dir = PathBuf::from(r"/var/lib/bjl");
            root_dir.push(unique_string);
            root_dir
        };

        let work_dir: PathBuf = if let Ok(var) = std::env::var("BJL_CACHE") {
            var.into()
        } else if let Some(dir) = dirs::cache_dir() {
            let mut work_dir = dir;
            work_dir.push("bjl");
            work_dir
        } else {
            let mut work_dir = PathBuf::from(r"/var/cache/bjl");
            let unique_string = Alphanumeric.sample_string(&mut rand::rng(), 6);
            work_dir.push(unique_string);
            work_dir
        };

        let mut task_db = root_dir.clone();
        task_db.push("task_db.db3");

        Self::global()
            .set(Arc::new(Self {
                root_dir,
                task_db,
                work_dir,
            }))
            .expect("Already initialized");
    }

    pub fn get() -> Arc<AppConfig> {
        Self::global()
            .get()
            .expect("Config not initialized")
            .clone()
    }
}
