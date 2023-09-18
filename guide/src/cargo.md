# About Cargo

`Cargo` is Rust's Swiss-Army Knife tool. Cargo does a *lot* of things, and is extensible through a plugin system. Here are some of the more common Cargo commands you will use:

## Built-In Commands

### Creating Projects

* `cargo new` (`cargo init` is an alias): create a new project. If you append `--lib` to the command, it will create a library project instead of a binary project.

### Building Projects

> Cargo isn't a compiler. Cargo is more like `cmake` in the C++ world in that it builds instructions for building your project (using `rustc`), and issues the build commands.

* `cargo build`: build the project. If you append `--release` to the command, it will build the project in release mode, which will optimize the code. If you append `--verbose` to the command, it will print out more information about the build process. The compiled program/library will be in `target/debug` or `target/release` depending on the build mode.
* `cargo run`: build and run the project. If you append `--release` to the command, it will build the project in release mode, which will optimize the code. If you append `--verbose` to the command, it will print out more information about the build process.
* `cargo check`: check the project for errors, but do not build it. This is faster than `cargo build` and is useful for quickly checking if the project compiles.

### Maintaining Projects

* `cargo clippy`: run the Clippy Linter against your project, warning of potential issues and making suggestions.
* `cargo fmt`: run the Rustfmt Formatter against your project, formatting your code according to the Rust Style Guide.
* `cargo fix`: run the Rustfix tool against your project, automatically fixing some common errors.

### Testing Projects

* `cargo test`: run all unit tests in the project.

### Benchmarking Projects

* `cargo bench`: run all benchmarks in the project.

### Documenting Projects

* `cargo doc`: generate documentation for the project. The documentation will be in `target/doc`.

### Managing Dependencies

* `cargo search`: search for a crate on [crates.io](https://crates.io).
* `cargo add`: add a dependency to the project. If you append `--dev` to the command, it will add the dependency to the development dependencies instead of the normal dependencies. You can use the `-F` flag to specify a *feature flag*.
* `cargo vendor`: download all dependencies and give you some commands to add to `Cargo.toml` to use the vendored dependencies. Once vendored, dependencies are not downloaded from the internet, but are instead downloaded from the local filesystem. I used this for the GopherCon project to ensure that the project would build even if the internet connection was down.
* `cargo update`: update all dependencies to the latest version. If you append `--aggressive` to the command, it will update all dependencies to the latest version, even if the latest version breaks the build. You can also use `--dry-run` to see what would change without applying it.
* `cargo publish`: publish your crate to [crates.io](https://crates.io). You can use `--dry-run` to see what would change without applying it.

## Third-Party Commands

You can also use Cargo to install third-party commands. For example:

* `cargo install mdbook` installs the `mdbook` tool that was used to create this workbook.
* `cargo install cargo-audit` installs a tool that can check your dependencies for security vulnerabilities.
* `cargo install cargo-deny` installs a tool that can apply policies to your dependencies. Deny by license, deny by vulnerability, deny by crate name, etc.
* `cargo install cargo-watch` gives you a handy tool that will watch your project for changes and automatically rebuild it.
* `cargo install cargo-edit` gives you a handy tool that will let you add, remove, and upgrade dependencies from the command line.

And so on---there are a *lot* of available plugins!
