//! Expanding tildes in paths.
//!
//! If the path starts with the `~` character, it will be expanded to the home directory.
//!
//! Any presence of `~` in paths except for the first character will not be expanded.
//!
//! The home directory is provided by the [`home_dir`] function.
//!
//! # Example
//!
//! There are two main ways to expand tildes with this crate;
//! the first is to use the [`expand_tilde`] with references:
//!
//! ```
//! use expand_tilde::expand_tilde;
//!
//! let path = "~/.config";
//!
//! let expanded = expand_tilde(path).unwrap();
//!
//! println!("{}", expanded.display());  // something like `/home/nekit/.config`
//! ```
//!
//! And the other way is to use the sealed extension trait:
//!
//! ```
//! use expand_tilde::ExpandTilde;
//!
//! let path = "~/.config";
//!
//! let expanded = path.expand_tilde().unwrap();
//!
//! println!("{}", expanded.display());  // something like `/home/nekit/.config`
//! ```
//!
//! The latter method simply calls the former one under the hood.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during `~` expansion.
///
/// The only error that can occur is if the home directory cannot be found.
#[derive(Debug, Error)]
#[cfg_attr(feature = "diagnostics", derive(Diagnostic))]
pub enum Error {
    /// The home directory cannot be found.
    #[error("home directory not found")]
    #[cfg_attr(
        feature = "diagnostics",
        diagnostic(
            code(expand_tilde::not_found),
            help("make sure the home directory exists")
        )
    )]
    NotFound,
    /// The home directory is empty.
    #[error("home directory is empty")]
    #[cfg_attr(
        feature = "diagnostics",
        diagnostic(
            code(expand_tilde::empty),
            help("make sure the home directory is non-empty")
        )
    )]
    Empty,
}

/// The result type used by this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The `~` literal.
pub const TILDE: &str = "~";

/// Wraps [`home::home_dir`] to improve diagnostics.
///
/// # Errors
///
/// Returns:
///
/// - [`Error::NotFound`] if the home directory cannot be found.
/// - [`Error::Empty`] if the home directory is empty.
pub fn home_dir() -> Result<PathBuf> {
    let dir = home::home_dir().ok_or(Error::NotFound)?;

    if dir.as_os_str().is_empty() {
        Err(Error::Empty)
    } else {
        Ok(dir)
    }
}

/// Expands the tilde (`~`) component of the given path to the home directory.
///
/// # Errors
///
/// This function propagates errors returned by [`home_dir`].
pub fn expand_tilde<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Cow<'_, Path>> {
    fn expand_tilde_inner(path: &Path) -> Result<Cow<'_, Path>> {
        path.strip_prefix(TILDE).map_or_else(
            |_| Ok(Cow::Borrowed(path)),
            |stripped| home_dir().map(|dir| Cow::Owned(dir.join(stripped))),
        )
    }

    expand_tilde_inner(path.as_ref())
}

/// Similar to [`expand_tilde`], but accepts the path by value and returns owned paths.
///
/// # Errors
///
/// This function propagates errors returned by [`expand_tilde`].
pub fn expand_tilde_owned<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    expand_tilde(path.as_ref()).map(Cow::into_owned)
}

mod sealed {
    pub trait Sealed {}
}

/// Represents values that can be tilde-expanded (sealed extension trait).
pub trait ExpandTilde: sealed::Sealed {
    /// Expands the tilde (`~`) component to the home directory.
    ///
    /// # Errors
    ///
    /// See [`expand_tilde`] for more information.
    fn expand_tilde(&self) -> Result<Cow<'_, Path>>;
}

impl<P: AsRef<Path>> sealed::Sealed for P {}

impl<P: AsRef<Path>> ExpandTilde for P {
    fn expand_tilde(&self) -> Result<Cow<'_, Path>> {
        expand_tilde(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{expand_tilde, ExpandTilde};

    #[test]
    fn consistent() {
        let path = "~/.config";

        let expanded = expand_tilde(path).unwrap();
        let extended = path.expand_tilde().unwrap();

        assert_eq!(expanded, extended);
    }
}
