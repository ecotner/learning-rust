# Memory management

Simple program to investigate how and where memory is allocated depending on data type.
Data types with sizes known at compile time (e.g. primitive types like floats, ints, etc) are stored on the _stack_.
Data types with sizes unknown at compile time are stored on the _heap_, and a _pointer_ to the memory address is generated.

## Some other info about setting up rust projects

* rust uses a package manager called `cargo`
* you can set up a new project with `cargo new project_name`
* you can build/run a project with `cargo build` or `cargo run` from the root directory of the project
    * these create an executable file with the same name as the project directory under `target/debug/project_name` along with a bunch of other files (not sure what they all do yet)
* rust dependencies are called "crates", and you can add them to your project by putting a `dep_name = "version.number"` line in your `Cargo.toml`
    * you can find the public crate repository at [crates.io](https://crates.io)