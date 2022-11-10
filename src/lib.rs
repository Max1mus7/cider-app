pub mod utils;
pub use utils::config;
pub use utils::config_generator;
pub use utils::executor;
pub use utils::parsing;

#[cfg(test)]
mod systests {
    use crate::parsing::JsonParser;
    use log::info;

    #[test]
    fn parse_json_from_file() {
        info!(
            "{}",
            JsonParser::parse_json_string("example_docker_config.json")
        );
        assert!(true);
    }

    #[test]
    fn test_parse_top_level_actions() {
        let config = JsonParser::new_top_level("example_docker_config.json");
        for action in config.get_actions() {
            info!("{:#?}", action);
        }
        assert!(true);
    }

    #[test]
    fn test_parse_pipeline_actions() {
        let config = JsonParser::new_top_level("example_docker_config.json");
        for pipeline in config.get_pipelines() {
            for action in pipeline.get_pipeline_config().get_actions() {
                info!("{:#?}", action);
            }
        }
    }

    #[test]
    fn test_all_actions() {
        let config = JsonParser::new_top_level("example_docker_config.json");
        for action in config.get_all_actions() {
            info!("{:#?}", action);
        }
    }

    #[test]
    fn test_parse_pipeline() {
        let config = JsonParser::new_top_level("example_docker_config.json");
        for pipeline in config.get_pipelines() {
            info!("{:#?}", pipeline);
        }
    }
}
