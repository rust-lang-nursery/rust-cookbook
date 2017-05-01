# Contributing to the Rust Cookbook

Have something useful to add to the Rust Cookbook? We'd love to have it!

This document contains information and guidelines that you should read before 
contributing to the project. If you think something in this document should change,
feel free propose a change in a pull request.

## Table of Contents
* [Getting Started](#getting-started)  
* [How to Contribute](#how-to-contribute)  
* [Crates](#crates)  
* [Tests](#tests)  
* [Style](#style)
    * [Git Commit Messages](#git-commit-messages)  
    * [Snippet Style](#snippet-style)  

## Getting Started
TODO: Mention Trello and how to join (if we keep using it)

## How to Contribute
TODO: Reporting bugs  
TODO: Project page suggestions  
TODO: Fixing bugs  
TODO: Pull requests  

## Crates

All crates used by the cookbook are listed in Cargo.toml. If you are
adding a new crate, first add it there.

## Tests

All examples are tested using [`skeptic`]. Run them with `cargo test`.

[`skeptic`]: https://github.com/brson/rust-skeptic

## Style

### Git Commit Messages
https://chris.beams.io/posts/git-commit/  
TODO: Possibly take relevant parts from this post or write our own

### Snippet Style

Snippets are intended to be read by complete beginners, and copied
into projects for experimentation. They should follow best practices.

Snippets should have a simple single-sentence description
that describes something a typical Rust user typically wants to do.

The snippet should have minimal code that doesn't directly support the
description of the snippet. Keep extra functions and types to a
minimum.

Follow the template in [src/error_handling_note.md]. Examples always
set up error handling correctly and propagate errors with `?` (not
`try!`).

Don't use glob imports, even for preludes.

Sort imports.

Examples should be simple and obvious enough that an experienced dev
won't need comments. Explanation should be in prose.

Describe the code in prose, not in comments. Things that should be
described include any traits imported and their methods used. Think
about what information here supports the use case and might not be
obvious to someone new. Say the minimum possible about aspects that
don't directly support the use case. See [basics.md] for examples.

Hyperlink all references to APIs, and style them as `code`.

