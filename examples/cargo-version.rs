use cross_command_exec::CommandExt;
use std::{
    error::Error,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from before cross_exec!");
    let err = Command::new("cargo").arg("--version").cross_exec();
    Err(err.into())
}
