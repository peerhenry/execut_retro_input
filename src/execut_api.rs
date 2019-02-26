use std::collections::HashMap;

const ADDRESS: &str = "http://localhost:4444/";

fn get_endpoint(endpoint: &str) -> String {
  format!("{}{}", ADDRESS, endpoint)
}

pub fn fetch_taken_nicknames() -> Result<Vec<String>, Box<std::error::Error>> {
  let endpoint = get_endpoint("taken-nicknames");
  println!("calling {}", endpoint); // DEBUG
  // let response = reqwest::get(&endpoint)?.json()?;
  let mut response = reqwest::get(&endpoint)?;
  let response_text: String = response.text()?;
  println!("response_text: {:#?}", response_text); // DEBUG
  let deserialized: Vec<String> = serde_json::from_str(&response_text).unwrap();
  println!("deserialized: {:#?}", deserialized); // DEBUG
  Ok(deserialized)
}

pub fn post_new_player(nickname: String) -> Result<(), Box<std::error::Error>> {
  let mut map = HashMap::new();
  map.insert("nickname", &nickname);
  let endpoint = get_endpoint("new-player");
  let client = reqwest::Client::new();
  let res = client.post(&endpoint).json(&map).send()?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use serde::{ Serialize, Deserialize };

  #[test]
  fn test_string_deserialization() {
    let deserialized: Vec<String> = serde_json::from_str("[\"piet\", \"klaas\"]").unwrap();
    let expected: Vec<String> = vec![String::from("piet"), String::from("klaas")];
    assert_eq!(expected, deserialized);
  }
}