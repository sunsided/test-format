use core::fmt::{Debug, Write};
use test_format::assert_debug_fmt;

#[cfg(feature = "std")]
use test_format::assert_display_fmt;

struct Test<'a>(&'a str, char, &'a str);

impl<'a> Debug for Test<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)?;
        f.write_char(self.1)?;
        f.write_str(self.2)
    }
}

#[cfg(feature = "std")]
impl<'a> std::fmt::Display for Test<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)?;
        f.write_char(self.1)?;
        f.write_str(self.2)
    }
}

#[test]
pub fn debug() {
    let input = Test("valid", ' ', "input");
    assert_debug_fmt!(input, "valid input");
}

#[test]
#[allow(clippy::should_panic_without_expect)]
#[should_panic]
pub fn debug_invalid() {
    let input = Test("valid", ' ', "inputs");
    assert_debug_fmt!(input, "valid input");
}

#[test]
#[cfg(feature = "std")]
pub fn display() {
    let input = Test("valid", ' ', "input");
    assert_display_fmt!(input, "valid input");
}

#[test]
#[allow(clippy::should_panic_without_expect)]
#[should_panic]
#[cfg(feature = "std")]
pub fn display_invalid() {
    let input = Test("valid", ' ', "inputs");
    assert_display_fmt!(input, "valid input");
}
