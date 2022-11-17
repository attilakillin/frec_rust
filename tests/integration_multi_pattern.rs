use frec_rust::multimatcher::MultiRegex;


const INPUTS: &'static [(&[&str], &str, Option<(isize, isize)>)] = &[
    (&["pattern", "not present"], "text with pattern", Some((10, 17))),
    (&["alpha", "beta"], "alpha beta gamma delta", Some((0, 5))),
    (&["beta", "delta"], "alpha beta gamma delta", Some((6, 10))),
    (&["delta", "beta"], "alpha beta gamma delta", Some((6, 10))),
    (&["gamma", "beta", "delta", "alpha"], "alpha beta gamma delta", Some((0, 5))),
];

#[test]
fn test_multiple_patterns() {
    for (patterns, text, expected) in INPUTS {
        let regex = MultiRegex::new(patterns).unwrap();

        let actual = regex.find(text);

        assert_eq!(expected.is_some(), actual.is_some());
        if let Some(content) = actual {
            assert_eq!(expected.unwrap().0, content.start());
            assert_eq!(expected.unwrap().1, content.end());
        }
    }
}
