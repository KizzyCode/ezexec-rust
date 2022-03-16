use crate::error::Result;
use std::process::{ Command, Child, ExitStatus, Stdio };


/// A process executor
#[derive(Debug)]
pub struct TransparentExecutor {
    /// The underlying child
    child: Child,
    /// The exit status
    exit_status: Option<ExitStatus>
}
impl TransparentExecutor {
    /// Creates a new binary executor
    pub(in crate) fn new(command: &mut Command) -> Result<Self> {
        // Set the stdio
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        // Start the child
        let child = command.spawn()
            .map_err(|e| eexec!("Failed to execute child ({})", e))?;
        Ok(Self { child, exit_status: None })
    }

    /// Closes the child's stdin
    pub fn close_stdin(&mut self) {
        self.child.stdin = None
    }
    /// Closes the child's stdout
    pub fn close_stdout(&mut self) {
        self.child.stdout = None
    }
    /// Closes the child's stderr
    pub fn close_stderr(&mut self) {
        self.child.stderr = None
    }

    /// Checks whether the child is still running or not
    pub fn is_running(&mut self) -> Result<bool> {
        // Check if we should query the process status
        if self.exit_status.is_none() {
            self.exit_status = self.child.try_wait()
                .map_err(|e| eexec!("Failed to get child exit status ({})", e))?;
        }

        // Check whether we have an exit status or not
        Ok(self.exit_status.is_none())
    }
    /// Closes stdin and stdout and waits for the child to exit
    pub fn wait(mut self) -> Result {
        // Close stdin
        self.child.stdin = None;

        // Wait for the process to exit
        let exit_status = self.exit_status.take()
            .map_or_else(|| self.child.wait(), Ok)
            .map_err(|e| eexec!("Failed to get child exit status ({})", e))?;
        
        // Read stderr if the process failed
        if !exit_status.success() {
            return Err(eexec!("Child process failed ({:?})", exit_status.code()));
        }
        Ok(())
    }
}
