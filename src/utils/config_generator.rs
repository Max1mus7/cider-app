// use crate::utils::config::TopLevelConfiguration;
// use std::fs::File;

// struct OutputConfig {
//     pub configuration: TopLevelConfiguration,
// }

// // struct json_output_config {

// // }

// trait OutputFile {
//     fn default(&self) -> File;
// }

// impl OutputFile for OutputConfig {
//     fn default(&self) -> File {
//         File::create(self.configuration.s_config.get_output()).unwrap_or_else(|err| {
//             eprintln!("No directory found at that location. {}", err);
//             panic!("No directory found at that location. {}", err);
//         })
//     }
// }
