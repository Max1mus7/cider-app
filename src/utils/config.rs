use log::{info, warn};
use std::collections::HashMap;

/// Contains information that can be shared between levels of a configuration
///
/// Contains information to be shared from a higher-level of configuration to a lower-level of a configuration.
/// This contains some "aesthetic" data, like metadata and tags, but also can contain some useful information like
/// the source directory you would like as your root, your output directory, and more.
///
/// [`ShareableConfiguration`]s also contain program-critical information like the backend, image, and the language you would
/// like to use in order to run your [`Action`] steps.
///
/// Configuration levels are as follows:
/// | Level | Priority |
/// |-------|----------|
/// | Top   |     1    |
/// | Pipeline |  2    |
/// | Action|     3    |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareableConfiguration {
    /// metadata not required
    ///
    /// defaulted to None
    metadata: Option<HashMap<String, String>>,

    /// title not required
    /// title can be used to name different configuration sections, but is completely optional, unlike action_defs or pipeline_defs
    /// in [`TopLevelConfiguration`]
    /// defaulted to None
    title: Option<String>,

    /// tags not required
    /// tags do nothing functionally at the moment, but may be used in a later release.
    /// defaulted to None
    tags: Option<HashMap<String, String>>,

    ///language required at runtime
    ///defaulted to bash
    language: String,

    /// image not required
    /// defaulted to None
    /// if "docker" is specified as a backend, this will default to alpine:latest
    /// IMAGE IS A DOCKER-SPECIFIC FEATURE. IF BACKEND IS NOT DOCKER, IMAGE SHOULD NOT BE DEFINED
    image: Option<String>,

    /// backend required
    /// defaulted to local(Windows in this case)
    backend: String,

    /// Output directory required
    /// defaulted to ./dist/cider/
    output: String,

    /// Source directory required
    /// defaulted to ./src
    source: String,
}

impl ShareableConfiguration {
    /// Creates a new [`ShareableConfiguration`]
    ///
    /// Some values are completely optional, and will either be defaulted or set as None if not provided.
    /// Note that some required information is set by default in [`crate::utils::parsing::json_parser`] if it is not explicitly defaulted here.
    /// Specifically, output, and source are defaulted to ./dist/cider and ./src, respectively.
    ///
    /// # Examples:
    /// Basic usage:
    /// ```
    /// use cider::config::ShareableConfiguration;
    ///
    /// let s = ShareableConfiguration::new(None, None, None, "Rust".to_string(), None, "bash".to_string(), "./dist/cider".to_string(), "./src".to_string());
    /// ```
    ///
    pub fn new(
        metadata: Option<HashMap<String, String>>,
        title: Option<String>,
        tags: Option<HashMap<String, String>>,
        language: String,
        image: Option<String>,
        backend: String,
        output: String,
        source: String,
    ) -> Self {
        let image = {
            if !backend.to_lowercase().eq("docker") {
                None
            } else {
                image
            }
        };
        Self {
            metadata,
            title,
            tags,
            language,
            image,
            backend,
            output,
            source,
        }
    }

    /// Returns metadata
    ///
    /// Returns the metadata associated with a [`ShareableConfiguration`], and logs whether the retrieval was successful
    /// or of a None type.
    ///
    /// # Warnings
    /// Will provide the user with a warning if metadata obtained returns a None type.
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_metadata();
    /// ```
    pub fn get_metadata(&self) -> Option<HashMap<String, String>> {
        match &self.metadata {
            Some(metadata) => {
                info!("Metadata successfully retrieved: {:#?}", &metadata);
                Some(metadata.to_owned())
            }
            None => {
                let res_str = "No metadata value found or no metadata value configured.";
                warn!("{}", res_str);
                None
            }
        }
    }

    ///Allows the metadata of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    /// use std::collections::HashMap;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let mut hm = HashMap::new();
    /// hm.insert("some metadata tag".to_string(), "some metadata data".to_string());
    ///
    /// let m = s.s_config.set_metadata(hm.clone());
    ///
    /// assert_eq!(s.s_config.get_metadata().unwrap(), hm);
    /// ```
    pub fn set_metadata(&mut self, new_metadata: HashMap<String, String>) {
        info!("New metadata set: {:#?}", new_metadata);
        self.metadata = Some(new_metadata);
    }

