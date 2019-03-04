use crate::spaceship_settings::{SpaceshipSetting, SpaceshipSettingValue};
use crate::text_scene::SelectedInput;

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
    let setting_name: &str;
    let points: u32 = elem.value;
    match elem.setting {
      SpaceshipSetting::Shields => {
        setting_name = "  Shields";
      }
      SpaceshipSetting::Firepower => {
        setting_name = "  Firepower";
      }
      SpaceshipSetting::DefenseThickness => {
        setting_name = "  DefenseThickness";
      }
      SpaceshipSetting::DodgeChance => {
        setting_name = "  DodgeChance";
      }
    }
    let new_line: String = format!("{}: {}", setting_name, points);
    lines.push(new_line);
  }
  lines.push(String::from(" "));
  lines.push(String::from("  SUBMIT"));
  match variables.selected_input {
    SelectedInput::Setting(setting) => match setting {
      SpaceshipSetting::Shields => {
        lines[7] = lines[7].replace("  ", "> ");
      }
      SpaceshipSetting::Firepower => {
        lines[8] = lines[8].replace("  ", "> ");
      }
      SpaceshipSetting::DefenseThickness => {
        lines[9] = lines[9].replace("  ", "> ");
      }
      SpaceshipSetting::DodgeChance => {
        lines[10] = lines[10].replace("  ", "> ");
      }
    },
    SelectedInput::Submit => {
      lines[12] = lines[12].replace("  ", "> ");
    }
  }
  lines.push(String::from(" "));
  lines.push(variables.appendix);
  lines.join("\n")
}
