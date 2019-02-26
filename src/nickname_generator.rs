use rand::*;
use std::collections::HashMap;

pub struct NicknameGenerator {
  adjectives: Vec<String>,
  nouns: Vec<String>,
  taken_names: HashMap<String, u32>
}

fn take_random_elem(vec: &Vec<String>) -> &str {
  let mut rng = rand::thread_rng();
  &vec[rng.gen_range(0, vec.len())]
}

fn make_vec(string_thing: &str) -> Vec<String> {
  let mut string_vec: Vec<String> = Vec::new();
  let mut lines = string_thing.lines();
  while let Some(line) = lines.next() {
    string_vec.push(capitalize(line));
  }
  string_vec
}

fn capitalize(thing: &str) -> String {
  let mut v: Vec<char> = thing.chars().collect();
  v[0] = v[0].to_uppercase().nth(0).unwrap();
  v.into_iter().collect()
}

impl NicknameGenerator {
  pub fn new(adjectives_string: &str, nouns_string: &str, taken_nicknames: Vec<String>) -> Self {
    let mut output = NicknameGenerator {
      adjectives: make_vec(adjectives_string),
      nouns: make_vec(nouns_string),
      taken_names: HashMap::new()
    };
    output.set_taken_nicknames(taken_nicknames);
    output
  }

  fn set_taken_nicknames(&mut self, taken_nicknames: Vec<String>) {
    for name in taken_nicknames {
      let splits: Vec<&str> = name.split(" ").collect();
      let last = splits[splits.len() - 1];
      if last.parse::<f64>().is_ok() {
        let actual_name = splits[0..splits.len() - 1].join(" ");
        self.register_nickname(String::from(actual_name));
      } else {
        self.register_nickname(String::from(name));
      }
    }
  }

  fn register_nickname(&mut self, nickname: String) -> String {
    let count = self.taken_names.entry(nickname.clone()).or_insert(0);
    *count += 1;
    let final_nickname = if *count > 1 {
      let nickname_with_counter = format!("{} {}", nickname, count);
      self.register_nickname(nickname_with_counter)
    } else { nickname };
    final_nickname
  }

  pub fn generate_nickname(&mut self) -> String {
    let adj: &str = take_random_elem(&self.adjectives);
    let noun: &str = take_random_elem(&self.nouns);
    let nickname = format!("{} {}", adj, noun);
    let final_nickname = self.register_nickname(nickname);
    final_nickname
  }
}

#[cfg(test)]
mod tests {
  use super::NicknameGenerator;

  #[test]
  fn test_generate_nickname() {
    let mut gen = NicknameGenerator::new("bad", "Man", vec![]);
    let result = gen.generate_nickname();
    let expected = "Bad Man";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_taken_nickname() {
    let mut gen = NicknameGenerator::new("bad", "man", vec![]);
    gen.generate_nickname();
    let result = gen.generate_nickname();
    let expected = "Bad Man 2";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_two_taken_nicknames() {
    let mut gen = NicknameGenerator::new("bad", "man", vec![]);
    gen.generate_nickname();
    gen.generate_nickname();
    let result = gen.generate_nickname();
    let expected = "Bad Man 3";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_taken_nicknames_constructor() {
    let mut gen = NicknameGenerator::new("bad", "man", vec![String::from("Bad Man")]);
    let result = gen.generate_nickname();
    let expected = "Bad Man 2";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_taken_nicknames_constructor_two() {
    let mut gen = NicknameGenerator::new("bad", "man", vec![String::from("Bad Man"), String::from("Bad Man 2")]);
    let result = gen.generate_nickname();
    let expected = "Bad Man 3";
    assert_eq!(expected, result);
  }
}