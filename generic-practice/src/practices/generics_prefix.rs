pub fn filter_by_prefix<'a>(words: &[&'a str], prefix: &'a str) -> Vec<&'a str> {
    words.iter().copied().filter(|x| x.starts_with(prefix)).collect()
}