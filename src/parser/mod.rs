
// A simple regex parser struct, with its own internal state.
pub struct RegexParser {
    escaped: bool,    // State variable, whether the next character is escaped or not.
    extended: bool,   // Initial setting, whether ERE or BRE should be used. If set, use ERE.
}

// An enum representing the possible types of a parsed character.
pub enum ParseResult {
    Character(char),  // A normal character was read. May be '\n'.
    Special(char),    // A special character was read.
    Nothing,          // A (state-modifying) character that must not be processed further.
    BadPattern,       // A character that indicates an invalid regex pattern.
}

impl RegexParser {
    // Creates a new parser. The parser will use ERE if the extended parameter is set.
    pub fn new(extended: bool) -> RegexParser {
        return RegexParser { escaped: false, extended: extended };
    }

    // Parse a character. The parser will work based on its own state that may have been
    // modified through previous parse calls.
    pub fn parse(self, c: char) -> ParseResult {
        // Toggle escaping off.
        let is_escaped = self.escaped;
        if self.escaped {
            self.escaped = false;
        }

        match c {
            // A '\' may toggle the escaped property.
            '\\' => if is_escaped {
                return ParseResult::Character(c);
            } else {
                self.escaped = true;
                return ParseResult::Nothing;
            }

            // A '\n' can only be parsed if it isn't escaped.
            '\n' => if is_escaped {
                return ParseResult::BadPattern;
            } else {
                return ParseResult::Character(c);
            }

            // These characters behave the same way in BRE and ERE.
            // If they aren't escaped, they have special meaning.
            '.' | '[' | '^' | '$' | '*' => if is_escaped {
                return ParseResult::Character(c);
            } else {
                return ParseResult::Special(c);
            }

            // These characters have their behavior flipped in BRE.
            // They have special meaning when escaped in BRE, or not escaped in ERE.
            '+' | '?' | '(' | '|' | '{' => if is_escaped ^ self.extended {
                return ParseResult::Special(c);
            } else {
                return ParseResult::Character(c);
            }

            // Any other escaped character causes an error with the exception of
            // the "\n" character sequence.
            _ => if is_escaped {
                return match c {
                    'n' => return ParseResult::Character('\n'),
                    _ => return ParseResult::BadPattern,
                }
            } else {
                return ParseResult::Character(c);
            }
        }
    }
}
