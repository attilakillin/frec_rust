use std::{cmp::{max, min_by, min}, collections::HashMap};
use crate::types::Match;

/// Wu-Manber compilation struct.
pub struct WuManber {
    /// A reference to every pattern, necessary during the searching phase.
    patterns: Vec<String>,
    /// Shift (or jump) table for each b-long block present in the patterns.
    shift_table: HashMap<String, usize>,
    /// Pattern prefix table for each pattern that starts with the key block.
    prefix_table: HashMap<String, Vec<PrefixHash>>,
    /// The length of the shortest pattern.
    min_length: usize,
    /// The default shift value, when a block was not present in the patterns.
    default_shift: usize,
    /// The "B" magic constant that represents the block size. 2 or 3 should be used.
    b: usize
}

/// Prefix hash struct used in the Wu-Manber searching phase.
struct PrefixHash {
    /// Which pattern the prefix was created from.
    pattern_id: usize,
    /// What the prefix is.
    prefix: String,
}

impl WuManber {
    /// Creates a new Wu-Manber search struct with the given patterns. The b argument
    /// can be used to define the block size used. Usually, 2 or 3 is recommended.
    pub fn new(patterns: &[&str], b: usize) -> WuManber {
        // Create variables for commonly used numbers below.
        let min_length = patterns
            .iter()
            .reduce(|p, q| min_by(p, q, |x, y| x.len().cmp(&y.len())))
            .unwrap()
            .len();

        if min_length < b {
            // TODO Handle this case - by default, WM shouldn't be used in this scenario.
            panic!("Minimum pattern length used in WM algorithm is too small! ('{:?}')", patterns);
        }

        // Create default shift and initialize block shift table.
        let default_shift = min_length + 1 - b;
        let mut shift_table: HashMap<String, usize> = HashMap::new();
        let mut prefix_table: HashMap<String, Vec<PrefixHash>> = HashMap::new();

        // Read patterns, and set block shift values.
        for (i, pattern) in patterns.iter().enumerate() {
            for j in (b ..= min_length).rev() {
                let block = &pattern[j-b ..= j-1];
                
                let this_shift = min_length - j;
                let stored_shift = shift_table.get(block).unwrap_or(&this_shift).to_owned();

                shift_table.insert(block.to_string(), min(this_shift, stored_shift));

                // For each pattern, we also store its prefix once to speed up potential match verification.
                if j == min_length {
                    let hash = PrefixHash {
                        pattern_id: i,
                        prefix: pattern[0..b].to_string()
                    };
                    
                    if prefix_table.contains_key(block) {
                        prefix_table.get_mut(block).unwrap().push(hash);
                    } else {
                        prefix_table.insert(block.to_string(), vec![hash]);
                    }
                }
            }
        }

        // Copy patterns to save in the struct.
        let patterns = patterns.iter().map(|p| p.to_string()).collect();

        // Return with the compiled struct.
        return WuManber { patterns, shift_table, prefix_table, min_length, default_shift, b };
    }

    /// Finds any one of the compiled patterns in the given text.
    pub fn find<'t>(&self, text: &'t str) -> Option<(Match<'t>, usize)> {
        let mut pos = self.min_length - 1;

        // We loop while there's text to read.
        while pos <= text.len() {
            let block = &text[pos-self.b .. pos];

            let shift = self.shift_table.get(block).unwrap_or(&self.default_shift).to_owned();

            // We found a potential match - check through prefixes for matching.
            if shift == 0 {
                let prefix_start = pos - self.min_length;
                let prefix = &text[prefix_start .. (prefix_start + self.b)];

                for candidate in self.prefix_table.get(block).unwrap() {
                    // If any candidate matches, try comparing the text with the referenced pattern.
                    if candidate.prefix == prefix {
                        let refd_pattern = &self.patterns[candidate.pattern_id];
                        let start = prefix_start;
                        let end = start + refd_pattern.len();

                        // If the whole pattern matches, return with a successful match.
                        if refd_pattern == &text[start..end] {
                            return Some((Match::new(start, end, &text[start..end]), candidate.pattern_id));
                        }
                    }
                }
            }

            pos += max(shift, 1);
        }

        // If we reached the end of the text, return with no match.
        return None;
    }
}
