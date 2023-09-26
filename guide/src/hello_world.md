# Hello World

If you've played with Rust a bit, you'll have seen this. But let's start at the beginning, and make sure your toolchains are good to go.

You should have Rust installed via [https://RustUp.rs](https://RustUp.rs) - you'll need it, and a text editor to proceed.

## Creating a New Project

At a terminal, type:

```bash
cargo new hello_world
```

This will create a new directory called `hello_world` with a basic Rust project inside. Let's take a look at what's in there:

File | Description
--- | ---
Cargo.toml | The project manifest, similar to `go.mod`. It describes the project, any build customization and its dependencies.
src/main.rs | The main source file for the project.

After you've built a project for the first time, you'll see a few more files:

File | Description
--- | ---
Cargo.lock | A lockfile, similar to `go.sum`. It describes the exact versions of dependencies used in the project.
target/ | The build directory. Similar to `go build` or `go install`, `cargo build` will build the project and place the output here.

The `main.rs` file gives you "Hello World" by default:

```rust
fn main() {
    println!("Hello, world!");
}
```

This should be pretty straightforward. Notice:

* `fn` means "function". It's just like `func main` in Go - your program starts here.
* `println!` is the same as `fmt.PrintLn` in Go - it prints a line to the console.
* You don't have to import anything to access `println!`.
* Rust uses `!` to indicate a macro - a function that generates code at compile time. Printing is surprisingly convoluted, so Rust uses its macro system to permit more flexible code than the regular function syntax.

> You can run the code by typing `cargo run`. This compiles and executes a *debug build*---no optimizations are performed, debugger symbols are generated. The executable is placed in `target/debug/hello_world`.