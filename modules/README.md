Packages: A Cargo feature that let you build, test, and share crates

Crates: A tree of modules that produces a library or executable

Modules and use: let you control and organization, scope, and privacy of paths

Paths: A way of naming an item, such as struct, function, or module

###Packages and Crates

A crate is a binary or library.

A package is one or more crates that provide a set of functionality. A package contains
a `Cargo.toml` file that describes how to build crates.

A package must contains zero or one library crates, and no more.


`cargo new project` -> create