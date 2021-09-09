//! [![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
//! [![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//! [![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ezexec-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ezexec-rust)
//! [![docs.rs](https://docs.rs/ezexec/badge.svg)](https://docs.rs/ezexec)
//! [![crates.io](https://img.shields.io/crates/v/ezexec.svg)](https://crates.io/crates/ezexec)
//! [![Download numbers](https://img.shields.io/crates/d/ezexec.svg)](https://crates.io/crates/ezexec)
//! [![dependency status](https://deps.rs/crate/ezexec/0.1.0/status.svg)](https://deps.rs/crate/ezexec/0.1.0)
//! 
//! 
//! # `ezexec`
//! Welcome to `ezexec` 🎉
//! 
//! `ezexec` provides a simple API to execute binaries or shell commands. Furthermore it implements a trivial but usually
//! good-enough API to find a binary in `PATH` or to get the current shell.
//! 
//! 
//! ## Example
//! ```rust
//! # use ezexec::{ ExecBuilder, error::Result };
//! #
//! # fn list() -> Result {
//! // Lists all files in the current directory and forwards the output to the parent's stdout
//! ExecBuilder::with_shell("ls")?
//!     .spawn_transparent()?
//!     .wait()?;
//! # Ok(())
//! # }
//! ```

#[macro_use] pub mod error;
mod lookup;
mod builder;
mod capturing_executor;
mod transparent_executor;


pub use crate::{
    builder::ExecBuilder,
    capturing_executor::CapturingExecutor,
    transparent_executor::TransparentExecutor
};