    /// Returns the title
    ///
    /// Returns the title associated with a [`ShareableConfiguration`], and logs whether the retrieval was successful
    /// or of a None type.
    ///
    /// # Warnings
    /// Will provide the user with a warning if metadata obtained returns a None type.
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_title();
    /// ```
    pub fn get_title(&self) -> Option<String> {
        match &self.title {
            Some(title) => {
                info!("Title successfully retrieved: {:?}", &title);
                Some(title.to_string())
            }
            None => {
                let res_str = "No title value found or no title value configured.";
                warn!("{}", res_str);
                None
            }
        }
    }

    ///Allows the title of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let t = "Cider".to_string();
    ///
    /// s.s_config.set_title(t.clone());
    ///
    /// assert_eq!(s.s_config.get_title().unwrap(), t);
    /// ```
    pub fn set_title(&mut self, new_title: String) {
        info!("New title set: {}", new_title);
        self.title = Some(new_title);
    }

    /// Returns tags
    ///
    /// Returns the tags associated with a [`ShareableConfiguration`], and logs whether the retrieval was successful
    /// or of a None type.
    ///
    /// # Warnings
    /// Will provide the user with a warning if metadata obtained returns a None type.
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_tags();
    /// ```
    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        match &self.tags {
            Some(tags) => {
                info!("Tags successfully retrieved: {:?}", &tags);
                Some(tags.to_owned())
            }
            None => {
                let res_str = "No tags found or no tags configured.";
                warn!("{}", res_str);
                None
            }
        }
    }

    ///Allows the tags of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///use std::collections::HashMap;
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let mut hm = HashMap::new();
    /// hm.insert("some tag".to_string(), "some data".to_string());
    ///
    /// let m = s.s_config.set_tags(hm.clone());
    ///
    /// assert_eq!(s.s_config.get_tags().unwrap(), hm);
    /// ```
    pub fn set_tags(&mut self, new_tags: HashMap<String, String>) {
        self.tags = Some(new_tags);
    }

    /// Returns language
    ///
    /// Returns the language associated with a [`ShareableConfiguration`], and logs whether the retrieval was successful
    /// or of a None type.
    ///
    /// # Warnings
    /// Will provide the user with a warning if metadata obtained returns a None type.
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_language();
    /// println!("{}", m);
    /// ```
    pub fn get_language(&self) -> &str {
        &self.language
    }

    /// Allows the language of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let l = "Rust".to_string();
    ///
    /// s.s_config.set_language(l.clone());
    ///
    /// assert_eq!(s.s_config.get_language(), l);
    /// ```
    pub fn set_language(&mut self, new_language: String) {
        info!("New language set: {}", new_language);
        self.language = new_language;
    }

    /// Returns configured image
    ///
    /// Returns the image associated with a [`ShareableConfiguration`], and logs whether the retrieval was successful
    /// or of a None type.
    ///
    /// # Warnings
    /// Will provide the user with a warning if image obtained returns a None type.
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_image();
    /// ```
    pub fn get_image(&self) -> Option<String> {
        match &self.image {
            Some(image) => {
                info!("Image successfully retrieved: {:?}", &image);
                Some(image.to_string())
            }
            None => {
                if Self::get_backend(&self) == "docker" {
                    let res_str = "No image found or no image configured.";
                    warn!("{}", res_str);
                }
                None
            }
        }
    }

    /// Allows the image of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let i = "rust:1.65.0".to_string();
    ///
    /// let m = s.s_config.set_image(i.clone());
    ///
    /// assert_eq!(s.s_config.get_image().unwrap(), i);
    /// ```
    pub fn set_image(&mut self, new_image: String) {
        if !self.get_backend().to_lowercase().eq("docker") {
            warn!("image can only be set on configurations with a docker backend");
            self.image = None
        }
        info!("New title set: {}", new_image);
        self.image = Some(new_image);
    }

    /// Returns backend
    ///
    /// Returns the backend associated with a [`ShareableConfiguration`]
    ///
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_backend();
    /// ```
    pub fn get_backend(&self) -> &str {
        &self.backend
    }

    ///Allows the backend of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let b = "bash".to_string();
    ///
    /// s.s_config.set_backend(b.clone());
    ///
    /// assert_eq!(s.s_config.get_backend(), b);
    /// ```
    pub fn set_backend(&mut self, new_backend: String) {
        info!("New backend set: {}", new_backend);
        self.backend = new_backend;
    }

    /// Returns output directory
    ///
    /// Returns the output directory associated with a [`ShareableConfiguration`]
    ///
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_output();
    /// ```
    pub fn get_output(&self) -> &str {
        info!(
            "Output directory successfully retrieved: {:?}",
            &self.output
        );
        &self.output
    }

    ///Allows the output directory of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let o = "./dist/cider".to_string();
    ///
    /// s.s_config.set_output(o.clone());
    ///
    /// assert_eq!(s.s_config.get_output(), o);
    /// ```
    pub fn set_output(&mut self, new_output: String) {
        info!("New output directory set: {}", new_output);
        self.output = new_output;
    }

    /// Returns source directory
    ///
    /// Returns the source directory associated with a [`ShareableConfiguration`]
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let s = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = s.s_config.get_source();
    /// ```
    pub fn get_source(&self) -> &str {
        info!(
            "Source directory successfully retrieved: {:?}",
            &self.source
        );
        &self.source
    }

    ///Allows the source directory of a [`ShareableConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration, which contains a ShareableConfiguration
    /// let mut s = json_parser::new_top_level("./cider_config.json");
    /// let src = "./src".to_string();
    ///
    /// s.s_config.set_source(src.clone());
    ///
    /// //assert_eq!(s.s_config.get_source(), &src);
    /// ```
    pub fn set_source(&mut self, new_source: String) {
        info!("New source directory set: {}", new_source);
        self.backend = new_source;
    }
}

