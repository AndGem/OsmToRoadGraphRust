
pub fn keep_characters(original: &String, to_strip: &str) -> String {
    original.chars().filter(|&c| to_strip.contains(c)).collect()
}
