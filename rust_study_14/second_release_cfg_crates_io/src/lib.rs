// * 使用 pub use 导出到顶层
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue
  }
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple
  }
}

pub mod utils {
  use crate::kinds::*;
  
  pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> SecondaryColor {
    return SecondaryColor::Green;
  }
}