/// Contains information pertinent to a CIder configuration as a whole.
///
/// A [`TopLevelConfiguration`] is meant to contain information relevant to multiple pipelines, or actions, or metadata/information relevant
/// to the entire configuraiton.
///
/// [`TopLevelConfiguration`]s are the highest level of configuration possible, and they allow the user to define actions and pipelines
/// to be used by the program and allow for better-organized configuration structures.
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopLevelConfiguration {
    /// ShareableConfiguration data required to perform top-level tasks and pass on to lower-level tasks. See [`ShareableConfiguration`]
    pub s_config: ShareableConfiguration,

    ///pipeline definitions required at runtime, even if it is an empty Vector
    pipeline_defs: Vec<String>,

    /// The data used to run pipelines
    ///
    /// Pipelines require certain information in order to not only be valid but also in order to function.
    /// Pipelines are meant to hold actions, but actions can still be defined at the top level.
    pipelines: Vec<Pipeline>,

    ///Top-level action definitions
    ///
    /// Action definitions are an array of strings that contain the names of actions to be invoked by CIder.
    action_defs: Vec<String>,

    ///Top-level actions not required for a TopLevelConfiguration implementation to be valid
    actions: Vec<Action>,
}

impl TopLevelConfiguration {
    /// Creates a new [`TopLevelConfiguration`].
    ///
    /// Creates a new [`TopLevelConfiguration`]. Information that is not provided via a configuration file is defaulted when
    /// a configuration file is parsed in [`crate::utils::parsing`].
    ///
    /// For more information, see new_top_level() in [`crate::utils::parsing`]
    pub fn new(
        s_config: ShareableConfiguration,
        pipeline_defs: Vec<String>,
        pipelines: Vec<Pipeline>,
        action_defs: Vec<String>,
        actions: Vec<Action>,
    ) -> Self {
        Self {
            s_config,
            pipeline_defs,
            pipelines,
            action_defs,
            actions,
        }
    }

