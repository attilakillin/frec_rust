use std::slice;

use fregex::{Regex, MultiRegex, RegexMatcher};

const INPUTS: &'static [(&str, &str, Option<(usize, usize)>)] = &[
    ("pattern", "text with pattern", Some((10, 17))),
    ("many", "many many many many", Some((0, 4))),
    ("x", "find the first x", Some((15, 16))),

    ("\\$\\(\\)\\$", "text with $()$ chars", Some((10, 14))),

    ("p..ce", "piece peace pounce", Some((0, 5))),
    ("[ai][cx]e", "words with the letter e but only axe matches", Some((33, 36))),
    ("ba(se)+", "multiple ba ba but only one is base", Some((31, 35))),
    ("plus+", "only works with extended plus text", Some((25, 29))),
    ("C[a-z]*a", "Short parts Circa", Some((12, 17))),
    ("[Pp]refix.*:", "Long string with a prefix: somewhere", Some((19, 26))),

    ("a\nb+", "text with a\nbbb", Some((10, 15))),
    ("[^s]yy*", "text with \nyd", Some((10, 12))),
];

/// Test that on a number of input combinations, the (single-pattern) matcher
/// correctly finds the first match.
#[test]
fn test_each_pattern_single_base() {
    for (pattern, text, expected) in INPUTS {
        let regex = Regex::new(pattern).unwrap();

        let actual = regex.find(text);

        assert_eq!(expected.is_some(), actual.is_some(), "Expected Some(...) for {:?}", pattern);
        if let Some(content) = actual {
            assert_eq!(expected.unwrap().0, content.start(), "Expected {} for {:?}", expected.unwrap().0, pattern);
            assert_eq!(expected.unwrap().1, content.end(), "Expected {} for {:?}", expected.unwrap().1, pattern);
        }
    }
}

/// Test that on a number of input combinations, and with only one pattern supplied,
/// the multi-pattern matcher correctly finds the first match.
#[test]
fn test_each_pattern_multi_base() {
    for (pattern, text, expected) in INPUTS {
        let regex = MultiRegex::new(slice::from_ref(pattern)).unwrap();

        let actual = regex.find(text);

        assert_eq!(expected.is_some(), actual.is_some(), "Expected Some(...) for {:?}", pattern);
        if let Some(content) = actual {
            assert_eq!(expected.unwrap().0, content.start(), "Expected {} for {:?}", expected.unwrap().0, pattern);
            assert_eq!(expected.unwrap().1, content.end(), "Expected {} for {:?}", expected.unwrap().1, pattern);
        }
    }
}
