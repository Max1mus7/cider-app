# CIder Version 1.0.1 Release

## Updated features

### Logging

- Warning logs no longer show reduntant "Image cannot be set if docker is not the backend." warnings.
- Recursive file change information is no longer logged. (Previously logged at [Info] level.)
  - There is no immediate plan for this to return, however if a use case arises it may be reinstated, albeit reworked.
- `main_test.txt` file name changed to `cider_output.txt` for increased clarity.

### Code Quality

- Removed unused imports
- No longer retrieving directory metadata if the `-w` flag is not specified.
- Watch loop now within the watch `if` statement.
  - Previous behavior surrounded the core program function with a loop to be broken out of when the watch arg was present.

### QoL Improvements

- Metrics directory structure no longer created on runtime. This is a dev-only feature.
- An extremely basic sample .dockerignore file is now included in the repository
  - Added node_modules to this file.
  - *When using projects that include a `target` directory or large amounts of post-build code, this can significantly improve the performance of projects configured using a `docker` backend.*

### Planned improvements

#### Docker

- Improved `docker` build times(testing times).
  - As it stands, docker build takes a long time to work with package managers (most notably cargo and npm). As a result,  
- The source directory specifed on an action within a config file should be considered the root directory for Steps to execute commands in.
  - Currently, this is the directory which `cider.exe` was run from.
  - If not otherwise specified via configuration, the source directory for actions  is `cwd(or pwd for windows)/src`.
