# Cross-platform `exec` for Rust

⚡️ Unix `CommandExt::exec` but isomorphic

<table align=center><td>

```rs
Command::new("cargo")
  .arg("--version")
  .cross_exec()
```

</table>

🟦 Emulates process replacement behaviour on Windows \
🐧 Uses native `CommandExt::exec` on Unix

## Installation

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

```sh
cargo add cross-exec
```

## Usage

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

Use `.cross_exec()` on a `std::process::Command` just like you would use `.exec()` on Unix.

```rs
use cross_exec::CommandExt;
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let err = Command::new("cargo")
    .arg("--version")
    .cross_exec();
  Err(err.into())
}
```

## Development

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

**Why?** \
I wanted a cross-platform way to replace the current process with a new one for wrapper programs that just end up calling another program at the end.

This project is based on the [`exec_replace`](https://docs.rs/cargo-util/latest/cargo_util/struct.ProcessBuilder.html#method.exec_replace) function from [`cargo-util`](https://docs.rs/cargo-util/latest/cargo_util/).
