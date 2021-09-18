//! Provides functions to get a binary or shell by it's name

use crate::error::Result;
use std::{
    env, borrow::Cow, ops::Deref, ffi::OsStr,
    fmt::{ self, Display, Formatter },
    path::{ Path, PathBuf }
};


/// A reference to a binary
#[derive(Debug)]
pub struct Binary {
    /// The path to the binary
    path: PathBuf
}
impl Binary {
    /// Creates a new binary with `path` if it exists
    pub fn new<P>(path: P) -> Result<Self> where P: AsRef<Path> {
        // Create self and check if the path exists
        let this = Self { path: path.as_ref().to_path_buf() };
        if !this.path.is_file() {
            Err(epath!("Binary does not exist: {}", this))?
        }

        Ok(this)
    }
    /// Finds a `binary` in `PATH`
    pub fn find<B>(binary: B) -> Result<Self> where B: AsRef<str> {
        // Get the binary string
        let binary = binary.as_ref();
        let binary = Self::binary_name(binary);

        // Iterate over the PATHs
        let paths = env::var("PATH")
            .map_err(|e| epath!("Failed to access PATH variable ({})", e))?;
        let path = env::split_paths(&paths)
            .map(|p| p.join(binary.as_ref()))
            .find(|p| p.exists())
            .ok_or(epath!("Failed to find binary in PATH: {}", binary))?;

        Ok(Self { path })
    }

    /// Creates a binary name (i.e. appends ".exe" on Windows if appropriate)
    fn binary_name(binary: &str) -> Cow<'_, str> {
        // Append ".exe" as suffix if on windows if necessary
        #[cfg(target_family = "windows")]
        match binary.ends_with(".exe") {
            true => return Cow::Borrowed(binary),
            false => return Cow::Owned(format!("{}.exe", binary))
        }

        // Take the name unchanged
        #[cfg(not(target_family = "windows"))]
        Cow::Borrowed(binary)
    }
}
impl Display for Binary {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}
impl Deref for Binary {
    type Target = OsStr;
    fn deref(&self) -> &Self::Target {
        self.path.as_os_str()
    }
}
impl AsRef<OsStr> for Binary {
    fn as_ref(&self) -> &OsStr {
        self
    }
}
impl AsRef<Path> for Binary {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}


/// A reference to a shell
#[derive(Debug)]
pub struct Shell {
    /// The underlying shell
    shell: Binary
}
impl Shell {
    /// Finds the default shell or falls back to `sh` (on unix likes) or `powershell.exe` (on windows) if available
    pub fn find() -> Result<Self> {
        // Determine the shell
        let shell = match env::var("SHELL") {
            Ok(shell) => Binary::new(shell)?,
            Err(_) => Binary::find(Self::shell_name())?
        };
        Ok(Self { shell })
    }

    /// The shell arguments required to execute a string directly
    pub fn execstring_args(&self) -> Result<&'static [&'static str]> {
        // Get the shell name
        let shell_path: &Path = self.shell.as_ref();
        let shell_name = shell_path.file_name().and_then(|n| n.to_str())
            .ok_or(epath!("Failed to get the file name for the selected shell: {}", self.shell))?;
        
        // Get the appropriate arguments
        match shell_name {
            "powershell.exe" => Ok(&["-executionpolicy", "bypass", "&"]),
            "bash" | "zsh" | "sh" => Ok(&["-c"]),
            _ => Err(epath!("Cannot get execstring arguments for unknown shell: {}", shell_name))
        }
    }

    /// Gets the platform specific default shell name (i.e. `sh` or `powershell.exe` for Windows)
    fn shell_name() -> &'static str {
        // Use `powershell.exe` on Windows
        #[cfg(target_family = "windows")]
        return "powershell.exe";

        // Use `sh` as fallback on other systems
        #[cfg(not(target_family = "windows"))]
        return "sh";
    }
}
impl Display for Shell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.shell)
    }
}
impl Deref for Shell {
    type Target = OsStr;
    fn deref(&self) -> &Self::Target {
        &self.shell
    }
}
impl AsRef<OsStr> for Shell {
    fn as_ref(&self) -> &OsStr {
        &self.shell
    }
}
impl AsRef<Path> for Shell {
    fn as_ref(&self) -> &Path {
        self.shell.as_ref()
    }
}
