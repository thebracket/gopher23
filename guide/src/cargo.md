# About Cargo

`Cargo` is Rust's Swiss-Army Knife tool. Cargo does a *lot* of things, and is extensible through a plugin system. Here are some of the more common Cargo commands you will use:

## Built-In Commands

### Creating Projects

Command | Go Equivalent | Description
--- | --- | ---
`cargo new` | `go mod init` | Create a new project.
`cargo init` | `go mod init` | Create a new project (alias for `cargo new`)
`cargo new --lib` | `go mod init` | Create a new library project.
`cargo new --vcs none` | - | Create a new project without `git` integration.

### Building Projects

> Cargo isn't a compiler. Cargo is more like `cmake` in the C++ world in that it builds instructions for building your project (using `rustc`), and issues the build commands.

Command | Go Equivalent | Description
--- | --- | ---
`cargo build` | `go build -gcflags "all=-N -l" github.com/my/package` | Build the project in `debug` mode. No optimizations, debug information included in the binary. Also includes extra runtime checks for integer overflow/wrapping.
`cargo build --release` | `go build` | Build the project in `release` mode. Optimized, no debug information included in the binary. No extra runtime checks.
`cargo run` | `go run -gcflags "all=-N -l"` | Build the project and execute it in `debug` mode.
`cargo run --release` | `go run` | Build the project and execute it in `release` mode.
`cargo run -- --param1` | - | Build the project and execute it in `debug` mode, passing `--param1` to the binary.
`cargo check` | - | Check the project for errors, but do not build it. This is very fast, and is useful for quickly checking if the project compiles. Most IDEs run this regularly.
`cargo clean` | `go clean` | Remove the `target` directory. This is useful if you want to ensure that you are building from scratch, or if you want to remove the `target` directory to save space.

### Maintaining Projects

Command | Go Equivalent | Description
--- | --- | ---
`cargo clippy` | `go vet` | Run the Clippy linter, which will advise you on potential issues and suggest improvements.
`cargo fmt` | `go fmt` | Run the Rustfmt formatter, which will format your code according to the Rust Style Guide. You can customize the style guide by creating a `.rustfmt.toml` file in your project directory.
`cargo fix` | `go fix` | Run the Rustfix tool, which will automatically fix some common errors.

### Testing Projects

Command | Go Equivalent | Description
--- | --- | ---
`cargo test` | `go test` | Run all unit tests in the project. Unlike Go, unit tests by default are included in the source files they are testing.

### Benchmarking Projects

Command | Go Equivalent | Description
--- | --- | ---
`cargo bench` | `go test -bench .` | Run all benchmarks in the project.

### Documenting Projects

Command | Go Equivalent | Description
--- | --- | ---
`cargo doc` | `go doc` | Generate documentation for the project. The documentation will be in `target/doc`.

### Managing Dependencies

Command | Go Equivalent | Description
--- | --- | ---
`cargo search` | - | Search [crates.io](https://crates.io) for a crate.
`cargo add` | - | Add a dependency to the project. This is the same as adding it to the `[dependencies]` section of `Cargo.toml`
`cargo tree` | - | Display a tree of all of your project dependencies, showing your dependencies' dependencies.
`cargo update` | - | Update all dependencies to the latest version.
`cargo update --aggressive` | - | Update all dependencies to the latest version, even if the latest version breaks the build.
`cargo update --dry-run` | - | Show what would change without applying it.
`cargo publish` | - | Publish your crate to [crates.io](https://crates.io).
`cargo publish --dry-run` | - | Show what would change without applying it.
`cargo vendor` | - | Download all dependencies and give you some commands to add to `Cargo.toml` to use the vendored dependencies. Once vendored, dependencies are not downloaded from the internet, but are instead downloaded from the local filesystem.

## Third-Party Commands

You can also use Cargo to install third-party commands. Some popular cargo extensions include:

* `cargo install mdbook` installs the `mdbook` tool that was used to create this workbook.
* `cargo install cargo-audit` installs a tool that can check your dependencies for security vulnerabilities.
* `cargo install cargo-deny` installs a tool that can apply policies to your dependencies. Deny by license, deny by vulnerability, deny by crate name, etc.
* `cargo install cargo-watch` gives you a handy tool that will watch your project for changes and automatically rebuild it.
* `cargo install cargo-edit` gives you a handy tool that will let you add, remove, and upgrade dependencies from the command line.

And so on---there are a [lot of available plugins](https://crates.io/categories/development-tools::cargo-plugins)!
