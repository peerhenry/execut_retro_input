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
    string_vec.push(line.to_string());
  }
  string_vec
}

impl NicknameGenerator {
  pub fn new(adjectives_string: &str, nouns_string: &str) -> Self {
    NicknameGenerator {
      adjectives: make_vec(adjectives_string),
      nouns: make_vec(nouns_string),
      taken_names: HashMap::new()
    }
  }

  fn register_nickname(&mut self, nickname: String) -> String {
    let count = self.taken_names.entry(nickname.clone()).or_insert(0);
    *count += 1;
    let final_nickname = if *count > 1 {
      format!("{} {}", nickname, count)
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
    let mut gen = NicknameGenerator::new("bad", "Man");
    let result = gen.generate_nickname();
    let expected = "bad Man";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_taken_nickname() {
    let mut gen = NicknameGenerator::new("bad", "man");
    gen.generate_nickname();
    let result = gen.generate_nickname();
    let expected = "bad man 2";
    assert_eq!(expected, result);
  }

  #[test]
  fn test_two_taken_nickname() {
    let mut gen = NicknameGenerator::new("bad", "man");
    gen.generate_nickname();
    gen.generate_nickname();
    let result = gen.generate_nickname();
    let expected = "bad man 3";
    assert_eq!(expected, result);
  }
}