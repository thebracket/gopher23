# Workspaces

We're going to build our code inside a `workspace`. Workspaces are a handy way to group multiple Rust projects together. Workspace projects share an output `target` directory, and all compilation artifacts are shared.

> Using workspaces is a great way to conserve disk space! When I was writing the Rust Roguelike Tutorial---which features 80 projects, all of which use the same libraries---without a workspace I was using several gigabytes of disk space to build the project. With a workspace, I was able to reduce that to a few hundred megabytes.

Workspaces also reduce compile times. Incremental builds are shared between workspace members, so if you change a library that is used by multiple projects, you don't have to recompile it for each project.

## Creating a Workspace

Open up `Cargo.toml` and add a new section:

```toml
[workspace]
members = [
]
```

This tells Cargo that this is a workspace, and that it should look for other projects in the `members` list. Let's add a new project:

```toml
[workspace]
members = [
    "hello_again",
]
```

In your terminal, make a second project (from inside the first):

```bash
cargo new hello_again
```

You now have two projects - but if you compile and run the second project as follows:

```bash
cd hello_again
cargo run
```

You'll notice that you only have the one `target` directory. This is really helpful when you are building libraries and consumers together, or working with a lot of dependencies---you don't have to recompile the dependencies every time, and you only download them once.

> Workspaces share compilation flags. If you need to customize your build, you can do so in the `Cargo.toml` of the workspace, and it will apply to all projects.