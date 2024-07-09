use core::fmt::{Debug, Write};
use test_format::assert_debug_fmt;
use test_format::assert_display_fmt;

struct Test<'a>(&'a str, char, &'a str);

impl<'a> Debug for Test<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)?;
        f.write_char(self.1)?;
        f.write_str(self.2)
    }
}

impl<'a> core::fmt::Display for Test<'a> {
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
#[should_panic(
    expected = r#"Expected "valid input" but found "inputs" starting at position 6: mismatch at position 11."#
)]
pub fn debug_invalid() {
    let input = Test("valid", ' ', "inputs");
    assert_debug_fmt!(input, "valid input");
}

#[test]
#[should_panic(
    expected = r#"Expected "valid input" but found "-" starting at position 5: mismatch at position 5."#
)]
pub fn debug_invalid_2() {
    let input = Test("valid", '-', "input");
    assert_debug_fmt!(input, "valid input");
}

#[test]
#[should_panic(expected = r#"Expected "valid input" but got "valid": missing 6 characters."#)]
pub fn debug_invalid_too_short() {
    let input = Test("valid", ' ', "");
    assert_debug_fmt!(input, "valid input");
}

#[test]
#[should_panic(
    expected = r#"Expected "valid input" but found "valid inputs would be nice" starting at position 0: mismatch at position 11."#
)]
pub fn debug_invalid_too_long() {
    let input = Test("valid inputs would be nice", ' ', "");
    assert_debug_fmt!(input, "valid input");
}

#[test]
pub fn display() {
    let input = Test("valid", ' ', "input");
    assert_display_fmt!(input, "valid input");
}

#[test]
#[should_panic(
    expected = r#"Expected "valid input" but found "inputs" starting at position 6: mismatch at position 11."#
)]
pub fn display_invalid() {
    let input = Test("valid", ' ', "inputs");
    assert_display_fmt!(input, "valid input");
}

#[test]
#[should_panic(
    expected = r#"Expected "valid input" but found "-" starting at position 5: mismatch at position 5."#
)]
pub fn display_invalid_2() {
    let input = Test("valid", '-', "inputs");
    assert_display_fmt!(input, "valid input");
}

#[test]
#[should_panic(expected = r#"Expected "valid input" but got "valid in": missing 9 characters."#)]
pub fn display_invalid_too_short() {
    assert_display_fmt!("val", "valid input");
}

#[test]
#[should_panic(
    expected = r#"Expected "valid input" but found "valid inputs would be nice" starting at position 0: mismatch at position 11."#
)]
pub fn display_invalid_too_long() {
    assert_display_fmt!("valid inputs would be nice", "valid input");
}