    /// Returns pipeline definitions
    ///
    /// Returns the a reference to the pipeline definitions associated with a [`TopLevelConfiguration`] in a vector form
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let t = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = t.get_pipeline_defs();
    /// ```
    pub fn get_pipeline_defs(&self) -> &Vec<String> {
        info!(
            "Pipelines successfully retrieved from configuration: {:#?}",
            &self.pipeline_defs
        );
        &self.pipeline_defs
    }

    ///Allows the pipeline definitions of a [`TopLevelConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let mut t = json_parser::new_top_level("./cider_config.json");
    /// let p = vec!["Pipeline_1".to_string(), "Pipeline_2".to_string(), "Pipeline_3".to_string()];
    ///
    /// t.set_pipeline_defs(p.clone());
    ///
    /// assert_eq!(t.get_pipeline_defs(), &p);
    /// ```
    pub fn set_pipeline_defs(&mut self, new_pipeline_defs: Vec<String>) {
        info!("New pipeline definitions set: {:#?}", new_pipeline_defs);
        self.pipeline_defs = new_pipeline_defs;
    }

    /// Returns all [`Pipeline`]s contained by a TopLevelConfiguration
    ///
    /// Returns the a reference to the [`Pipeline`]s associated with a [`TopLevelConfiguration`] in a vector form
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let t = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = t.get_pipelines();
    /// ```
    pub fn get_pipelines(&self) -> &Vec<Pipeline> {
        // info!("Pipelines successfully retrieved: \n{:#?}", &self.pipelines);
        &self.pipelines
    }

    ///Allows the [`Pipeline`]s of a [`TopLevelConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let mut t = json_parser::new_top_level("./cider_config.json");
    /// let mut p = t.get_pipelines().clone();
    /// p.pop();
    ///
    /// t.set_pipelines(p.clone());
    ///
    /// assert_eq!(t.get_pipelines(), &p);
    /// ```
    pub fn set_pipelines(&mut self, new_pipelines: Vec<Pipeline>) {
        info!("New pipelines set: \n{:#?}", new_pipelines);
        self.pipelines = new_pipelines;
    }

    /// Returns action definitions
    ///
    /// Returns the a reference to the [`Action`] definitions associated with a [`TopLevelConfiguration`] in a vector form
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let t = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = t.get_action_defs();
    /// ```
    pub fn get_action_defs(&self) -> &Vec<String> {
        info!(
            "Actions successfully retrieved from configuration: {:#?}",
            &self.action_defs
        );
        &self.action_defs
    }

    ///Allows the action definitions of a [`TopLevelConfiguration`] to be changed
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let mut t = json_parser::new_top_level("./cider_config.json");
    /// let p = vec!["Action_1".to_string(), "Action_2".to_string(), "Action_3".to_string()];
    ///
    /// t.set_action_defs(p.clone());
    ///
    /// assert_eq!(t.get_action_defs(), &p);
    /// ```
    pub fn set_action_defs(&mut self, new_action_defs: Vec<String>) {
        info!("New action definitions set: {:#?}", new_action_defs);
        self.action_defs = new_action_defs;
    }

    /// Returns [`Action`] information
    ///
    /// Returns the a reference to the [`Action`]s associated with a [`TopLevelConfiguration`] in a vector form
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let t = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = t.get_action_defs();
    pub fn get_actions(&self) -> &Vec<Action> {
        // info!("Actions successfully retrieved: {:#?}", &self.actions);
        &self.actions
    }

    ///Allows the [`Action`]s of a [`TopLevelConfiguration`] to be changed
    ///
    pub fn set_actions(&mut self, new_actions: Vec<Action>) {
        info!("New actions set: \n{:#?}", new_actions);
        self.actions = new_actions;
    }

