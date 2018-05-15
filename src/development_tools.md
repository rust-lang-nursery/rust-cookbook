# Development Tools

{{#include development_tools/debugging.md}}

## Versioning

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse and increment a version string][ex-semver-increment] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Parse a complex version string][ex-semver-complex] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check if given version is pre-release][ex-semver-prerelease] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Find the latest version satisfying given range][ex-semver-latest] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check external command version for compatibility][ex-semver-command] | [![semver-badge]][semver] | [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

## Build Time

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Compile and link statically to a bundled C library][ex-cc-static-bundled] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile and link statically to a bundled C++ library][ex-cc-static-bundled-cpp] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile a C library while setting custom defines][ex-cc-custom-defines] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |

[ex-semver-increment]: development_tools/versioning.html#parse-and-increment-a-version-string
[ex-semver-complex]: development_tools/versioning.html#parse-a-complex-version-string
[ex-semver-prerelease]: development_tools/versioning.html#check-if-given-version-is-pre-release
[ex-semver-latest]: development_tools/versioning.html#find-the-latest-version-satisfying-given-range
[ex-semver-command]: development_tools/versioning.html#check-external-command-version-for-compatibility

[ex-cc-static-bundled]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library
[ex-cc-static-bundled-cpp]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library-1
[ex-cc-custom-defines]: development_tools/build_tools.html#compile-a-c-library-while-setting-custom-defines

{{#include links.md}}
