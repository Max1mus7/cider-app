# CIder Configuration

## Table of Contents

1. **[Overview](#overview)**
2. **[Top-Level Configuration](#top-level-configuration)**
   1. [Overview](#overview-of-top-level-configurations)
   2. [metadata](#metadata)
   3. [title](#title)
   4. [tags](#tags)
   5. [language](#language)
   6. [image](#image)
   7. [backend](#backend)
   8. [output_directory](#output_directory)
   9. [source_directory](#source_directory)
   10. [pipelines](#pipelines)
   11. [actions](#actions)
3. **[Pipeline Configuration](#pipeline-configuration)**
   1. [conditions](#conditions)
   2. [actions](#actions-1)
   3. [requires](#requires)
4. **[Action Configuration](#action-configuration)**
   1. [conditions](#conditions-1)
   2. [retries](#retries)
   3. [allowed_failure](#allowed_failure)
   4. **[manual](#manual)**
5. **[Examples](#examples)**
6. **[References](#references)**
7. **[Additional Notes](#additional-notes)**

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

Top-Level Configurations are the most broadly scoped configuration option, with the ability to hold both [Pipeline](#pipeline-configuration) and [Action](#action-configuration) configurations within them. Additionally, most of the properties of [Pipeline](#pipeline-configuration) and [Action](#action-configuration) configurations can also be applied to a Top-Level Configuration, so as to reduce the amount of boilerplate json code used within a configuration file.

The following information details the different keywords that can be used in a cider configuration file as well as their purposes.

***

*: Settings that can be shared between different levels of configuration

#### metadata*

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

*This does not currently have any purpose, but may be able to be output or filtered in future iterations.*

***

#### title*

- A title for the configuration.

Example:

```json
{
    "title": "Configuration for API service"
}
```

*This does not currently have any purpose,* and is intended to be used purely as decoration.

***

#### tags*

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

#### language*

- A piece of metadata outlining which language of code is intended to be built/tested in a cider configuration.
- In the future, this may be used to make pipelines easier to run.
  - For example, if a pipeline's language is python, actions defined underneath it may be configured in the future to automatically run the `python` command.

Example:

```json
{
    "language": "Python"
}
```

*This does not do anything at the moment, but may be used as a way to specify which actions are performed via CLI arguments at a later date.*

***

#### image*

- For use with the Docker [backend](#backend), specifies which base image will be used to run corresponding [Action] scripts.

Example:

```json
{
    "image": "python:latest"
}
```

***

#### backend

- If no value is provided, this defaults to Windows(batch).
- The [backend](#backend) keyword is used to specify what shell or program will be used to execute the scripts outlined in [Actions](#action-configuration)
- Currently, the supported options are `bash`, `batch` or `bat`, and `docker`.
- If the `docker` backend is selected, [Action](#action-configuration) scripts will be executed within the context of a docker container. If the [image](#image) configuration is not set, the default image to be used is alpine:latest.

Example:

```json
{
    "backend": "bash"
}
```

*This is a very limited feature in its current state, but improvements are planned for the future.*

***

#### output_directory*

- Specifies the output directory that CIder will place logs into.
- This supports relative and absolute paths, but there have been some issues with how CIder handles directories both in the case of this setting and the [source_directory](#source_directory) setting. These issues will be looked into and resolved in the future.
- Default value is `./dist/cider`

Example:

```json
{
    "output_directory": "./output_logs"
}
```

***

#### source_directory*

- Specifies the "root" directory for [Action](#action-configuration) scripts to be executed within.
- This supports both relative and absolute paths, but there have been some issues with how CIder handles directories both in the case of this setting and the [source](#source) setting. These issues will be looked into and resolved in the future.
- Defaulted to ./
- This can also be used if you want to have CIder installed to a different directory from the project you are developing.

Example:

```json
{
    "source_directory": "/home/users/jsmith/dev/project_1/src"
}
```

***

#### pipelines

- An array of strings that describes what [pipelines](#pipeline-configuration) are currently active in your CIder configuration.
- [Pipelines](#pipeline-configuration) can exist within your CIder configuration that are not included in this array, but they WILL NOT be parsed/executed.

Example:

```json
{
    "pipelines": ["test_pipeline", "test_pipeline_1"],
    "test_pipeline": {
        Will be executed.
    },
    "test_pipeline_1": {
        Will be executed.
    },
    "test_pipeline_2": {
        Will not be executed.
    }
}
```

***

#### actions*

- An array of strings that describes what top-level [actions](#action-configuration) are currently active in your CIder configuration.
- These actions will be run without needing to be contained within a pipeline.
- [Actions](#pipeline-configuration) can exist within your CIder configuration that are not included in this array, but they WILL NOT be parsed/executed.

Example:

```json
{
    "actions": ["example_action", "example_action_1"],
    "example_action": {
        Will be executed.
    },
    "example_action_1": {
        Will be executed.
    },
    "example_action_2": {
        Will not be executed.
    }
}
```

***

## Pipeline Configuration

### Overview of Pipeline Configurations

Pipeline configurations are the second tier of the CIder configuration, with the ability to hold multiple [Action configurations](#action-configuration) within them. Additionally, many of the settings held within [Action configurations](#action-configuration) can also be applied to a Pipeline Configuration, so as to reduce the amount of boilerplate json code used within a configuration file (see shared keywords outlined in [Top-Level Configuration](#top-level-configuration)). Pipeline Configurations also hold some keywords which, in the future, will enable them to be run depending on certain conditions.

The following information details the different keywords that can be used in a cider configuration file as well as their purposes.

***

#### conditions*

- A JSON object that contains different conditions which must be met to run a configuration.
- The format is currently expected as { "name": "Condition" }
- This is a planned feature and does not currently do anything.

Example:

```json
{
    "pipelines": ["Example_Pipeline"],
    "Example_Pipeline": {
        "conditions": {
            "Run while watch mode enabled": "False"
        }
    }
}
```

*This is currently a W.I.P. feature. There is no timeline for the implementation of conditions.*

***

#### actions

- See [actions](#actions) for information regarding this keyword.

***

#### requires

- Forces CIder pipelines to wait to execute until other defined pipelines have executed.
- This is a future feature and is not currently supported in any form other than the configuration file accepting it.
- Pipelines should not require each other. It is not known whether this would force pipelines to infinitely try to restart, infinitely run, or not run at all.

Example:

```json
{
    "pipelines": ["Example_Pipeline", "Example_Pipeline_2"],
    "Example_Pipeline": {
        "requires": ["Example_Pipeline_2"]
    },
    "Example_Pipeline_2": {
        "actions": ["idk"],
        "idk": {
            Some_Actions
        }
    }
}
```

*This is currently a W.I.P. feature. There is no timeline for the implementation of requires.*

***

## Action Configuration

### Overview of Action Configurations

Action configurations are the third-and-final tier of the CIder configuration, with the ability to execute scripts detailed within them. Though a single Action may have multiple steps defined within it (see [manual](#manual)), the intent is for every action to take place within the same shell session. So as to reduce the amount of boilerplate json code used within a configuration file, Actions inherit settings from the two upper tiers (see shared keywords outlined in [Top-Level Configuration](#top-level-configuration)). Action Configurations also hold some action-specific settings.

The following information details the different keywords that can be used in a cider configuration file as well as their purposes.

***

#### conditions

See [conditions](#conditions)

***

#### retries

- Forces failed CIder actions to attempt to run `x` amount of times until they are considered `Failing`.
- Statuses such as `Failing` are not currently implemented and do not return user feedback.

Example:

```json
{
    "actions": ["Action_1"],
    "Action_1": {
        "retries": 2
    }
}
```

*This is currently a W.I.P. feature. There is no timeline for the implementation of retries.*

***

#### allowed_failure

- A boolean which tells whether or not an action is considered successful, even if there is an error.

Example:

```json
{
    "actions": ["Action_1"],
    "Action_1": {
        "allowed_failure": true
    }
}
```

*This is currently a W.I.P. feature. There is no timeline for the implementation of allowed_failure.*

***

#### manual

- Manuals are how CIder knows what scripts to run.
- Scripts are provided with a name, then outlined immediately after.
- As of now, each manual "step" works as an individual bash script. This will be fixed in a future iteration of CIder.

Example:

```json
{
    "actions": ["Action_1"],
    "Action_1": {
        "manual": {
            "build": "cd src/rust && cargo build",
            "test": "cd src/rust && cargo run"
        }
    }
}
```

***

## Examples

> A configuration using docker to run a cargo build:

```json
{
    "title": "CIder 0.1 Example Docker Config",
    "operating_system": "Windows",
    "backend": "Docker",
    "pipelines": ["Test_Docker"],
    "Test_Docker": {
        "image": "rust:1.65.0",
        "actions": ["Run_Tests"],
        "Run_Tests": {
            "manual": {
                "step_1": "rustc --version",
                "step_2": "cargo build"
            },
            "source_directory": "./"
        },
        "Consolidate_Reports": {
            "backend": "bash",
            "manual": {
                "combine_csvs": "python ./tests/pyscripts/accumulate_tests.py"
            }
        }
    },
    "source_directory": "./src"
}
```

> An example running cargo test via bash then consolidating runtime metrics using a python script:

```json
{
    "title": "CIder 0.1 Example Docker Config",
    "backend": "bash",
    "pipelines": ["Test_Program"],
    "Test_Program": {
        "actions": ["Run_Tests"],
        "Run_Tests": {
            "manual": {
                "step_1": "cargo test"
            }
        },
        "Consolidate_Reports": {
            "manual": {
                "combine_csvs": "python ./tests/pyscripts/accumulate_tests.py"
            }
        }
    }
}
```

> A multi-project, multi-language workflow:

```json
{
    "title": "CIder 0.1 Example Config",
    "backend": "bash",
    "pipelines": ["Test_Compiled_Programs", "Test_Interpreted_Programs"],
    "image": "rust:1.65",
    "Test_Compiled_Programs": {
        "actions": ["Run_Tests_Rs", "Run_Tests_Rb", "Run_Tests_Java", "Run_Tests_CSharp"],
        "Run_Tests_Rs": {
            "manual": {
                "build": "cd src/rust && cargo build",
                "test": "cd src/rust && cargo run"
            }
        },
        "Run_Tests_Rb": {
            "image": "ruby:3.1",
            "manual": {
                "test": "ruby ./src/ruby/test.rb"
            }
        },
        "Run_Tests_Java": {
            "image": "openjdk:18.0",
            "manual": {
                "build": "cd src/java && javac Test.java",
                "test": "cd src/java && java Test"
            }
        },
        "Run_Tests_CSharp": {
            "image": "mcr.microsoft.com/dotnet/aspnet:6.0",
            "manual": {
                "test": "cd src/dotnet && dotnet run"
            }
        }
    },
    "Test_Interpreted_Programs": {
        "actions": ["Run_Tests_Py", "Run_Tests_JS"],
        "Run_Tests_Py": {
            "image": "python:3.9",
            "manual": {
                "test": "python ./src/python/test.py"
            }
        },
        "Run_Tests_JS": {
            "image": "node:latest",
            "manual": {
                "build": "cd src/javascript && npm i",
                "test": "node ./src/javascript/test.js"
            }
        }

    },
    "source_directory": "./"
}
```

## References

For more information regarding the code, please see [the code docs](https://max1mus7.github.io/cider-app/).

## Additional Notes
