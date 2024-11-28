# Development Tools

## Debugging

| Recipe                                                                     | Crates                                                                          | Categories                              |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | --------------------------------------- |
| [Log a debug message to the console][ex-log-debug]                         | [![log-badge]][log] [![env_logger-badge]][env_logger]                           | [![cat-debugging-badge]][cat-debugging] |
| [Log an error message to the console][ex-log-error]                        | [![log-badge]][log] [![env_logger-badge]][env_logger]                           | [![cat-debugging-badge]][cat-debugging] |
| [Log to stdout instead of stderr][ex-log-stdout]                           | [![log-badge]][log] [![env_logger-badge]][env_logger]                           | [![cat-debugging-badge]][cat-debugging] |
| [Log messages with a custom logger][ex-log-custom-logger]                  | [![log-badge]][log]                                                             | [![cat-debugging-badge]][cat-debugging] |
| [Log to the Unix syslog][ex-log-syslog]                                    | [![log-badge]][log] [![syslog-badge]][syslog]                                   | [![cat-debugging-badge]][cat-debugging] |
| [Enable log levels per module][ex-log-mod]                                 | [![log-badge]][log] [![env_logger-badge]][env_logger]                           | [![cat-debugging-badge]][cat-debugging] |
| [Use a custom environment variable to set up logging][ex-log-env-variable] | [![log-badge]][log] [![env_logger-badge]][env_logger]                           | [![cat-debugging-badge]][cat-debugging] |
| [Include timestamp in log messages][ex-log-timestamp]                      | [![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] | [![cat-debugging-badge]][cat-debugging] |
| [Log messages to a custom location][ex-log-custom]                         | [![log-badge]][log] [![log4rs-badge]][log4rs]                                   | [![cat-debugging-badge]][cat-debugging] |

[ex-log-debug]: development_tools/debugging/log.html#log-a-debug-message-to-the-console
[ex-log-error]: development_tools/debugging/log.html#log-an-error-message-to-the-console
[ex-log-stdout]: development_tools/debugging/log.html#log-to-stdout-instead-of-stderr
[ex-log-custom-logger]: development_tools/debugging/log.html#log-messages-with-a-custom-logger
[ex-log-syslog]: development_tools/debugging/log.html#log-to-the-unix-syslog
[ex-log-mod]: development_tools/debugging/config_log.html#enable-log-levels-per-module
[ex-log-env-variable]: development_tools/debugging/config_log.html#use-a-custom-environment-variable-to-set-up-logging
[ex-log-timestamp]: development_tools/debugging/config_log.html#include-timestamp-in-log-messages
[ex-log-custom]: development_tools/debugging/config_log.html#log-messages-to-a-custom-location

## Versioning

| Recipe                                                                | Crates                    | Categories                                                                    |
| --------------------------------------------------------------------- | ------------------------- | ----------------------------------------------------------------------------- |
| [Parse and increment a version string][ex-semver-increment]           | [![semver-badge]][semver] | [![cat-config-badge]][cat-config]                                             |
| [Parse a complex version string][ex-semver-complex]                   | [![semver-badge]][semver] | [![cat-config-badge]][cat-config]                                             |
| [Check if given version is pre-release][ex-semver-prerelease]         | [![semver-badge]][semver] | [![cat-config-badge]][cat-config]                                             |
| [Find the latest version satisfying given range][ex-semver-latest]    | [![semver-badge]][semver] | [![cat-config-badge]][cat-config]                                             |
| [Check external command version for compatibility][ex-semver-command] | [![semver-badge]][semver] | [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os] |

## Build Time

| Recipe                                                                           | Crates            | Categories                                              |
| -------------------------------------------------------------------------------- | ----------------- | ------------------------------------------------------- |
| [Compile and link statically to a bundled C library][ex-cc-static-bundled]       | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile and link statically to a bundled C++ library][ex-cc-static-bundled-cpp] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile a C library while setting custom defines][ex-cc-custom-defines]         | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |

[ex-semver-increment]: development_tools/versioning.html#parse-and-increment-a-version-string
[ex-semver-complex]: development_tools/versioning.html#parse-a-complex-version-string
[ex-semver-prerelease]: development_tools/versioning.html#check-if-given-version-is-pre-release
[ex-semver-latest]: development_tools/versioning.html#find-the-latest-version-satisfying-given-range
[ex-semver-command]: development_tools/versioning.html#check-external-command-version-for-compatibility
[ex-cc-static-bundled]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library
[ex-cc-static-bundled-cpp]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library-1
[ex-cc-custom-defines]: development_tools/build_tools.html#compile-a-c-library-while-setting-custom-defines

{{#include links.md}}
