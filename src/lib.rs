#![deny(missing_docs)]
#![warn(
    rustdoc::missing_doc_code_examples,
    dead_code,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::style
)]

//! Contains all necessary functions.
//!
/// Contains functions that allow CIder to create docker images, parse JSON, and more.
pub mod utils;
pub use utils::config;
pub use utils::config_generator;
pub use utils::executor;
pub use utils::parsing;
// pub use utils::watcher;

#[cfg(test)]
mod systests {
    use crate::parsing::json_parser;
    use log::info;

    #[test]
    fn parse_json_from_file() {
        info!("{}", json_parser::parse_json_string("cider_config.json"));
        assert!(true);
    }

    #[test]
    fn test_parse_top_level_actions() {
        let config = json_parser::new_top_level("cider_config.json");
        for action in config.get_actions() {
            info!("{:#?}", action);
        }
        assert!(true);
    }

    #[test]
    fn test_parse_pipeline_actions() {
        let config = json_parser::new_top_level("cider_config.json");
        for pipeline in config.get_pipelines() {
            for action in pipeline.pipeline_config.get_actions() {
                info!("{:#?}", action);
            }
        }
    }

    #[test]
    fn test_all_actions() {
        let config = json_parser::new_top_level("cider_config.json");
        for action in config.get_all_actions() {
            info!("{:#?}", action);
        }
    }

    #[test]
    fn test_parse_pipeline() {
        let config = json_parser::new_top_level("cider_config.json");
        for pipeline in config.get_pipelines() {
            info!("{:#?}", pipeline);
        }
    }
}
