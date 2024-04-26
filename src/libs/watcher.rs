// use std::path::PathBuf;

// /// Contains path information needed to watch directory.
// ///
// /// Watcher is a struct design to hold a path, as well as an enum that contains the necessary information/tools required
// /// in order to watch a directory for changes being made to it.
// ///
// #[derive(Debug, Clone)]
// pub struct Watcher {
//     mode: WatchMode,
//     watching: bool,
//     watch_dir: PathBuf,
// }

// impl Watcher {
//     /// Creates a new Watcher struct.
//     ///
//     /// The point of a watcher struct as see at [`Watcher`] is to contain path information and perform actions based on its
//     /// [`WatchMode`]
//     ///`
//     ///
//     pub fn new(mode: Option<WatchMode>, watching: bool, watch_dir: PathBuf) -> Self {
//         Watcher {
//             mode: match mode {
//                 Some(mode) => mode,
//                 None => WatchMode::Default,
//             },
//             watching,
//             watch_dir,
//         }
//     }
// }

// /// Will be used to define multiple types of watchers with differing functionality
// #[derive(Debug, Clone)]
// pub enum WatchMode {
//     /// The default mode for Watcher structs
//     ///
//     ///
//     Default,
// }
