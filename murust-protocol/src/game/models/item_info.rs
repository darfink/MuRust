#![allow(dead_code)]
use murust_data_model::entities::Item;
use murust_data_model::types::ItemCode;

/// The size required by the protocol.
const ITEM_INFO_SIZE: usize = 7;

#[derive(Serialize, Debug)]
pub struct ItemInfo([u8; ITEM_INFO_SIZE]);

impl ItemInfo {
  pub fn new(item: &Item) -> Self {
    let mut data = [0u8; ITEM_INFO_SIZE];
    {
      let mut prot = ItemInfoView(&mut data);
      prot.set_code(item.code);
      prot.set_level(item.level);
      prot.set_durability(item.durability);
    }
    ItemInfo(data)
  }
}

bitfield! {
  struct ItemInfoView([u8]);
  u8;
  code_low8, set_code_low8:      7,  0;

  /// +4/+8/+12
  option3, set_option3:          9,  8;
  /// Luck
  option2, set_option2:         10, 10;
  /// Up to +13
  level, set_level:             14, 11;
  /// Skill
  option1, set_option1:         15, 15;

  /// 0 - 255
  durability, set_durability:   23, 16;

  /// Excellent options
  noption, set_noption:         29, 24;
  /// +16 (or dinorant)
  option3ext, set_option3ext:   30, 30;
  code_mid1, set_code_mid1:     31, 31;

  /// Ancient, increase stamina % 4 == 1|2
  tier_option, set_tier_option: 39, 32;

  /// 380+ items enchantment
  jog_option, set_jog_option:   43, 43;
  code_high4, set_code_high4:   47, 44;

  /// Harmony bullshit
  unk1, set_unk1:   51, 48;
  unk2, set_unk2:   55, 52;
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> ItemInfoView<T> {
  pub fn set_code(&mut self, item: ItemCode) {
    self.set_code_low8((item.as_raw() & 0xFF) as u8);
    self.set_code_mid1(((item.as_raw() & 0x100) >> 8) as u8);
    self.set_code_high4(((item.as_raw() & 0x1E00) >> 9) as u8);
  }
}
