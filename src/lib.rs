//! Expanding tildes in paths.
//!
//! # Example
//!
//! ```
//! use expand_tilde::{ExpandTilde, expand_tilde};
//!
//! let path = "~/.config";
//!
//! let direct = expand_tilde(path).unwrap();
//!
//! let extended = path.expand_tilde().unwrap();
//!
//! assert_eq!(direct, extended);
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::path::{Path, PathBuf};

use home::home_dir;
use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during `~` expansion.
///
/// The only error that can occur is if the home directory cannot be found.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to expand")]
#[diagnostic(code(expand_tilde), help("make sure the home directory can be found"))]
pub struct Error;

/// The result type used by this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The `~` literal.
pub const TILDE: &str = "~";

/// Expands the tilde (`~`) component to the home directory.
///
/// # Errors
///
/// Returns an error if the home directory cannot be found.
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    fn expand_tilde_inner(path: &Path) -> Result<PathBuf> {
        path.strip_prefix(TILDE).map_or_else(
            |_| Ok(path.to_owned()),
            |stripped| home_dir().map(|dir| dir.join(stripped)).ok_or(Error),
        )
    }

    expand_tilde_inner(path.as_ref())
}

mod private {
    pub trait Sealed {}
}

/// Represents paths that can be expanded (sealed extension trait).
pub trait ExpandTilde: private::Sealed {
    /// Expands the tilde (`~`) component to the home directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory cannot be found.
    fn expand_tilde(&self) -> Result<PathBuf>;
}

impl<P: AsRef<Path>> private::Sealed for P {}

impl<P: AsRef<Path>> ExpandTilde for P {
    fn expand_tilde(&self) -> Result<PathBuf> {
        expand_tilde(self)
    }
}
