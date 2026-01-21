# `.cross_exec()` for Rust

⚡️ Isomorphic `CommandExt::exec` for Unix and Windows

## Installation

## Usage

```rs
use cross_exec::CommandExt;
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Command::new("cargo")
        .arg("--version")
        .cross_exec()?;
}
```

## Development

Why? I wanted a cross-platform way to assimilate or replace the current process with a new one for executable bootstrap wrappers (download, install, self-replace, and then `exec` the real binary).

This project is based on the `exec_replace` function from `cargo-utils`. https://docs.rs/cargo-util/latest/cargo_util/struct.ProcessBuilder.html#method.exec_replace
