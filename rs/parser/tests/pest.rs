#![cfg(test)]

use glicol_parser::{GlicolParser, Rule};
use pest::Parser;

#[test]
fn test_line_endings() {
    // Unix style (LF)
    let unix_input = "a: sin 440\nb: saw 220";
    let unix_result = GlicolParser::parse(Rule::block, unix_input);
    assert!(unix_result.is_ok());

    // Windows style (CRLF)
    let windows_input = "a: sin 440\r\nb: saw 220";
    let windows_result = GlicolParser::parse(Rule::block, windows_input);
    assert!(windows_result.is_ok());

    // Mixed line endings
    let mixed_input = "a: sin 440\nb: saw 220\r\nc: squ 110";
    let mixed_result = GlicolParser::parse(Rule::block, mixed_input);
    assert!(mixed_result.is_ok());
}
