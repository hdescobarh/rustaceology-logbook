use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let [lowercase_word, sorted_lowercase_word] = generate_comparison_forms(word);
    let candidates = possible_anagrams
        .iter()
        .map(|s| (*s, generate_comparison_forms(s)));
    let mut anagram_set: HashSet<&str> = HashSet::new();

    for (candidate, [lowercase_candidate, sorted_lowercase_candidate]) in candidates {
        if sorted_lowercase_candidate == sorted_lowercase_word
            && lowercase_candidate != lowercase_word
        {
            anagram_set.insert(candidate);
        }
    }

    anagram_set
}

fn generate_comparison_forms(s: &str) -> [String; 2] {
    let lowercase = s.to_lowercase();
    let mut to_sort: Vec<char> = lowercase.chars().collect();
    to_sort.sort();
    let sorted_lowercase: String = to_sort.into_iter().collect();
    [lowercase, sorted_lowercase]
}
