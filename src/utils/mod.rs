/// Holds information relevant to configuration
pub mod config;

/// This module will create a configuration for you
/// Not implemented yet.
pub mod config_generator;

/// This module executes scripts based on the configuration provided
pub mod executor;

/// This module contains the necessary functionality to parse configuration files into a usable form.
pub mod parsing;

/// This module contains functionality relevant to the watch functionality of this program (Not implemented yet.)
pub mod watcher;
