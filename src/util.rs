
pub fn keep_characters(original: &String, to_keep: &str) -> String {
    original.chars().filter(|&c| to_keep.contains(c)).collect()
}



#[test]
fn digit_test() {
  let s = "85abc22".to_string();
  let result = keep_characters(&s, "0123456789");
  println!("{}", result);
  assert!(result == "8522");
}