    /// Returns every action in the configuration
    ///
    /// Returns the a reference to the [`Action`] definitions associated with a [`TopLevelConfiguration`] and all underlying [`Pipeline`]s in a vector form
    ///
    /// # Examples:
    /// ```
    /// use cider::parsing::json_parser;
    ///
    /// //returns a TopLevelConfiguration
    /// let t = json_parser::new_top_level("./cider_config.json");
    ///
    /// let m = t.get_all_actions();
    pub fn get_all_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = vec![];
        for action in self.get_actions() {
            actions.push(action.to_owned());
        }
        for pipeline in self.get_pipelines() {
            for action in pipeline.pipeline_config.get_actions() {
                actions.push(action.to_owned());
            }
        }
        actions
    }
}

///holds action-specific configuration information
///
/// Actions are designed to hold the necessary information to run scripts, as well as any specific configuration pieces that may be necessary.
///
/// It is important to note that action-specific configuration overrides [`ShareableConfiguration`] information provided from any other level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    /// ShareableConfiguration data required to perform bottom-level tasks. See [`ShareableConfiguration`]
    pub shared_config: ShareableConfiguration,

    /// Contains configuration information relevant only to [`Action`]s
    pub action_config: ActionConfig,
}

impl Action {
    ///Creates a new [`Action`]
    pub fn new(shared_config: ShareableConfiguration, action_config: ActionConfig) -> Action {
        Action {
            shared_config,
            action_config,
        }
    }
}

/// Contains information required to run defined [`Action`]s
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionConfig {
    ///The conditions which are required to be true in order for the program to run an action
    /// Currently not implemented.
    conditions: Option<Vec<Condition>>,

    /// Specifies how many times the program will a given action in the event that the result is a failure.
    /// Currently not implemented.
    retries: i8,

    ///Specifies whether the action is allowed to fail and the result is still able to be considered a success
    allowed_failure: bool,

    ///The actual steps required for program execution
    /// # Examples:
    /// ```
    /// use cider::config::*;
    /// let step_1 = Step::new("step1".to_string(), "echo \"hello world!\"".to_string());
    /// let step_2 = Step::new("step1".to_string(), "echo \"hello world!\"".to_string());
    /// let manual = vec![step_1, step_2];
    /// ```
    manual: Vec<Step>,
}

impl ActionConfig {
    /// Creates a new [`ActionConfig`]
    pub fn new(
        conditions: Option<Vec<Condition>>,
        retries: Option<i8>,
        allowed_failure: Option<bool>,
        manual: Vec<Step>,
    ) -> Self {
        let retries = retries.unwrap_or(0);

        let allowed_failure = allowed_failure.unwrap_or(false);

        ActionConfig {
            conditions,
            retries,
            allowed_failure,
            manual,
        }
    }

    /// Gets all [`Condition`]s within an [`ActionConfig`]
    pub fn get_conditions(&self) -> Option<Vec<Condition>> {
        self.conditions.clone()
    }

    /// Changes the conditions within an [`ActionConfig`]
    pub fn set_conditions(&mut self, new_conditions: Vec<Condition>) {
        info!("New conditions set: {:#?}", new_conditions);
        self.conditions = Some(new_conditions);
    }

    /// Gets the retries within an [`ActionConfig`]
    pub fn get_retries(&self) -> &i8 {
        info!("Retry count successfully acquired: {} ", &self.retries);
        &self.retries
    }

    /// Changes the retries of an [`ActionConfig`]
    pub fn set_retries(&mut self, new_retries: i8) {
        info!("New retry count set: {:?}", &new_retries);
        self.retries = new_retries
    }

    /// Returns whether or not the [`Action`] is allowed to fail.
    pub fn get_allowed_failure(&self) -> &bool {
        info!(
            "Failure allowance successfully acquired: {} ",
            &self.allowed_failure
        );
        &self.allowed_failure
    }

    /// Changes the conditions within an [`ActionConfig`]
    pub fn set_allowed_failure(&mut self, new_allowed_failure: bool) {
        info!("New failure allowance set: {:?}", &new_allowed_failure);
        self.allowed_failure = new_allowed_failure;
    }

    /// Returns the [`Step`]s to be executed by the program.
    pub fn get_manual(&self) -> &Vec<Step> {
        info!("Manual successfully retrieved: {:#?}", &self.manual);
        &self.manual
    }

