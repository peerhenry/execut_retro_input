use crate::spaceship_settings::{SpaceshipSetting, SpaceshipSettingValue};
use crate::text_scene::SelectedInput;
use crate::text_scene::cost_calculator::*;

pub struct TextVariables {
  pub player_name: String,
  pub player_remaining_points: u32,
  pub ship_settings: [SpaceshipSettingValue; 4],
  pub selected_input: SelectedInput,
  pub appendix: String,
}

pub fn generate_string(variables: TextVariables) -> String {
  let name: &str = &variables.player_name;
  let mut lines: Vec<String> = vec![
    format!("Welcome {}.", name),
    String::from(""),
    String::from("Prepare for space invaders!"),
    String::from("Please input your spaceship settings..."),
    String::from(""),
    String::from(format!(
      "Points remaining: {}",
      variables.player_remaining_points
    )),
    String::from(""),
  ];
  let setting_points = variables.ship_settings.clone();
  for (_i, elem) in setting_points.iter().enumerate() {
    let setting_name: &str = elem.setting.name();
    let points: u32 = elem.value;
    let cost: u32 = calculate_cost(&elem);
    let new_line: String = format!("  {}: {} (cost: {} pts)", setting_name, points, cost);
    lines.push(new_line);
  }
  lines.push(String::from(" "));
  lines.push(String::from("  SUBMIT"));
  match variables.selected_input {
    SelectedInput::Setting(setting) => {
      let offest = setting.to_index();
      lines[7 + offest] = lines[7 + offest].replace("  ", "> ");
    },
    SelectedInput::Submit => {
      lines[12] = lines[12].replace("  ", "> ");
    }
  }
  lines.push(String::from(" "));
  lines.push(variables.appendix);
  lines.join("\n")
}
