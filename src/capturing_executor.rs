use crate::error::{ Result, Error };
use std::{
    convert::TryFrom,
    io::{ self, Read, Write, ErrorKind },
    process::{ Command, Child, ExitStatus, Stdio }
};


/// A process executor
#[derive(Debug)]
pub struct CapturingExecutor {
    /// The underlying child
    child: Child,
    /// The exit status
    exit_status: Option<ExitStatus>
}
impl CapturingExecutor {
    /// Creates a new binary executor
    pub(in crate) fn new(command: &mut Command) -> Result<Self> {
        // Set the stdio
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

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
    /// Closes stdin and waits for the child to exit
    pub fn wait(mut self) -> Result {
        // Close stdin
        self.child.stdin = None;

        // Wait for the process to exit
        let exit_status = self.exit_status.take()
            .map_or_else(|| self.child.wait(), Ok)
            .map_err(|e| eexec!("Failed to get child exit status ({})", e))?;
        
        // Read stderr if the process failed
        if !exit_status.success() {
            // Read stderr
            let mut stderr_bytes = Vec::new();
            if let Some(mut stderr) = self.child.stderr {
                stderr.read_to_end(&mut stderr_bytes)
                    .map_err(|e| eexec!("Failed to read child stderr ({})", e))?;
            }

            // Throw the child error
            let stderr_string = String::from_utf8_lossy(&stderr_bytes);
            Err(eexec!("Child process failed ({:?}): {}", exit_status.code(), stderr_string))?
        }
        Ok(())
    }
}
impl TryFrom<CapturingExecutor> for Vec<u8> {
    type Error = Error;

    /// Closes stdin, reads everything in stdout and waits for the child to exit
    fn try_from(mut executor: CapturingExecutor) -> Result<Self> {
        // Close stdin
        executor.close_stdin();
        
        // Read stdout
        let mut stdout = Vec::new();
        executor.read_to_end(&mut stdout)
            .map_err(|e| echild!("Failed to read stdout from child ({})", e))?;
        
        // Wait for the child to exit
        executor.wait()?;
        Ok(stdout)
    }
}
impl TryFrom<CapturingExecutor> for String {
    type Error = Error;

    /// Closes stdin, reads everything in stdout as `String` and waits for the child to exit
    fn try_from(executor: CapturingExecutor) -> Result<Self> {
        let stdout = Vec::try_from(executor)?;
        String::from_utf8(stdout).map_err(|e| echild!("Cannot read child's stdout as string ({})", e))
    }
}
impl Read for CapturingExecutor {
    fn read(&mut self, buf: &mut[u8]) -> io::Result<usize> {
        let stdout = self.child.stdout.as_mut()
            .ok_or(ErrorKind::BrokenPipe)?;
        stdout.read(buf)
    }
}
impl Write for CapturingExecutor {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let stdin = self.child.stdin.as_mut()
            .ok_or(ErrorKind::BrokenPipe)?;
        stdin.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        let stdin = self.child.stdin.as_mut()
            .ok_or(ErrorKind::BrokenPipe)?;
        stdin.flush()
    }
}
