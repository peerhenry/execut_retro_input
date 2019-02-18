use rand::*;

pub struct NicknameGenerator {
  adjectives: Vec<String>,
  nouns: Vec<String>,
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

  /*let splits = string_thing.split('\n');
  let ref_vec = splits.collect::<Vec<&str>>();
  for s in &ref_vec {
    string_vec.push(s.to_string());
  }*/

  string_vec
}

impl NicknameGenerator {
  pub fn new(adjectives_string: &str, nouns_string: &str) -> Self {
    NicknameGenerator {
      adjectives: make_vec(adjectives_string),
      nouns: make_vec(nouns_string)
    }
  }

  pub fn generate_nickname(&self) -> String {
    // call endpoint to see if name is taken
    let adj: &str = take_random_elem(&self.adjectives);
    let noun: &str = take_random_elem(&self.nouns);
    format!("{} {}", adj, noun)
  }
}
