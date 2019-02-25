use std::collections::HashMap;

const ADDRESS: &str = "http://localhost:4444/";

fn get_endpoint(endpoint: &str) -> String {
  format!("{}{}", ADDRESS, endpoint)
}

pub fn fetch_taken_nicknames() -> Result<(), Box<std::error::Error>> {
  let endpoint = get_endpoint("taken-nicknames");
  println!("calling {}", endpoint); // DEBUG
  // let response = reqwest::get(&endpoint)?.json()?;
  let mut response = reqwest::get(&endpoint)?;
  println!("{:#?}", response.text()?);
  Ok(())
}

pub fn post_new_player(nickname: String) -> Result<(), Box<std::error::Error>> {
  let mut map = HashMap::new();
  map.insert("nickname", &nickname);
  let endpoint = get_endpoint("new-player");
  let client = reqwest::Client::new();
  let res = client.post(&endpoint).json(&map).send()?;
  Ok(())
}