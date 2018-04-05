#![allow(dead_code)]
use mu::{ItemCode, ItemGroup, ItemId, equipment::Slot};

#[derive(Serialize, Debug)]
pub struct Equipment([u8; 17]);

impl Default for Equipment {
  fn default() -> Self {
    let mut data = [0u8; 17];
    {
      let mut view = EquipmentView(&mut data);
      for slot in Slot::values() {
        view.set_item_slot(*slot, None);
      }
    }
    Equipment(data)
  }
}

bitfield! {
  struct EquipmentView([u8]);
  u8;
  weapon_right_low8, set_weapon_right_low8:            7,   0;
  weapon_left_low8, set_weapon_left_low8:             15,   8;
  armor_low4, set_armor_low4:                         19,  16;
  helm_low4, set_helm_low4:                           23,  20;
  gloves_low4, set_gloves_low4:                       27,  24;
  pants_low4, set_pants_low4:                         31,  28;

  helper_low2, set_helper_low2:                       33,  32;
  wings_mod2, set_wings_mod2:                         35,  34;
  boots_low4, set_boots_low4:                         39,  36;

  shine, set_shine:                                   63,  40;

  wings_mod3, set_wings_mod3:                         66,  64;
  boots_mid5, set_boots_mid5:                         67,  67;
  gloves_mid5, set_gloves_mid5:                       68,  68;
  pants_mid5, set_pants_mid5:                         69,  69;
  armor_mid5, set_armor_mid5:                         70,  70;
  helm_mid5, set_helm_mid5:                           71,  71;

  has_dinorant, set_has_dinorant:                     72,  72;
  excellent_weapon_left, set_excellent_weapon_left:   73,  73;
  excellent_weapon_right, set_excellent_weapon_right: 74,  74;
  excellent_boots, set_excellent_boots:               75,  75;
  excellent_gloves, set_excellent_gloves:             76,  76;
  excellent_pants, set_excellent_pants:               77,  77;
  excellent_armor, set_excellent_armor:               78,  78;
  excellent_helm, set_excellent_helm:                 79,  79;

  has_full_tier, set_has_full_tier:                   80,  80;
  ancient_weapon_left, set_ancient_weapon_left:       81,  81;
  ancient_weapon_right, set_ancient_weapon_right:     82,  82;
  ancient_boots, set_ancient_boots:                   83,  83;
  ancient_gloves, set_ancient_gloves:                 84,  84;
  ancient_pants, set_ancient_pants:                   85,  85;
  ancient_armor, set_ancient_armor:                   86,  86;
  ancient_helm, set_ancient_helm:                     87,  87;

  dark_horse, set_dark_horse:                         88,  88;
  unk1, set_unk1:                                     89,  89;
  has_fenrir, set_has_fenrir:                         90,  90;
  //padding0, _:                                      91,  91;
  weapon_right_high4, set_weapon_right_high4:         95,  92;

  helm_high4, set_helm_high4:                         99,  96;
  weapon_left_high4, set_weapon_left_high4:           103, 100;

  pants_high4, set_pants_high4:                       107, 104;
  armor_high4, set_armor_high4:                       111, 108;

  gloves_high4, set_gloves_high4:                     115, 112;
  boots_high4, set_boots_high4:                       119, 116;

  fenrir, set_fenrir:                                 121, 120;
  //padding1, _:                                      135, 122;
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> EquipmentView<T> {
  pub fn set_wings(&mut self, item: Option<ItemId>) {
    match item {
      Some(item) => {
        match item.tuple() {
          // 1st level wings
          (ItemGroup::Wings, 0...2) => {
            self.set_wings_mod2(item.index() as u8);
            self.set_wings_mod3(0);
          },
          // 2nd level wings (including MGs)
          (ItemGroup::Wings, 3...6) => {
            self.set_wings_mod2(0b11);
            self.set_wings_mod3(item.index() as u8 - 2);
          },
          // Cape of Lord is sent as 'Wings of Devil'
          (ItemGroup::Helper, 30) => {
            self.set_wings_mod2(0b11);
            self.set_wings_mod3(0b101);
          },
          _ => unreachable!("invalid item code"),
        }
      },
      None => {
        self.set_wings_mod2(0b11);
        self.set_wings_mod3(0);
      },
    }
  }

  pub fn set_helper(&mut self, item: Option<ItemId>) {
    self.set_helper_low2(0b11);
    self.set_has_dinorant(0);
    self.set_dark_horse(0);
    self.set_has_fenrir(0);
    self.set_fenrir(0);

    if let Some(item) = item {
      match item.tuple() {
        (ItemGroup::Helper, 0...2) => {
          self.set_helper_low2(item.index() as u8);
        },
        (ItemGroup::Helper, 3) => {
          self.set_helper_low2(item.index() as u8);
          self.set_has_dinorant(1);
        },
        (ItemGroup::Helper, 4) => {
          self.set_dark_horse(1);
        },
        (ItemGroup::Helper, 37) => {
          // TODO: Implement disss
          unimplemented!();
          // self.set_fenrir(item.mod());
          // self.set_has_fenrir(1);
        },
        _ => unreachable!("invalid item code"),
      }
    }
  }

  pub fn set_weapon_right(&mut self, item: Option<ItemId>) {
    let code = item.map(|item| item.as_raw()).unwrap_or(0x1FFF);
    self.set_weapon_right_low8((code & 0xFF) as u8);
    self.set_weapon_right_high4(((code & 0xF00) >> 8) as u8);
  }

  pub fn set_weapon_left(&mut self, item: Option<ItemId>) {
    let code = item
      .map(|item| {
        // Dark Raven is sent as 'Legendary Staff'
        if item.tuple() == (ItemGroup::Helper, 5) {
          ItemCode::new(ItemGroup::Staff, 5).as_raw()
        } else {
          item.as_raw()
        }
      })
      .unwrap_or(0x1FFF);
    self.set_weapon_left_low8((code & 0xFF) as u8);
    self.set_weapon_left_high4(((code & 0xF00) >> 8) as u8);
  }

  pub fn set_boots(&mut self, item: Option<ItemId>) {
    let index = item
      .map(|item| {
        assert_eq!(item.group(), ItemGroup::Boots);
        item.index()
      })
      .unwrap_or(0x1FF);
    self.set_boots_low4((index & 0x0F) as u8);
    self.set_boots_mid5(((index & 0x10) >> 4) as u8);
    self.set_boots_high4(((index & 0x1E0) >> 5) as u8);
  }

  pub fn set_gloves(&mut self, item: Option<ItemId>) {
    let index = item
      .map(|item| {
        assert_eq!(item.group(), ItemGroup::Gloves);
        item.index()
      })
      .unwrap_or(0x1FF);
    self.set_gloves_low4((index & 0x0F) as u8);
    self.set_gloves_mid5(((index & 0x10) >> 4) as u8);
    self.set_gloves_high4(((index & 0x1E0) >> 5) as u8);
  }

  pub fn set_pants(&mut self, item: Option<ItemId>) {
    let index = item
      .map(|item| {
        assert_eq!(item.group(), ItemGroup::Pants);
        item.index()
      })
      .unwrap_or(0x1FF);
    self.set_pants_low4((index & 0x0F) as u8);
    self.set_pants_mid5(((index & 0x10) >> 4) as u8);
    self.set_pants_high4(((index & 0x1E0) >> 5) as u8);
  }

  pub fn set_armor(&mut self, item: Option<ItemId>) {
    let index = item
      .map(|item| {
        assert_eq!(item.group(), ItemGroup::Armor);
        item.index()
      })
      .unwrap_or(0x1FF);
    self.set_armor_low4((index & 0x0F) as u8);
    self.set_armor_mid5(((index & 0x10) >> 4) as u8);
    self.set_armor_high4(((index & 0x1E0) >> 5) as u8);
  }

  pub fn set_helm(&mut self, item: Option<ItemId>) {
    let index = item
      .map(|item| {
        assert_eq!(item.group(), ItemGroup::Helm);
        item.index()
      })
      .unwrap_or(0x1FF);
    self.set_helm_low4((index & 0x0F) as u8);
    self.set_helm_mid5(((index & 0x10) >> 4) as u8);
    self.set_helm_high4(((index & 0x1E0) >> 5) as u8);
  }

  pub fn set_item_slot(&mut self, slot: Slot, item: Option<ItemId>) {
    match slot {
      Slot::WeaponRight => self.set_weapon_right(item),
      Slot::WeaponLeft => self.set_weapon_left(item),
      Slot::Helm => self.set_helm(item),
      Slot::Armor => self.set_armor(item),
      Slot::Pants => self.set_pants(item),
      Slot::Gloves => self.set_gloves(item),
      Slot::Boots => self.set_boots(item),
      Slot::Wings => self.set_wings(item),
      Slot::Helper => self.set_helper(item),
      _ => unreachable!("invalid item slot {:?}", slot),
    }
  }
}
