#[derive(Clone, Copy)]
pub enum SpaceshipSetting {
  Shields,
  Firepower,
  DefenseThickness,
  DodgeChance
}

impl Default for SpaceshipSetting {
  fn default() -> Self { SpaceshipSetting::Shields }
}

#[derive(Clone, Copy, Default)]
pub struct SpaceshipSettingValue {
  pub setting: SpaceshipSetting,
  pub value: u32
}

impl SpaceshipSettingValue {
  pub fn new(setting: SpaceshipSetting) -> Self {
    SpaceshipSettingValue {
      setting,
      value: 0
    }
  }
}