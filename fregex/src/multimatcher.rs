use crate::{
    matcher::{Matcher},
    matchers::{LiteralMatcher, LongestMatcher, PrefixMatcher, NothingMatcher},
    multimatchers::{LiteralMultiMatcher, LongestMultiMatcher, NothingMultiMatcher}, 
    preprocessor::{Preprocessor, Suggestion},
    types::{Error, Match},
};

pub struct MultiRegex<'p> {
    matcher: Box<dyn Matcher + 'p>,
}

impl<'p> MultiRegex<'p> {
    /// Create a new regular expression matcher from the given patterns.
    /// 
    /// The function determines which internal matcher works best on the
    /// given patterns and instantiates it to be used during matching.
    pub fn new(patterns: &'p [&'p str]) -> Result<MultiRegex<'p>, Error> {
        // Assert that at least one pattern is present
        if patterns.len() == 0 {
            return Err(Error::Argument("No patterns were specified!"));
        }

        // Preprocess each pattern.
        let types: Vec<Result<Suggestion, Error>> = patterns
            .iter()
            .map(|p| Preprocessor::new(p).determine_type())
            .collect();

        // If any one of the preprocessing runs resulted in an error, return that error.
        for result in &types {
            if let Err(reason) = result {
                return Err(reason.to_owned());
            }
        }

        // If only one pattern is present, use the proper single pattern matcher.
        if patterns.len() == 1 {
            let pattern = patterns.first().unwrap();
            let matcher: Box<dyn Matcher + 'p> = match types.first().unwrap().as_ref().unwrap() {
                Suggestion::Literal => Box::new(LiteralMatcher::new(pattern)),
                Suggestion::Longest => Box::new(LongestMatcher::new(pattern)),
                Suggestion::Prefix => Box::new(PrefixMatcher::new(pattern)),
                Suggestion::Nothing => Box::new(NothingMatcher::new(pattern))
            };

            return Ok(MultiRegex { matcher });
        }

        // If all of the patterns are literal, we can use the Wu-Manber matcher directly.
        if types.iter().all(|t| t.clone().unwrap() == Suggestion::Literal) {
            return Ok(MultiRegex { matcher: Box::new(LiteralMultiMatcher::new(patterns)) });
        }

        // If any one pattern cannot be used with the literal or the longest matcher,
        // we'll run the naive algorithm that checks each pattern sequentially.
        if types.iter().any(|t| [Suggestion::Nothing, Suggestion::Prefix].contains(t.as_ref().unwrap())) {
            return Ok(MultiRegex { matcher: Box::new(NothingMultiMatcher::new(patterns)) });
        }

        // Otherwise, every pattern uses either the longest or the literal heuristics, but
        // we can't use the Wu-Manber matcher directly, as at least one pattern isn't literal.
        return Ok(MultiRegex { matcher: Box::new(LongestMultiMatcher::new(patterns)) });
    }

    /// Determines whether the given text contains any matches for the compiled patterns.
    pub fn is_match(&self, text: &str) -> bool {
        return self.matcher.find(text).is_some();
    }
    
    /// Finds the first match of the compiled patterns present
    /// in the text, or returns None if no matches are found.
    pub fn find(&self, text: &str) -> Option<Match> {
        return self.matcher.find(text);
    }
}
