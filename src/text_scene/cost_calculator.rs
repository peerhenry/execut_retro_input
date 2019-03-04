use crate::spaceship_settings::{SpaceshipSetting, SpaceshipSettingValue};

pub fn calculate_cost(setting_value: &SpaceshipSettingValue) -> u32 {
  let mut cost = 1;
  match setting_value.setting {
    SpaceshipSetting::Shields => {
      cost = setting_value.value + 1;
    }
    _ => {}
  }
  return cost;
}