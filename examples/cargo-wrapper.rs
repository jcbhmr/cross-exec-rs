use cross_exec::CommandExt;
use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from before cross_exec!");
    let mut cmd = Command::new("cargo");
    // #[cfg(unix)]
    // {
    //     use std::os::unix::process::CommandExt;
    //     if let Some(arg0) = env::args_os().next() {
    //         cmd.arg0(arg0);
    //     }
    // }
    let err = cmd.args(env::args_os().skip(1)).cross_exec();
    Err(err.into())
}
