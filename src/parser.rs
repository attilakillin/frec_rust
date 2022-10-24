use crate::flags::CompileFlags;

pub struct Parser<'p> {
    pattern: &'p str,
    flags: CompileFlags
}

pub enum ParseResult {
    UseLiteral,
    UseLongest,
    UsePrefix,
    UseNothing,
}

impl<'p> Parser<'p> {
    pub fn new(pattern: &str, flags: CompileFlags) -> Parser {
        return Parser { pattern, flags };
    }

    pub fn determine_type(&self) -> ParseResult {
        let mut is_literal = true;
        let mut is_longest = true;

        let mut state_escaped = false;

        for c in self.pattern.chars() {
            if c == '\\' {
                state_escaped = !state_escaped;
            }

            match c {
                '.' | '[' | '^' | '$' | '*' => is_literal &= state_escaped,
                '+' | '?' | '(' | '|' | '{' => is_literal &= !(state_escaped ^ self.flags.use_extended),
                _ => (),
            }

            match c {
                '*' => is_longest &= state_escaped,
                '{' | '+' | '?' | '|' => is_longest &= !(state_escaped ^ self.flags.use_extended),
                _ => (),
            }
        }

        if is_literal {
            return ParseResult::UseLiteral;
        } else if is_longest {
            return ParseResult::UseLongest;
        } else {
            return ParseResult::UseNothing;
        }
    }
}
