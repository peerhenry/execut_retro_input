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

impl SpaceshipSetting {
  pub fn from_index(index: usize) -> Self {
    let output = match index {
      0 => SpaceshipSetting::Shields,
      1 => SpaceshipSetting::Firepower,
      2 => SpaceshipSetting::DefenseThickness,
      _ => SpaceshipSetting::DodgeChance
    };
    output
  }

  pub fn to_index(&self) -> usize {
    match self {
      SpaceshipSetting::Shields => 0,
      SpaceshipSetting::Firepower => 1,
      SpaceshipSetting::DefenseThickness => 2,
      SpaceshipSetting::DodgeChance => 3,
    }
  }

  pub fn count() -> usize {
    return 4;
  }

  pub fn name(&self) -> &str {
    match self {
      SpaceshipSetting::Shields => "Shields",
      SpaceshipSetting::Firepower => "Firepower",
      SpaceshipSetting::DefenseThickness => "DefenseThickness",
      SpaceshipSetting::DodgeChance => "DodgeChance",
    }
  }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count() {
    assert_eq!(4, SpaceshipSetting::count());
  }

  #[test]
  fn assert_from_to_index() {
    for i in 0..4 {
      let thing = SpaceshipSetting::from_index(i);
      let res = thing.to_index();
      assert_eq!(i, res);
    }
  }
}