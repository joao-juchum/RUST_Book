# Learning Rust

## Hello world
In this directory we just do a simple hello world program

## Hello cargo
Cargo is Rust’s build system and package manager.

* We can create a project using cargo new.
* We can build a project using cargo build.
* We can build and run a project in one step using cargo run.
* We can build a project without producing a binary to check for errors using cargo check.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

### Cargo.toml file
The struct is:

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

* The first line, [package], is a section heading that indicates that the following statements are configuring a package.
* The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.
* The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.