    /// Changes the execution of [`Step`]s within an [`ActionConfig`]
    pub fn set_manual(&mut self, new_manual: Vec<Step>) {
        info!("New manual set: {:#?}", new_manual);
        self.manual = new_manual;
    }
}

/// Contains information relevant to pipelines
///
/// Pipelines are meant to "own" multiple [`Action`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    /// ShareableConfiguration data required to perform bottom-level tasks. See [`ShareableConfiguration`]
    pub shared_config: ShareableConfiguration,

    /// Contains configuration information relevant only to [`Action`]s
    pub pipeline_config: PipelineConfig,
}
impl Pipeline {
    /// Creates a nwe [`Pipeline`]
    pub fn new(shared_config: ShareableConfiguration, pipeline_config: PipelineConfig) -> Pipeline {
        Pipeline {
            shared_config,
            pipeline_config,
        }
    }
}

/// Holds information that is specific to the functionality of [`Pipeline`]s
#[derive(Debug, Clone, PartialEq, Eq)]
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
    requires: Vec<String>,
}

impl PipelineConfig {
    ///Creates a new [`PipelineConfig`]
    pub fn new(
        conditions: Option<Vec<Condition>>,
        action_defs: Vec<String>,
        actions: Vec<Action>,
        requires: Option<Vec<String>>,
    ) -> Self {
        let has_run = false;
        let requires = match requires {
            Some(requires) => requires,
            None => {
                vec![]
            }
        };
        Self {
            conditions,
            action_defs,
            actions,
            has_run,
            requires,
        }
    }

    /// Gets all [`Condition`]s within a [`PipelineConfig`]
    pub fn get_conditions(&self) -> Result<&Vec<Condition>, &'static str> {
        match &self.conditions {
            Some(conditions) => {
                info!("Conditions successfully retrieved: {:#?}", &conditions);
                Ok(conditions)
            }
            None => {
                let res_str = "No conditions found or no conditions configured.";
                warn!("{}", res_str);
                Err(res_str)
            }
        }
    }

    /// Allows the [`Condition`]s for a [`PipelineConfig`] to be changed.
    pub fn set_conditions(&mut self, new_conditions: Vec<Condition>) {
        info!("New conditions set: {:#?}", new_conditions);
        self.conditions = Some(new_conditions);
    }

    /// Gets all action definitions within a [`PipelineConfig`]
    pub fn get_action_defs(&self) -> &Vec<String> {
        &self.action_defs
    }

    /// Returns the [`Action`]s of a [`Pipeline`].
    pub fn get_actions(&self) -> &Vec<Action> {
        &self.actions
    }
}

/// Holds information with conditions that will resolve to either true or false
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    //A name is necessary for a condition to exist.
    //There cannot be a default name for a condition, as it would be meaningless
    name: String,
    //A condition is also necessary for a condition to exist.
    //There cannot be a default condition for a condition struct, as it would be forced to default to true
    condition: String,
}

impl Condition {
    /// Creates a new [`Condition`]
    pub fn new(name: String, condition: String) -> Condition {
        Condition { name, condition }
    }

    /// Returns the [`Condition`] name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the [`Condition`]
    pub fn get_condition(&self) -> &str {
        &self.condition
    }

    /// Updates the [`Condition`] with the given information.
    pub fn update_condition(&mut self, name: String, condition: String) {
        self.name = name;
        self.condition = condition;
    }
}

/// Holds hashmap information with data necessary to run scripts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    name: String,
    script: String,
}

impl Step {
    /// Creates a new [`Step`]
    pub fn new(name: String, script: String) -> Self {
        Self { name, script }
    }

    /// Returns the name of the [`Step`]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the script to be executed in this [`Step`]
    pub fn get_script(&self) -> &str {
        &self.script
    }

    /// Changes the information held by the [`Step`]
    pub fn update_script(&mut self, name: String, script: String) {
        self.name = name;
        self.script = script;
    }
}
