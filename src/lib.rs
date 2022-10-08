pub mod executor;
pub mod config {
    use std::collections::HashMap;
    use log::{info,warn};

    #[derive(Debug, Clone)]
    #[derive(PartialEq, Eq)]
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
                } else {
                    image
                }
            };
            Self { metadata, title, tags, language, image, backend, output, source }
        }

        pub fn get_metadata(&self) -> Option<HashMap<String, String>> {
            match &self.metadata {
                Some(metadata) => {
                    info!("Metadata successfully retrieved: {:#?}", &metadata);
                    Some(metadata.to_owned())
                },
                None => {
                    let res_str = "No metadata value found or no metadata value configured.";
                    warn!("{}", res_str);
                    None
                }
            }
        }

        pub fn set_metadata(&mut self, new_metadata: HashMap<String, String>) {
            info!("New metadata set: {:#?}", new_metadata);
            self.metadata = Some(new_metadata);
        }

        pub fn get_title(&self) -> Option<String> {
            match &self.title {
                Some(title) => {
                    info!("Title successfully retrieved: {:?}", &title);
                    Some(title.to_string())
                },
                None => {
                    let res_str = "No title value found or no title value configured.";
                    warn!("{}", res_str);
                    None
                }
            }
        }

        pub fn set_title(&mut self, new_title: String)  {
            info!("New title set: {}", new_title);
            self.title = Some(new_title);
        }

        pub fn get_tags(&self) -> Option<HashMap<String, String>> {
            match &self.tags {
                Some(tags) => {
                    info!("Tags successfully retrieved: {:?}", &tags);
                    Some(tags.to_owned())
                },
                None => {
                    let res_str = "No tags found or no tags configured.";
                    warn!("{}", res_str);
                    None
                }
            }
        }

        pub fn set_tags(&mut self, new_tags: HashMap<String, String>)  {
            self.tags = Some(new_tags);
        }

        pub fn get_language(&self) -> &str {
            &self.language
        }
        pub fn set_language(&mut self, new_language: String)  {
            info!("New language set: {}", new_language);
            self.language = new_language;
        }

        pub fn get_image(&self) -> Option<String> {
            match &self.image {
                Some(image) => {
                    info!("Image successfully retrieved: {:?}", &image);
                    Some(image.to_string())
                },
                None => {
                    let res_str = "No image found or no image configured.";
                    warn!("{}", res_str);
                    None
                }
            }
        }
        pub fn set_image(&mut self, new_image: String)  {
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
        pub fn set_backend(&mut self, new_backend: String)  {
            info!("New backend set: {}", new_backend);
            self.backend = new_backend;
        }

        

        pub fn get_output(&self) -> &str {
            info!("Output directory successfully retrieved: {:?}", &self.output);
            &self.output
        }
        pub fn set_output(&mut self, new_output: String)  {
            info!("New output directory set: {}", new_output);
            self.output = new_output;
        }

        pub fn get_source(&self) -> &str {
            info!("Source directory successfully retrieved: {:?}", &self.source);
            &self.source
        }
        pub fn set_source(&mut self, new_source: String)  {
            info!("New source directory set: {}", new_source);
            self.backend = new_source;
        }
    }
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct TopLevelConfiguration {
        //ShareableConfiguration data required to perform top-level tasks
        pub s_config: ShareableConfiguration,

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

        pub fn new(s_config: ShareableConfiguration, pipeline_defs: Vec<String>, pipelines: Vec<Pipeline>, action_defs: Vec<String>, actions: Vec<Action>) -> Self {
            Self { s_config, pipeline_defs, pipelines, action_defs, actions }
        }

        pub fn get_shared_config(&self) -> &ShareableConfiguration {
            // info!("Shareable configuration successfully retrieved from top-level configuration: \n{:#?}", &self.s_config);
            &self.s_config
        }
        pub fn set_shared_config(&mut self, new_s_config: ShareableConfiguration)  {
            info!("New shareable configuration set: \n{:#?}", new_s_config);
            self.s_config = new_s_config;
        }

        pub fn get_pipeline_defs(&self) -> &Vec<String> {
            info!("Pipelines successfully retrieved from configuration: {:#?}", &self.pipeline_defs);
            &self.pipeline_defs
        }
        pub fn set_pipeline_defs(&mut self, new_pipeline_defs: Vec<String>)  {
            info!("New pipeline definitions set: {:#?}", new_pipeline_defs);
            self.pipeline_defs = new_pipeline_defs;
        }

        pub fn get_pipelines(&self) -> &Vec<Pipeline> {
            // info!("Pipelines successfully retrieved: \n{:#?}", &self.pipelines);
            &self.pipelines
        }
        pub fn set_pipelines(&mut self, new_pipelines: Vec<Pipeline>)  {
            info!("New pipelines set: \n{:#?}", new_pipelines);
            self.pipelines = new_pipelines;
        }

        pub fn get_action_defs(&self) -> &Vec<String> {
            info!("Actions successfully retrieved from configuration: {:#?}", &self.action_defs);
            &self.action_defs
        }
        pub fn set_action_defs(&mut self, new_action_defs: Vec<String>)  {
            info!("New action definitions set: {:#?}", new_action_defs);
            self.action_defs = new_action_defs;
        }

        pub fn get_actions(&self) -> &Vec<Action> {
            // info!("Actions successfully retrieved: {:#?}", &self.actions);
            &self.actions
        }
        pub fn set_actions(&mut self, new_actions: Vec<Action>)  {
            info!("New actions set: \n{:#?}", new_actions);
            self.actions = new_actions;
        }

        pub fn get_all_actions(&self) -> Vec<Action> {
            let mut actions: Vec<Action> = vec![];
            for action in self.get_actions() {
                actions.push(action.to_owned());
            }
            for pipeline in self.get_pipelines() {
                for action in pipeline.get_pipeline_config().get_actions() {
                    actions.push(action.to_owned());
                }
            }
            actions
        }

    }
    

    //holds action-specific configuration information
    #[derive(Debug, Clone)]
    #[derive(PartialEq)]
    pub struct Action {
        shared_config: ShareableConfiguration,
        action_config: ActionConfig
    }
    impl Action {
        pub fn new(shared_config: ShareableConfiguration, action_config: ActionConfig) -> Action {
            Action { shared_config , action_config }
        }

        pub fn get_shared_config(&self) -> &ShareableConfiguration {
            &self.shared_config
        }
        pub fn set_shared_config(&mut self, new_shared_config: ShareableConfiguration) {
            self.shared_config = new_shared_config;
        }

        pub fn get_action_config(&self) -> &ActionConfig {
            &self.action_config
        }
        pub fn set_action_config(&mut self, action_config: ActionConfig)  {
            self.action_config = action_config;
        }
    }
    
    #[derive(Debug, Clone)]
    #[derive(PartialEq, Eq)]
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
        pub fn new(conditions: Option<Vec<Condition>>, retries: Option<i8>, allowed_failure: Option<bool>, manual: Vec<Step>) -> Self {
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

        pub fn get_conditions(&self) -> Option<Vec<Condition>> {
            self.conditions.clone()
        }
        pub fn set_conditions(&mut self, new_conditions: Vec<Condition>)  {
            info!("New conditions set: {:#?}", new_conditions);
            self.conditions = Some(new_conditions);
        }

        pub fn get_retries(&self) -> &i8 {
            info!("Retry count successfully acquired: {} ", &self.retries);
            &self.retries
        }
        pub fn set_retries(&mut self, new_retries: i8)  {
            info!("New retry count set: {:?}", &new_retries);
            self.retries = new_retries
        }

        pub fn get_allowed_failure(&self) -> &bool {
            info!("Failure allowance successfully acquired: {} ", &self.allowed_failure);
            &self.allowed_failure
        }
        pub fn set_allowed_failure(&mut self, new_allowed_failure: bool)  {
            info!("New failure allowance set: {:?}", &new_allowed_failure);
            self.allowed_failure = new_allowed_failure;
        }

        pub fn get_manual(&self) -> &Vec<Step> {
            info!("Manual successfully retrieved: {:#?}", &self.manual);
            &self.manual
        }
        pub fn set_manual(&mut self, new_manual: Vec<Step>)  {
            info!("New manual set: {:#?}", new_manual);
            self.manual = new_manual;
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Pipeline {
        shared_config: ShareableConfiguration,
        pipeline_config: PipelineConfig
    }
    impl Pipeline {
        pub fn new(shared_config: ShareableConfiguration, pipeline_config: PipelineConfig) -> Pipeline {
            Pipeline { shared_config , pipeline_config }
        }

        pub fn get_shared_config(&self) -> &ShareableConfiguration {
            &self.shared_config
        }
        pub fn set_shared_config(&mut self, new_shared_config: ShareableConfiguration) {
            self.shared_config = new_shared_config;
        }

        pub fn get_pipeline_config(&self) -> &PipelineConfig {
            &self.pipeline_config
        }
        pub fn set_action_config(&mut self, new_pipeline_config: PipelineConfig)  {
            self.pipeline_config = new_pipeline_config;
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct PipelineConfig { 
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
        //not able to be set on new objects
        //default = false
        has_run: bool,

        //A pipeline should be able to wait until other pipelines have executed before it attempts to run
        //not required at runtime
        //default = empty Vector
        requires: Vec<String>

    }
    impl PipelineConfig {
        pub fn new(conditions: Option<Vec<Condition>>, action_defs: Vec<String>, actions: Vec<Action>, requires: Option<Vec<String>>) -> Self {
            let has_run = false;
            let requires = match requires {
                Some(requires) => {
                    requires
                } None => {
                    vec![]
                }  
            };
            Self{ conditions, action_defs, actions, has_run, requires }
        }
        pub fn get_conditions(&self) -> Result<&Vec<Condition>, &'static str> {
            match &self.conditions {
                Some(conditions) => {
                    info!("Conditions successfully retrieved: {:#?}", &conditions);
                    Ok(conditions)
                } None => {
                    let res_str = "No conditions found or no conditions configured.";
                    warn!("{}", res_str);
                    Err(res_str)
                }
            }
        }
        pub fn set_conditions(&mut self, new_conditions: Vec<Condition>)  {
            info!("New conditions set: {:#?}", new_conditions);
            self.conditions = Some(new_conditions);
        }

        pub fn get_action_defs(&self) -> &Vec<String> {
            &self.action_defs
        }

        pub fn get_actions(&self) -> &Vec<Action> {
            &self.actions
        }
    }
    
    //Holds information with conditions that will resolve to either true or false
    #[derive(Debug, Clone)]
    #[derive(PartialEq, Eq)]
    pub struct Condition {
        //A name is necessary for a condition to exist.
        //There cannot be a default name for a condition, as it would be meaningless
        name: String,
        //A condition is also necessary for a condition to exist.
        //There cannot be a default condition for a condition struct, as it would be forced to default to true
        condition: String
    }
    impl Condition {
        pub fn new(name: String, condition: String) -> Condition {
            Condition { name, condition }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_condition(&self) -> &str {
            &self.condition
        }

        pub fn update_condition(&mut self, name: String, condition: String)  {
            self.name = name;
            self.condition = condition;
        }
    }


    //Holds hashmap information with data necessary to run scripts
    #[derive(Debug, Clone)]
    #[derive(PartialEq, Eq)]
    pub struct Step {
        name: String,
        script: String
    }
    impl Step {
        pub fn new(name: String, script: String) -> Self {
            Self {name, script}
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn get_script(&self) -> &str {
            &self.script
        }
        pub fn update_script(&mut self, name: String, script: String)  {
            self.name = name;
            self.script = script;
        }
    }

}

pub mod parsing {
    use std::{fs, collections::HashMap};
    use json::JsonValue;
    use log::{warn, error};
    use std::env::current_dir;
    use relative_path::RelativePath;
    use super::config::*;

    pub struct Parser {
    }

    impl Parser {

        

        fn parse_json_map(json: &JsonValue) -> HashMap<String, String>{
            // println!("{:#?}", json);
            let mut map = HashMap::new();
            for key_value in json.entries() {
                map.insert(key_value.0.to_string(), key_value.1.to_string());
            }
            // println!("{:#?}", json);
            if map.is_empty() {
                warn!("No mappable values found in json hashmap {:#?}", json);
                return map;
            }
            map
        } 

        fn parse_json_to_conditions(json: &JsonValue) -> Vec<Condition> {
            // info!("{:#?}", json);
            let mut conditions = vec![];
            for key_value in json.entries() {
                conditions.push(Condition::new(key_value.0.to_string(), key_value.1.to_string()));
            }
            conditions
        }

        fn parse_json_to_steps( json: &JsonValue) -> Vec<Step> {
            // info!("{:#?}", json);
            let mut steps = vec![];
            for key_value in json.entries() {
                steps.push(Step::new(key_value.0.to_string(), key_value.1.to_string()));
            }
            steps
        }

        fn parse_json_vector(json: &JsonValue) -> Vec<String> {
            // println!("{:#?}", json);
            let mut vec = vec![];
            for value in json.members() {
                vec.push(value.to_string())
            }
            // println!("{:#?}", json);
            if vec.is_empty() {
                warn!("No mappable values found in json vector {:#?}", json);
                return vec;
            }
            vec
        }

        fn parse_action_defs(shared_config: &ShareableConfiguration, action_defs: &Vec<String>, data: &JsonValue) -> Vec<Action> {
            let mut actions = vec![];
            for str in action_defs {
                actions.push(Self::parse_action(shared_config, &data[str], str));
            }
            actions
        }

        fn parse_action(shared_config: &ShareableConfiguration, json: &JsonValue, name: &str) -> Action {
            let root = current_dir().unwrap();
            if json.is_null(){
                panic!("Could not find action defined with appropriate tag: {}", name)
            }
            let backend = {
                if json["backend"].is_null() {
                    shared_config.get_backend().to_string()
                } else {
                    json["backend"].to_string()
                }
            };
            
            let new_shared_config = ShareableConfiguration::new(
                {
                    if json["metadata"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["metadata"]))
                    }
                },
                Some(name.to_string()),
                {
                    if json["tags"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["tags"]))
                    }
                },
                { 
                    if json["language"].is_null() {
                        shared_config.get_language().to_string()
                    } else {
                        json["language"].to_string()
                    }
                },
                {
                    if !backend.to_lowercase().eq("docker") {
                        warn!("Image cannot be set if docker is not the backend.");
                        None
                    }
                    else if json["image"].is_null() {
                        Some(shared_config.get_image().unwrap())
                    } else {
                        Some(json["image"].to_string())
                    }
                },
                backend,
                { 
                    if json["output_directory"].is_null() {
                        shared_config.get_output().to_string()
                    } else {
                        RelativePath::new(&json["output_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()
                    }
                },
                { 
                    if json["source_directory"].is_null() {
                        shared_config.get_source().to_string()
                    } else {
                        RelativePath::new(&json["source_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()
                    }
                },
            );

            let action_config = ActionConfig::new(
                {
                    let conditions = Self::parse_json_to_conditions(&json["conditions"]);
                    if conditions.is_empty() {
                        None
                    } else {
                        Some(conditions)
                    }
                },
                {
                    if json["retries"].is_null() {
                        Some(0)
                    } else {
                        Some(json["retries"].as_i8().unwrap_or_else(|| {
                            error!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                            panic!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                        }))
                    }
                },
                {
                    if json["allowed_failure"].is_null() {
                        Some(false)
                    } else {
                        Some(json["allowed_failure"].as_bool().unwrap_or_else(|| {
                            error!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                            panic!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                            }
                        ))
                    }
                },
                {
                    let manual = Self::parse_json_to_steps(&json["manual"]);
                    if manual.is_empty() {
                        error!("Actions require at least one step in their manual. Error occured in Action: {}", name);
                        panic!("Actions require at least one step in their manual. Error occured in Action: {}", name);
                    }
                    manual
                }
            );
            Action::new( new_shared_config, action_config)

        }

        fn parse_pipeline_defs(shared_config: &ShareableConfiguration, json: &JsonValue, pipeline_defs: &Vec<String>) -> Vec<Pipeline> {
            let mut pipelines = vec![];
            for str in pipeline_defs {
                pipelines.push(Self::parse_pipeline(shared_config, &json[str], str));
            }
            pipelines
        }

        fn parse_pipeline(shared_config: &ShareableConfiguration, json: &JsonValue, name: &str) -> Pipeline {
            let root = current_dir().unwrap();
            if json.is_null() {
                panic!("No pipeline found with the name: {}", name);
            }
            let backend = {
                if json["backend"].is_null() {
                    shared_config.get_backend().to_string()
                } else {
                    json["backend"].to_string()
                }
            };
            
            let new_shared_config = ShareableConfiguration::new(
                {
                    if json["metadata"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["metadata"]))
                    }
                },
                Some(name.to_string()),
                {
                    if json["tags"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["tags"]))
                    }
                },
                { 
                    if json["language"].is_null() {
                        shared_config.get_language().to_string()
                    } else {
                        json["language"].to_string()
                    }
                },
                {
                    if !backend.to_lowercase().eq("docker") {
                        warn!("Image cannot be set if docker is not the backend.");
                        None
                    } else if json["image"].is_null() {
                        Some(shared_config.get_image().unwrap())
                    } else {
                        Some(json["image"].to_string())
                    }
                },
                backend,
                { 
                    if json["output_directory"].is_null() {
                        shared_config.get_output().to_string()
                    } else {
                        RelativePath::new(&json["output_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()
                    }
                },
                { 
                    if json["source_directory"].is_null() {
                        shared_config.get_source().to_string()
                    } else {
                        RelativePath::new(&json["source_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()
                    }
                },
            );

            let pipeline_config = PipelineConfig::new(
                {
                    let conditions = Self::parse_json_to_conditions(&json["conditions"]);
                    if conditions.is_empty() {
                        None
                    } else {
                        Some(conditions)
                    }
                },
                { 
                    if json["actions"].is_null() {
                        panic!("No list of action definitions found!");
                    } else {
                        Self::parse_json_vector(&json["actions"])
                    }
                },
                Self::parse_action_defs(&new_shared_config, &Self::parse_json_vector(&json["actions"]), json),
                { 
                    if json["requires"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_vector(&json["requires"]))
                    }
                }
            );
            Pipeline::new( new_shared_config, pipeline_config ) 
        }

        fn parse_shared_config(json: &JsonValue) -> ShareableConfiguration {
            let root = current_dir().unwrap();
            let backend = {
                if json["backend"].is_null() {
                    "bash".to_string()
                } else {
                    json["backend"].to_string()
                }
            };
            
            let new_shared_config = ShareableConfiguration::new(
                {
                    if json["metadata"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["metadata"]))
                    }
                },
                Some(json["title"].to_string()),
                {
                    if json["tags"].is_null() {
                        None
                    } else {
                        Some(Self::parse_json_map(&json["tags"]))
                    }
                },
                { 
                    if json["language"].is_null() {
                        "Python".to_string()
                    } else {
                        json["language"].to_string()
                    }
                },
                {
                    if !backend.to_lowercase().eq("docker") {
                        warn!("Image cannot be set if docker is not the backend.");
                        None
                    } else if json["image"].is_null() {
                        None
                    } else {
                        Some(json["image"].to_string())
                    }
                    
                },
                backend,
                { 
                    if json["output_directory"].is_null() {
                        RelativePath::new("./dist/cider/").to_path(&root).to_str().unwrap().to_string()

                    }
                    else {
                        RelativePath::new(&json["output_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()

                    }
                },
                { 
                    if json["source_directory"].is_null() {
                        RelativePath::new("./src").to_path(&root).to_str().unwrap().to_string()
                    }
                    else {
                        RelativePath::new(&json["source_directory"].to_string()).to_path(&root).to_str().unwrap().to_string()
                    }
                },
            );
            new_shared_config
        }

        pub fn new_top_level(filename: &str) -> TopLevelConfiguration {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                eprintln!("{}", err);
                error!("There was an error locating your configuration file: {}", err);
                panic!("{}",err.to_string());
            });
            let parsed_data = json::parse(&file_contents).unwrap_or_else(|err| {
                eprintln!();
                error!("There was an error parsing your configuration file: {}", err);
                panic!("{}", err.to_string());
            });
            let s_config = Self::parse_shared_config(&parsed_data);
            let pipeline_defs = {
                if (parsed_data["pipelines"]).is_null() {
                    vec![]
                }
                else {
                    Self::parse_json_vector(&parsed_data["pipelines"])
                }
            };
            let pipelines = Self::parse_pipeline_defs(&s_config, &parsed_data, &pipeline_defs);
            let action_defs = {
                if (parsed_data["actions"]).is_null() {
                    vec![]
                }
                else {
                    Self::parse_json_vector(&parsed_data["actions"])
                }
            };
            let actions = Self::parse_action_defs(&s_config, &action_defs, &parsed_data);
            TopLevelConfiguration::new(s_config, pipeline_defs, pipelines, action_defs, actions)
        }

        pub fn overwrite_top_level(mut config: TopLevelConfiguration, filename: &str) -> TopLevelConfiguration {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                eprintln!("{}", err);
                error!("There was an error locating your configuration file: {}", err);
                panic!("{}",err.to_string());
            });
            let parsed_data = json::parse(&file_contents).unwrap_or_else(|err| {
                eprintln!();
                error!("There was an error parsing your configuration file: {}", err);
                panic!("{}", err.to_string());
            });
            config.set_shared_config(Self::parse_shared_config(&parsed_data));
            config.set_pipeline_defs(
                {
                    if (parsed_data["pipelines"]).is_null() {
                        vec![]
                    }
                    else {
                        Self::parse_json_vector(&parsed_data["pipelines"])
                    }
                }
            );
            config.set_pipelines(Self::parse_pipeline_defs(config.get_shared_config(), &parsed_data, config.get_pipeline_defs()));
            config.set_action_defs(
                {
                    if (parsed_data["actions"]).is_null() {
                        vec![]
                    }
                    else {
                        Self::parse_json_vector(&parsed_data["actions"])
                    }
                }
            );
            config.set_actions(Self::parse_action_defs(config.get_shared_config(), config.get_action_defs(), &parsed_data));
            config
        }

        //Created strictly for testing purposes.
        pub fn parse_json_string(filename: &str) -> JsonValue {
            let contents = fs::read_to_string(filename).unwrap();
            let parsed_data = json::parse(&contents);
            // println!("{:#?}", parsed_data.as_ref().unwrap().clone());
            parsed_data.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{parsing::*};
    use log::info;
    #[test]
    fn print_file_name(){
        // let config = Configuration::new(None).unwrap();
        println!();
        assert!(true);
    }

    #[test]
    fn parse_string() {
        info!("{}", Parser::parse_json_string("example_docker_config.json"));
        assert!(true);
    }

    // #[test]
    // fn test_parse_top_level_actions() {
    //     let config = Parser::new_top_level("example_docker_config.json");
    //     for action in config.get_actions(){
    //         // println!("{:#?}", action);
    //     }
    //     assert!(true);
    // }

    // #[test]
    // fn test_parse_pipeline_actions() {
    //     let config = Parser::new_top_level("example_docker_config.json");
    //     for pipeline in config.get_pipelines(){
    //         for action in pipeline.get_pipeline_config().get_actions() {
    //             // println!("{:#?}", action);
    //         }
    //     }
    // }

    // #[test]
    // fn test_parse_pipeline() {
    //     let config = Parser::new_top_level("example_docker_config.json");
    //     for pipeline in config.get_pipelines() {
    //         // println!("{:#?}", pipeline);
    //     }
    // }


}