use crate::{
    error::Result, capturing_executor::CapturingExecutor, transparent_executor::TransparentExecutor,
    lookup::{ Binary, Shell }
};
use std::{ iter, ffi::OsStr, path::Path, process::Command };


/// A child executor builder
#[derive(Debug)]
pub struct ExecBuilder {
    /// The underlying command
    command: Command
}
impl ExecBuilder {
    /// Creates a new builder for `binary` (requires a valid path; does not search `PATH` for `binary`)
    pub fn with_path<B, A, AS>(binary: B, args: A) -> Result<Self>
        where B: AsRef<Path>, A: IntoIterator<Item = AS>, AS: AsRef<OsStr>
    {
        // Create the command
        let binary = Binary::new(binary)?;
        let mut command = Command::new(&binary);
        command.args(args);

        Ok(Self { command })
    }
    /// Looks for `binary` in `PATH` and creates a new builder
    pub fn with_name<B, A, AS>(binary: B, args: A) -> Result<Self>
        where B: AsRef<str>, A: IntoIterator<Item = AS>, AS: AsRef<OsStr>
    {
        // Create the command
        let binary = Binary::find(binary)?;
        let mut command = Command::new(&binary);
        command.args(args);

        Ok(Self { command })
    }
    /// Returns a new shell-executed based command
    pub fn with_shell<P>(command: P) -> Result<Self> where P: AsRef<str> {
        // Get the shell parameters
        let shell = Shell::find()?;
        let execstring_args = shell.execstring_args()?;
        
        // Create the arguments
        let command = command.as_ref();
        let args = execstring_args.into_iter()
            .chain(iter::once(&command));
        Self::with_path(shell, args)
    }

    /// Sets the process working dir for the child
    pub fn set_pwd<P>(&mut self, pwd: P) -> &mut Self where P: AsRef<Path> {
        self.command.current_dir(pwd);
        self
    }

    /// Sets the environment variables for the child
    pub fn set_envs<I, K, V>(&mut self, envs: I) -> &mut Self
        where I: IntoIterator<Item = (K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>
    {
        self.command.envs(envs);
        self
    }
    /// Sets a single environment variable for the child
    pub fn set_env<K, V>(&mut self, key: K, value: V) -> &mut Self where K: AsRef<OsStr>, V: AsRef<OsStr> {
        self.command.env(key, value);
        self
    }

    /// Spawn the child and capture the output
    pub fn spawn_captured(self) -> Result<CapturingExecutor> {
        CapturingExecutor::new(self.command)
    }
    /// Spawn the child and inherit stdin/-out/-err from the parent process
    pub fn spawn_transparent(self) -> Result<TransparentExecutor> {
        TransparentExecutor::new(self.command)
    }
}
