use std::{
    io,
    process::{self, Command},
};

pub trait CommandExt {
    ///
    /// On Unix, this will call [`std::os::unix::process::CommandExt::exec`].
    ///
    /// On Windows, this will:
    ///
    /// 1. Set a `Ctrl+C` handler that ignores all `Ctrl+C` events and lets the child process handle them.
    /// 2. Run the command with [`Command::status`]. If it fails to start, return the error.
    /// 3. Call [`process::exit`] with the exit code of the child process.
    ///
    /// On success this function will not return, and otherwise it will return an error indicating why the exec (or another part of the setup of the [`Command`]) failed.
    ///
    /// `cross_exec` not returning has the same implications as calling [`process::exit`] – no destructors on the current stack or any other thread’s stack will be run. Therefore, it is recommended to only call `cross_exec` at a point where it is fine to not run any destructors.
    ///
    /// This function, unlike `spawn`, will not fork the process to create a new child. Like spawn, however, the default behavior for the stdio descriptors will be to inherit them from the current process.
    ///
    /// # Notes
    ///
    /// The process may be in a "broken state" if this function returns in error. For example the working directory, environment variables, signal handling settings, various user/group information, or aspects of stdio file descriptors may have changed. If a "transactional spawn" is required to gracefully handle errors it is recommended to use the cross-platform spawn instead.
    #[must_use]
    fn cross_exec(&mut self) -> io::Error;
}

impl CommandExt for Command {
    #[allow(unreachable_code)]
    fn cross_exec(&mut self) -> io::Error {
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            return self.exec();
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            use windows::{Win32::System::Console::SetConsoleCtrlHandler, core::BOOL};

            // Ignore Ctrl+C in this process so that the child process can handle it.
            unsafe extern "system" fn ignore_all(_: u32) -> BOOL {
                true.into()
            }
            let res = unsafe { SetConsoleCtrlHandler(Some(ignore_all), true.into()) };
            if let Err(e) = res {
                return io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to set Ctrl+C handler: {}", e),
                );
            }
        }

        // No ? because return type isn't Result or Option,
        // it's only the E part of Result<T, E>.
        let res = self.status();
        let status = match res {
            Ok(s) => s,
            Err(e) => return e,
        };
        process::exit(status.code().unwrap_or(128));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_example_cargo_version() -> Result<(), Box<dyn Error>> {
        let expected = {
            let output = Command::new("cargo").arg("--version").output()?;
            let stdout = String::from_utf8(output.stdout)?;
            format!("Hello from before cross_exec!\n{}", stdout)
        };
        let actual = {
            let output = Command::new("cargo")
                .args(&["run", "--example", "cargo-version"])
                .output()?;
            String::from_utf8(output.stdout)?
        };
        assert_eq!(expected, actual);
        Ok(())
    }
}
