//! Parse environment variables by defining a struct.
//!
//! ## Example
//! ```no_run
//! use envopt::EnvOpt;
//!
//! #[derive(EnvOpt)]
//! pub enum EnvOpts {
//!     #[envopt(name = "FOO")]
//!     Foo,
//!     #[envopt(name = "BAR", default = "default-bar")]
//!     Bar,
//! }
//!
//! pub fn main() {
//!     EnvOpts::validate_or_exit();
//!
//!     println!("FOO: {}", EnvOpts::Foo.value_or_exit());
//!     println!("BAR: {}", EnvOpts::Bar.value_or_exit());
//! }
//! ```

#[doc(hidden)]
pub use envopt_derive::*;

/// A struct that is converted from environment variables.
pub trait EnvOpt {
    /// Validate all configured environment variables.
    ///
    /// Variants without a default cause an error.
    fn validate() -> Result<(), Vec<String>>;

    /// Validate all configured environment variables.
    ///
    /// If variants without a default are missing exit.
    fn validate_or_exit() {
        match Self::validate() {
            Ok(()) => {}
            Err(errors) => {
                for error in errors {
                    eprintln!("{}", error);
                }

                std::process::exit(64);
            }
        }
    }

    /// Get the value for a variant.
    ///
    /// If it is missing the default value is returend.
    /// If no default is defined an error is returned.
    fn value(&self) -> Result<String, String>;

    /// Get the value for a variant.
    ///
    /// If it is missing the default value is returend.
    /// If no default is defined exit.
    fn value_or_exit(&self) -> String {
        match self.value() {
            Ok(val) => val,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(64);
            }
        }
    }
}
