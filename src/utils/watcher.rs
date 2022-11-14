use std::path::{PathBuf};

pub struct Watcher {
    mode: WatchMode,
    watching: bool,
    watch_dir: PathBuf 

}

impl Watcher {
    pub fn new(mode: Option<WatchMode>, watching: bool, watch_dir: PathBuf) -> Self {
        Watcher { mode: WatchMode::Default, watching, watch_dir }
    }
}

/// Will be used to define multiple types of watchers with differing functionality
pub enum WatchMode {
    Default
}

