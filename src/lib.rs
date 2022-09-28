pub mod config {
    use std::collections::HashMap;
    use log::{info,warn, error};
    use super::parsing::Parser;

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct ShareableConfiguration {
        //metadata not required at runtime
        //defaulted to None
        metadata: Option<HashMap<String, String>>,

        //title not required at runtime
        //defaulted to None
        title: Option<String>,

        //tags not required at runtime
        //defaulted to None
        tags: Option<HashMap<String, String>>,

        //language required at runtime, so it is non-optional
        //defaulted to bash
        language: String,

        //image not required at runtime
        //defaulted to None
        //if "docker" is specified as a backend, this will default to ubuntu:latest
        //IMAGE IS A DOCKER-SPECIFIC FEATURE. IF BACKEND IS NOT DOCKER, IMAGE SHOULD BE NONE
        image: Option<String>,

        //backend required at runtime, so it is non-optional
        //defaulted to local(Windows in this case)
        //TODO: upon implementing Docker functionality, make this default to Docker 
        backend: String,

        //Output directory required at runtime, so it is not optional
        //defaulted to dist/cider/
        output: String,

        //Source directory required at runtime, so it is not optional
        //defaulted to ./src
        source: String
    }
    impl ShareableConfiguration {

        pub fn new(metadata: Option<HashMap<String, String>>, title: Option<String>, tags: Option<HashMap<String, String>>, language: String, 
        image: Option<String>, backend: String, output: String, source: String) -> Self {
            let image = {
                if !backend.to_lowercase().eq("docker") {
                    warn!("Image cannot be set if docker is not the backend.");
                    None
                }
                else {
                    image
                }
            };
            Self { metadata, title, tags, language, image, backend, output, source }
        }

        pub fn get_metadata(&self) -> Result<&HashMap<String, String>, &'static str> {
            match &self.metadata {
                Some(metadata) => {
                    info!("Metadata successfully retrieved: {:#?}", &metadata);
                    Ok(metadata)
                },
                None => {
                    let res_str = "No metadata value found or no metadata value configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }

        pub fn set_metadata(&mut self, new_metadata: HashMap<String, String>) -> () {
            info!("New metadata set: {:#?}", new_metadata);
            self.metadata = Some(new_metadata);
        }

        pub fn get_title(&self) -> Result<&str, &'static str> {
            match &self.title {
                Some(title) => {
                    info!("Title successfully retrieved: {:?}", &title);
                    Ok(&title)
                },
                None => {
                    let res_str = "No title value found or no title value configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }

        pub fn set_title(&mut self, new_title: String) -> () {
            info!("New title set: {}", new_title);
            self.title = Some(new_title);
        }

        pub fn get_tags(&self) -> Result<&HashMap<String, String>, &'static str> {
            match &self.tags {
                Some(tags) => {
                    info!("Tags successfully retrieved: {:?}", &tags);
                    Ok(tags)
                },
                None => {
                    let res_str = "No tags found or no tags configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }

        pub fn set_tags(&mut self, new_tags: HashMap<String, String>) -> () {
            self.tags = Some(new_tags);
        }

        pub fn get_language(&self) -> &str {
            &self.language
        }
        pub fn set_language(&mut self, new_language: String) -> () {
            info!("New language set: {}", new_language);
            self.language = new_language;
        }

        pub fn get_image(&self) -> Result<&str, &'static str> {
            match &self.image {
                Some(image) => {
                    info!("Image successfully retrieved: {:?}", &image);
                    Ok(image)
                },
                None => {
                    let res_str = "No image found or no image configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }
        pub fn set_image(&mut self, new_image: String) -> () {
            if !self.get_backend().to_lowercase().eq("docker") {
                warn!("image can only be set on configurations with a docker backend");
                self.image = None
            }
            info!("New title set: {}", new_image);
            self.image = Some(new_image);
        }

        pub fn get_backend(&self) -> &str {
            &self.backend
        }
        pub fn set_backend(&mut self, new_backend: String) -> () {
            info!("New backend set: {}", new_backend);
            self.backend = new_backend;
        }

        

        pub fn get_output(&self) -> &str {
            info!("Output directory successfully retrieved: {:?}", &self.output);
            &self.output
        }
        pub fn set_output(&mut self, new_output: String) -> () {
            info!("New output directory set: {}", new_output);
            self.output = new_output;
        }

        pub fn get_source(&self) -> Result<&str, &'static str> {
            info!("Source directory successfully retrieved: {:?}", &self.source);
            Ok(&self.source)
        }
        pub fn set_source(&mut self, new_source: String) -> () {
            info!("New source directory set: {}", new_source);
            self.backend = new_source;
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct TopLevelConfiguration {
        //ShareableConfiguration data required to perform top-level tasks
        s_config: ShareableConfiguration,

        //pipeline definitions required at runtime, even if it is an empty Vector
        //defaulted to an empty vector
        pipeline_defs: Vec<String>,
        
        //No pipelines required at runtime, but Vector will exist prepared.
        pipelines: Vec<Pipeline>,

        //Top-level action definitions not required at runtime
        //defaulted to empty Vector
        action_defs: Vec<String>,

        //Top-level actions not required for a TopLevelConfiguration implementation to be valid
        //defaulted to empty Vectory
        actions: Vec<Action>
        
    }
    impl TopLevelConfiguration {

        // pub fn new(filename: String) -> Self {
        //     Parser::parse_new(filename)
        // }

        pub fn get_pipeline_defs(&self) -> &Vec<String> {
            info!("Pipelines successfully retrieved from configuration: {:#?}", &self.pipeline_defs);
            &self.pipeline_defs
        }
        pub fn set_pipeline_defs(&mut self, new_pipeline_defs: Vec<String>) -> () {
            info!("New pipeline definitions set: {:#?}", new_pipeline_defs);
            self.pipeline_defs = new_pipeline_defs;
        }

        pub fn get_pipelines(&self) -> &Vec<Pipeline> {
            info!("Pipelines successfully retrieved: {:#?}", &self.pipelines);
            &self.pipelines
        }
        pub fn set_pipelines(&mut self, new_pipelines: Vec<Pipeline>) -> () {
            info!("New pipelines set: {:#?}", new_pipelines);
            self.pipelines = new_pipelines;
        }

        pub fn get_action_defs(&self) -> &Vec<String> {
            info!("Actions successfully retrieved from configuration: {:#?}", &self.action_defs);
            &self.action_defs
        }
        pub fn set_action_defs(&mut self, new_action_defs: Vec<String>) -> () {
            info!("New action definitions set: {:#?}", new_action_defs);
            self.action_defs = new_action_defs;
        }

        pub fn get_actions(&self) -> &Vec<Action> {
            info!("Actions successfully retrieved: {:#?}", &self.actions);
            &self.actions
        }
        pub fn set_actions(&mut self, new_actions: Vec<Action>) -> () {
            info!("New actions set: {:#?}", new_actions);
            self.actions = new_actions;
        }

    }
    

    //holds action-specific configuration information
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Action {
        shared_config: ShareableConfiguration,
        action_config: ActionConfig
    }
    impl Action {
        pub fn new(shared_config: ShareableConfiguration, action_config: ActionConfig) -> Action {
            Action { shared_config , action_config }
        }

        pub fn get_config(&self) -> &ShareableConfiguration {
            &self.shared_config
        }
        pub fn set_config(&mut self, new_shared_config: ShareableConfiguration)-> () {
            self.shared_config = new_shared_config;
        }

        pub fn get_action_config(&self) -> &ActionConfig {
            &self.action_config
        }
        pub fn set_action_config(&mut self, action_config: ActionConfig) -> () {
            self.action_config = action_config;
        }
    }
    
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct ActionConfig {
        //Not required at runtime, can be None
        //default = None
        conditions: Option<Vec<Condition>>,
        //required for runtime, will be defaulted if not included
        //default = 0
        retries: i8,
        //required for runtime, will be defaulted if not included
        //default = false
        allowed_failure: bool,
        //required for runtime, no defaults included other than in default method
        //Note: NOT DEFAULTED IN new() METHOD
        //default = Step {"step1": "echo \"hello world!\""}
        manual: Vec<Step>
    }
    impl ActionConfig {
        fn new(conditions: Option<Vec<Condition>>, retries: Option<i8>, allowed_failure: Option<bool>, manual: Vec<Step>) -> Self {
            let retries = match retries {
                Some(retries) => {
                    retries
                }
                None => {
                    0
                }
            };

            let allowed_failure = match allowed_failure {
                Some(allowed_failure) => {
                    allowed_failure
                }
                None => {
                    false
                }
            };

            ActionConfig { conditions, retries, allowed_failure, manual }
        }

        fn get_conditions(&self) -> Result<&Vec<Condition>, &'static str> {
            match &self.conditions {
                Some(conditions) => {
                    info!("Conditions successfully retrieved: {:#?}", &conditions);
                    Ok(conditions)
                },
                None => {
                    let res_str = "No conditions found or no conditions configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }
        fn set_conditions(&mut self, new_conditions: Vec<Condition>) -> () {
            info!("New conditions set: {:#?}", new_conditions);
            self.conditions = Some(new_conditions);
        }

        fn get_retries(&self) -> &i8 {
            info!("Retry count successfully acquired: {} ", &self.retries);
            &self.retries
        }
        fn set_retries(&mut self, new_retries: i8) -> () {
            info!("New retry count set: {:?}", &new_retries);
            self.retries = new_retries
        }

        fn get_allowed_failure(&self) -> &bool {
            info!("Failure allowance successfully acquired: {} ", &self.allowed_failure);
            &self.allowed_failure
        }
        fn set_allowed_failure(&mut self, new_allowed_failure: bool) -> () {
            info!("New failure allowance set: {:?}", &new_allowed_failure);
            self.allowed_failure = new_allowed_failure;
        }

        fn get_manual(&self) -> &Vec<Step> {
            info!("Manual successfully retrieved: {:#?}", &self.manual);
            &self.manual
        }
        fn set_manual(&mut self, new_manual: Vec<Step>) -> () {
            info!("New manual set: {:#?}", new_manual);
            self.manual = new_manual;
        }
    }
    
    //Holds information with conditions that will resolve to either true or false
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Condition {
        //A name is necessary for a condition to exist.
        //There cannot be a default name for a condition, as it would be meaningless
        name: String,
        //A condition is also necessary for a condition to exist.
        //There cannot be a default condition for a condition struct, as it would be forced to default to true
        condition: String
    }
    impl Condition {
        fn new(name: String, condition: String) -> Condition {
            Condition { name, condition }
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_condition(&self) -> &str {
            &self.condition
        }

        fn update_condition(&mut self, name: String, condition: String) -> () {
            self.name = name;
            self.condition = condition;
        }
    }


    //Holds hashmap information with data necessary to run scripts
    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Step {
        name: String,
        script: String
    }
    impl Step {
        fn new(name: String, script: String) -> Self {
            Self {name, script}
        }
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_script(&self) -> &str {
            &self.script
        }
        fn update_script(&mut self, name: String, script: String) -> () {
            self.name = name;
            self.script = script;
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Pipeline {
        shared_config: ShareableConfiguration,
        pipeline_config: PipelineConfig
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct PipelineConfig { 
        //Not required at runtime, can be None
        //default = None
        conditions: Option<Vec<Condition>>,

        //A pipeline must contain one or more actions, and those actions must be defined
        //default = no default, required argument.
        action_defs: Vec<String>,

        //A pipeline must contain one or more actions
        //default = no default, required argument
        actions: Vec<Action>,

        //allows a Pipeline object to keep track of whether or not it has run
        //default = false
        has_run: bool,

        //A pipeline should be able to wait until other pipelines have executed before it attempts to run
        //default = empty Vector
        requires: Vec<String>

    }

}

pub mod parsing {
    use std::{fs, collections::HashMap};
    use std::error::Error;
    use json::JsonValue;
    use log::{info, warn, error};

    use super::config::TopLevelConfiguration;

    pub struct Parser {

    }

    impl Parser {

        fn parse_json_map(json: JsonValue) -> Option<HashMap<String, String>>{
            println!("{:#?}", json);
            let mut map = HashMap::new();
            for key_value in json.entries() {
                map.insert(key_value.0.to_string(), key_value.1.to_string());
            }
            println!("{:#?}", json);
            if map.is_empty() {
                warn!("No mappable values found in {:#?}", json);
                return None;
            }
            Some(map)
        } 

        fn parse_json_vector(json: JsonValue) -> Option<Vec<String>> {
            println!("{:#?}", json);
            let mut vec = vec![];
            for value in json.members() {
                vec.push(value.to_string())
            }
            println!("{:#?}", json);
            if vec.is_empty() {
                warn!("No mappable values found in {:#?}", json);
                return None;
            }
            Some(vec)
        }

        // pub fn parse_overwrite(mut config: Configuration, filename: String) -> Result<Configuration, Box<dyn Error>> {
        //     let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        //         eprintln!("{}", err);
        //         error!("There was an error locating your configuration file: {}", err);
        //         panic!("{}",err.to_string());
        //     });
        //     let parsed = json::parse(&contents).unwrap_or_else(|err| {
        //         eprintln!();
        //         error!("There was an error parsing your configuration file: {}", err);
        //         panic!("{}", err.to_string());
        //     });
        //     info!("'backend' tag from example_docker_file.json: {}", &parsed["backend"]);
        //     config.set_metadata(Self::parse_json_map(parsed["metadata"].clone()).unwrap());
        //     config.set_title(parsed["title"].clone().to_string());
        //     config.set_tags(Self::parse_json_map(parsed["tags"].clone()).unwrap());
        //     config.set_language(parsed["language"].clone().to_string());
        //     config.set_image(parsed["image"].clone().to_string());
        //     config.set_backend(parsed["backend"].clone().to_string());
        //     config.set_actions(Self::parse_json_vector(parsed["actions"].clone()).unwrap());
        //     config.set_output(parsed["output_directory"].clone().to_string());
        //     config.set_source(parsed["source_directory"].clone().to_string());
        //     Ok(config)
        // }


        //Created strictly for testing purposes.
        pub fn parse_json_string(filename: &str) -> JsonValue {
            let contents = fs::read_to_string(filename).unwrap();
            let parsed = json::parse(&contents);
            println!("{:#?}", parsed.as_ref().unwrap().clone());
            parsed.unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{config::*, parsing::*};
    use log::info;
    // #[test]
    // fn print_file_name(){
    //     let config = Configuration::new(None).unwrap();
    //     println!("{:#?}", config);
    //     assert!(true);
    // }

    // #[test]
    // fn parse_string() {
    //     info!("{}", Parser::parse_json_string("example_docker_config.json"));
    //     assert!(true);
    // }

    // fn test_for_string_equality() {
    //     let config = Configuration::default();
    //     info!("{:#?}, {}, Equal? {:#?}", config.get_backend(), "bash", config.get_backend().eq("bash"));
    //     assert!(config.get_backend().eq("bash"));
    // }

    // #[test]
    // fn ensure_return_equality() {
    //     let filename = "example_docker_config.json".to_string();
    //     let conf1 = Configuration::new(Some(filename.clone())).unwrap();
    //     let conf2 = Configuration::new(None).unwrap();
    //     let conf2 = Parser::parse_overwrite(conf2, filename).unwrap();
    //     assert!(conf1 == conf2);
    // }
}