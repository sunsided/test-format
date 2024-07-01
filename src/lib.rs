//! Testing of Debug and Display format implementations with support for `no_std` via
//! [`assert_debug_fmt`] and `assert_display_fmt!` macros.
//!
//! ## `std` vs `no_std`
//! This crate builds in `no_std` mode by default, allowing for testing of [`Debug`]
//! implementations. If you wish to test `Display` implementations, enable the `std` crate feature.
//!
//! ## Examples
//!
//! Assume the following type `Test` that we will use to provide some test data:
//!
//! ```
//! struct Test<'a>(&'a str, char, &'a str);
//! ```
//!
//! ```
//! # struct Test<'a>(&'a str, char, &'a str);
//! use core::fmt::{Debug, Write};
//! use test_format::assert_debug_fmt;
//!
//! impl<'a> Debug for Test<'a> {
//!     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!         f.write_str(self.0)?;
//!         f.write_char(self.1)?;
//!         f.write_str(self.2)
//!     }
//! }
//!
//! let input = Test("valid", ' ', "input");
//! assert_debug_fmt!(input, "valid input");
//! ```
//!
//! If the formatting fails, the assertion will fail:
//!
//! ```should_panic
//! # struct Test<'a>(&'a str, char, &'a str);
//! # use core::fmt::{Debug, Write};
//! # use test_format::assert_debug_fmt;
//! #
//! # impl<'a> Debug for Test<'a> {
//! #     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//! #         f.write_str(self.0)?;
//! #         f.write_char(self.1)?;
//! #         f.write_str(self.2)
//! #     }
//! # }
//! #
//! let input = Test("valid", ' ', "inputs");
//! assert_debug_fmt!(input, "valid input"); // panics
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(warnings, clippy::pedantic)]
#![warn(
    clippy::expect_used,
    clippy::missing_errors_doc,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused_qualifications
)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

use core::fmt::{Debug, Write};

/// Asserts that the [`Debug`] trait is correctly implemented.
#[macro_export]
macro_rules! assert_debug_fmt {
    ($input:expr, $expectation:expr) => {
        let input = $input;
        $crate::AssertFormat::assert_debug_fmt(&input, $expectation);
    };
}

/// Asserts that the `Display` trait is correctly implemented.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! assert_display_fmt {
    ($input:expr, $expectation:expr) => {
        let input = $input;
        $crate::AssertFormat::assert_display_fmt(&input, $expectation);
    };
}

/// Functionality for testing [`Debug`] or `Display` implementations.
#[derive(Debug)]
pub struct AssertFormat<'a> {
    /// The original string to compare.
    original: &'a str,
    /// The remaining text to compare.
    remaining: &'a str,
}

impl<'a> AssertFormat<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            original: s,
            remaining: s,
        }
    }
    /// Asserts that the `Display` trait is correctly implemented.
    ///
    /// ## Panics
    /// This call panics if the output generated by the `Display` implementation
    /// differs from the `expected` value.
    #[cfg(feature = "std")]
    pub fn assert_display_fmt<D>(instance: D, expected: &str)
    where
        D: std::fmt::Display,
    {
        let mut test = AssertFormat::new(expected);
        let _ = write!(&mut test, "{instance}");
    }

    /// Asserts that the `Debug` trait is correctly implemented.
    ///
    /// ## Panics
    /// This call panics if the output generated by the `Debug` implementation
    /// differs from the `expected` value.
    pub fn assert_debug_fmt<D>(instance: D, expected: &str)
    where
        D: Debug,
    {
        let mut test = AssertFormat::new(expected);
        let _ = write!(&mut test, "{instance:?}");
    }
}

impl<'a> Write for AssertFormat<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = self.original;
        if self.remaining.starts_with(s) && self.remaining.len() >= s.len() {
            self.remaining = &self.remaining[s.len()..];
        } else {
            let position = self.original.len() - self.remaining.len();
            panic!(
                "assertion failed: Expected \"{}\" but found \"{}\" starting at position {}",
                self.original, s, position
            );
        }
        Ok(())
    }
}
