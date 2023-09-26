# CIder Configuration

## Table of Contents

1. **[Overview](#overview)**
2. **[Top-Level Configuration](#top-level-configuration)**
3. **[Pipeline Configuration](#pipeline-configuration)**
4. **[Action Configuration](#action-configuration)**
5. **[References](#references)**
6. **[Additional Notes](#additional-notes)**

## Overview

### General

This document attempts to explain the keywords that can possibly be used in a CIder configuration as well as when they should and should not be used.

Additionally, this will attempt to explain the flags that CIder can be started with.

### Configuration Structure

A CIder configuration contains a contains a hierarchy of layers, consisting of the following layers in the following order:

1. **[Top-Level Configuration](#top-level-configuration)**
2. **[Pipelines](#pipeline-configuration)**
3. **[Actions](#action-configuration)**

This order is how the configuration is expected to be nested in a cider_config.json file. The idea is that higher-level configurations can pass their config values down to lower-level configurations. For example, if a Top-Level configuration has a "backend" of "Docker", all actions and pipelines will have those set as well.

This hierarchy also allows configurations to be granular and highly customizable. For example, you could have a Top-Level Configuration with a "backend" of "Docker", and specify an Action underneath it with a "backend" of "bash". This will run the action's steps within a bash cli instead of spinning up a container.

***Note for Devs:***\
The configuration settings that can be shared between the different layers of configuration is held within the ShareableConfiguration struct.

## Top-Level Configuration

### Overview of Top-Level Configurations

Top-Level Configurations are the most broadly scoped configuration option, with the ability to hold both Pipeline and Action configurations within them. Additionally, most of the properties of Pipeline and Action configurations can also be applied to a Top-Level Configuration, so as to reduce the amount of boilerplate json code used within a configuration file.

The following information details the different keywords that can be used in a cider configuration file as well as their purposes.

***

*: Settings that can be shared between different levels of configuration

#### Metadata*

- An array that contains any information that is not directly relevant to the runtime of CIder.

Example:

```json
{
    "metadata": {
        "Version Data": "Some version data",
        "Some Data":"Some other data",
        "Random Data":"Even more random data!"
    }
}
```

\
*This does not currently have any purpose, but may be able to be output or filtered in future iterations.*

***

#### Title*

- A title for the configuration.

Example:

```json
{
    "title": "Configuration for API service"
}
```

\
*This does not currently have any purpose,* and is intended to be used purely as decoration.

***

#### Tags*

- A set of tags to be used to provide more context to a configuration.

Example:

```json
{
    "tags": {
        "Version": "3.0.0",
        "Pipeline_To_Run": "Test"
    }
}
```

*This does not do anything at the moment, but will be used as a way to specify which actions are performed via CLI arguments at a later date.*

***

## Pipeline Configuration

## Action Configuration

## References

## Additional Notes
