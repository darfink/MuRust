//! Game Client Packets

pub use self::group::Client;
use super::{Serial, Version, util::deserialize_class};
use game::visitors::CharacterMoveVisitor;
use game::{models::ItemInfo, util::StringFixedCredentials};
use muonline_packet_serialize::{IntegerLE, StringFixed};
use murust_data_model::types::{Class, Direction, Position};
use serde::{Deserialize, Deserializer};
use typenum;

mod group;

/// `C1:0E:00` - Local client timing values.
///
/// This is sent by default every 20th second.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// time | `U32` | The client's time instant in milliseconds. | LE
/// speed (attack) | `U16` | The client's current attack speed. | LE
/// speed (magic) | `U16` | The client's current magic speed. | LE
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "0E", subcode = "00")]
pub struct ClientTime {
  #[serde(with = "IntegerLE")]
  pub time: u32,
  #[serde(with = "IntegerLE")]
  pub attack_speed: u16,
  #[serde(with = "IntegerLE")]
  pub magic_speed: u16,
}

#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "18")]
pub struct CharacterAction {
  pub direction: Direction,
  pub action: ActionType,
}

#[repr(u8)]
#[derive(Primitive, Debug, Copy, Clone)]
pub enum ActionType {
  SkillPoison = 0x1,
  SkillMeteorite = 0x2,
  SkillLightning = 0x3,
  SkillFireball = 0x4,
  SkillFlame = 0x5,
  SkillTeleport = 0x6,
  SkillIce = 0x7,
  SkillTwister = 0x8,
  SkillEvilSpirits = 0x9,
  SkillHellfire = 0xa,
  SkillPowerwave = 0xb,
  SkillFlash = 0xc,
  SkillBlast = 0xd,
  SkillInferno = 0xe,
  SkillTargetTeleport = 0xf,
  SkillMagicDefense = 0x10,
  SkillEnergyBall = 0x11,
  SkillBlocking = 0x12,
  SkillSword1 = 0x13,
  SkillSword2 = 0x14,
  SkillSword3 = 0x15,
  SkillSword4 = 0x16,
  SkillSword5 = 0x17,
  SkillTripleShotCrossbow = 0x18,
  SkillTripleShotBow = 0x19,
  SkillHeal = 0x1a,
  SkillDefenseAura = 0x1b,
  SkillAttackAura = 0x1c,
  SkillSummon1 = 0x1e,
  SkillSummon2 = 0x1f,
  SkillSummon3 = 0x20,
  SkillSummon4 = 0x21,
  SkillSummon5 = 0x22,
  SkillSummon6 = 0x23,
  SkillSummon7 = 0x24,
  SkillWheel = 0x29,
  SkillDevilsFire = 0x2a,
  SkillStrike = 0x2b,
  SkillKnightSpear = 0x2f,
  SkillGreaterFortitude = 0x30,
  SkillDinorantAttack = 0x31,
  SkillElfHarden = 0x33,
  SkillPenetration = 0x34,
  SkillDefenseDown = 0x37,
  SkillSword6 = 0x38,
  SkillPentaShotCrossbow = 0x36,
  SkillExpPoison = 0x26,
  SkillExpIce = 0x27,
  SkillExpHell = 0x28,
  SkillExpHellStart = 0x3a,
  SkillImproveAgRefill = 0x35,
  SkillDevilFire = 0x32,
  SkillCombo = 0x3b,
  SkillSpear = 0x3c,
  SkillFireburst = 0x3d,
  SkillDarkhorseAttack = 0x3e,
  SkillRecallParty = 0x3f,
  SkillAddCriticalDamage = 0x40,
  SkillElectricSpark = 0x41,
  SkillLongSpear = 0x42,
  SkillRush = 0x2c,
  SkillJavelin = 0x2d,
  SkillDeepImpact = 0x2e,
  SkillOneFlash = 0x39,
  SkillDeathCannon = 0x49,
  SkillSpaceSplit = 0x4a,
  SkillBrandOfSkill = 0x4b,
  SkillStun = 0x43,
  SkillRemoveStun = 0x44,
  SkillAddMana = 0x45,
  SkillInvisible = 0x46,
  SkillRemoveInvisible = 0x47,
  SkillRemoveMagic = 0x48,
  SkillFenrirAttack = 0x4c,
  SkillInfinityArrow = 0x4d,
  SkillFirescream = 0x4e,
  SkillExplosion = 0x4f,
  ImproveDamage = 0x50,
  ImproveWizardry = 0x51,
  ImproveBlock = 0x52,
  ImproveDefense = 0x53,
  Luck = 0x54,
  LifeRegeneration = 0x55,
  ImproveLife = 0x56,
  ImproveMana = 0x57,
  DecreaseDamage = 0x58,
  ReflectDamage = 0x59,
  ImproveBlockingPercent = 0x5a,
  ImproveMoneyDrop = 0x5b,
  ExcellentDamage = 0x5c,
  ImproveDamagePerLevel = 0x5d,
  ImproveDamagePercent = 0x5e,
  ImproveMagicLevel = 0x5f,
  ImproveMagicPercent = 0x60,
  ImproveAttackSpeed = 0x61,
  ImproveGainLife = 0x62,
  ImproveGainMana = 0x63,
  WingsMaxLife = 0x64,
  WingsMaxMana = 0x65,
  IncreaseDamageOnePercent = 0x66,
  ImproveMaxAbility = 0x67,
  DamageAbsorb = 0x68,
  WingIncreaseLeadership = 0x69,
  FenrirIncreaseLastDamage = 0x6a,
  FenrirDecreaseLastDamage = 0x6b,
  AnimationAttack1 = 0x78,
  AnimationAttack2 = 0x79,
  AnimationStand1 = 0x7a,
  AnimationStand2 = 0x7b,
  AnimationMove1 = 0x7c,
  AnimationMove2 = 0x7d,
  AnimationDamage1 = 0x7e,
  AnimationDie1 = 0x7f,
  AnimationSit1 = 0x80,
  AnimationPose1 = 0x81,
  AnimationHealing1 = 0x82,
  GestureGreeting1 = 0x83,
  GestureGoodbye1 = 0x84,
  GestureClap1 = 0x85,
  GestureGesture1 = 0x86,
  GestureDirection1 = 0x87,
  GestureUnknown1 = 0x88,
  GestureCry1 = 0x89,
  GestureCheer1 = 0x8a,
  GestureAwkward1 = 0x8b,
  GestureSee1 = 0x8c,
  GestureWin1 = 0x8d,
  GestureSmile1 = 0x8e,
  GestureSleep1 = 0x8f,
  GestureCold1 = 0x90,
  GestureAgain1 = 0x91,
  GestureRespect1 = 0x92,
  GestureSalute1 = 0x93,
  GestureRush1 = 0x94,
  SetRingOptionAddSkillDamage = 0x95,
  SetOptionImproveStrength = 0xa0,
  SetOptionImproveAgility = 0xa1,
  SetOptionImproveEnergy = 0xa2,
  SetOptionImproveVitality = 0xa3,
  SetOptionImproveCommand = 0xa4,
  SetOptionImproveMinAttackDamage = 0xa5,
  SetOptionImproveMaxAttackDamage = 0xa6,
  SetOptionImproveWizardry = 0xa7,
  SetOptionImproveDamage = 0xa8,
  SetOptionImproveAttackRate = 0xa9,
  SetOptionImproveDefense = 0xaa,
  SetOptionImproveMaxLife = 0xab,
  SetOptionImproveMaxMana = 0xac,
  SetOptionImproveMaxAbility = 0xad,
  SetOptionImproveAbility = 0xae,
  SetOptionImproveCriticalDamageSuccess = 0xaf,
  SetOptionImproveCriticalDamage = 0xb0,
  SetOptionImproveExDamageSuccess = 0xb1,
  SetOptionImproveExDamage = 0xb2,
  SetOptionImproveSkillDamage = 0xb3,
  SetOptionDoubleDamage = 0xb4,
  SetOptionDefenseIgnore = 0xb5,
  SetOptionImproveShieldDefense = 0xb6,
  SetOptionTwoHandSwordImproveDamage = 0xb7,
  SetOptionImproveAttackdamageWithStr = 0xb8,
  SetOptionImproveAttackdamageWithDex = 0xb9,
  SetOptionImproveDefenseWithAgility = 0xba,
  SetOptionImproveDefenseWithVitality = 0xbb,
  SetOptionImproveMagicdamageWithEnergy = 0xbc,
  SetOptionIceMastery = 0xbd,
  SetOptionPosionMastery = 0xbe,
  SetOptionThunderMastery = 0xbf,
  SetOptionFireMastery = 0xc0,
  SetOptionEarthMastery = 0xc1,
  SetOptionWindMastery = 0xc2,
  SetOptionWaterMastery = 0xc3,
  SetImproveStrength = 0xc4,
  SetImproveDexterity = 0xc5,
  SetImproveEnergy = 0xc6,
  SetImproveVitality = 0xc7,
  SkillSummon = 0xc8,
  SkillImmuneToMagic = 0xc9,
  SkillImmuneToHarm = 0xca,
}

primitive_serialize!(ActionType, u8);

//
/// `C1:24` - Describing the relocation of an inventory item.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// source | `U8` | The storage type of the source. | -
/// slot | `U8` | The slot the item is moved from. | -
/// item | `Item` | The item that is being moved. | -
/// target | `U8` | The storage type of the target. | -
/// slot | `U8` | The slot the item is moved to. | -
///
/// Value | Type
/// `0x00` | Inventory
/// `0x01` | Trade
/// `0x02` | Warehouse
/// `0x03` | Chaos box
/// `0x05` | Dark trainer box
// #[derive(MuPacket, Debug)]
// #[packet(kind = "C1", code = "24")]
// pub struct ItemMove {
// pub source: (StorageType, u8),
// pub target: (StorageType, u8),
// TODO: What to do with diiis one?
// pub item_info: ItemInfo,
// }
//
// #[derive(Eq, PartialEq, Debug)]
// pub enum StorageType {
// Equipment,
// Inventory,
// PersonalShop,
// Trade,
// Warehouse,
// ChaosBox,
// DarkTrainerBox,
// }
//
// impl<'de> Deserialize<'de> for ItemMove {
// fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
// #[derive(Deserialize)]
// struct ItemMovePacket {
// source_storage: u8,
// source_slot: u8,
// item_info: ItemInfo,
// target_storage: u8,
// target_slot: u8,
// }
// fn parse_slot_type(storage: u8, slot: u8) -> Option<(StorageType, u8)> {
// use std::ops::Range;
// use murust_data_model::types::ItemSlot;
// let result = if storage == 0 {
// const EQ_RANGE: Range<usize> = 0..ItemSlot::SIZE;
// const IV_RANGE: Range<usize> = ItemSlot::SIZE..76;
// const PS_RANGE: Range<usize> = 76..108;
// match slot {
// EQ_RANGE => (StorageType::Equipment, slot.saturating_sub(EQ_RANGE.end as u8)),
// IV_RANGE => (StorageType::Inventory, slot.saturating_sub(IV_RANGE.end as u8)),
// PS_RANGE => (StorageType::PersonalShop, slot.saturating_sub(PS_RANGE.end as u8)),
// _ => return None,
// }
// } else {
// let storage = match storage {
// 1 => StorageType::Trade,
// 2 => StorageType::Warehouse,
// 3 => StorageType::ChaosBox,
// 5 => StorageType::DarkTrainerBox,
// _ => return None,
// };
// (storage, slot)
// };
// Some(result)
// }
//
// let item_move = ItemMovePacket::deserialize(deserializer)?;
// Ok(ItemMove {
// source: parse_slot_type(item_move.source_storage, item_move.source_slot),
// target: parse_slot_type(item_move.target_storage, item_move.target_slot),
// item_info: item_move.item_info,
// })
// }
// }
//
/// `C1:D4` - Describes a character's movement.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// x | `U8` | The character's current X coordinate. | -
/// y | `U8` | The character's current Y coordinate. | -
/// count | `U4` | The number of path directions. | -
/// direction | `U4` | The character's current direction. | -
/// path | `U4[]` | An array of path directions. | -
///
/// Value | Direction
/// ------ | -------
/// `0x00` | South West
/// `0x01` | South
/// `0x02` | South East
/// `0x03` | East
/// `0x04` | North East
/// `0x05` | North
/// `0x06` | North West
/// `0x07` | West
#[derive(MuPacket, Debug)]
#[packet(kind = "C1", code = "D4")]
pub struct CharacterMove {
  pub direction: Direction,
  pub path: Vec<Position>,
}

impl<'de> Deserialize<'de> for CharacterMove {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    deserializer.deserialize_tuple(usize::max_value(), CharacterMoveVisitor)
  }
}

/// `C1:F1:01` - Authentication request sent upon client login.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// username | `CHAR(10)` | The specified username. | -
/// password | `CHAR(10)` | The specified password. | -
/// time | `U32` | The client's time instant in milliseconds. | LE
/// version | `U8(5)` | The client's protocol version. | -
/// serial | `CHAR(16)` | The client's serial version. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F1", subcode = "01")]
pub struct AccountLoginRequest {
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub username: String,
  #[serde(with = "StringFixedCredentials::<typenum::U10>")]
  pub password: String,
  #[serde(with = "IntegerLE")]
  pub time: u32,
  pub version: Version,
  pub serial: Serial,
}

/// `C1:F3:00` - Request for an account's characters.
///
/// This is sent from the client as soon as it has successfully logged in with an
/// account.
///
/// ## Example
///
/// ```c
/// [0xC1, 0x04, 0xF3, 0x00]
/// ```
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "00")]
pub struct CharacterListRequest;

/// `C1:F3:01` - Request for a character creation.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
/// class | `U8` | The character's class. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "01")]
pub struct CharacterCreate {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
  #[serde(deserialize_with = "deserialize_class")]
  pub class: Class,
}

/// `C1:F3:02` - Request for a character deletion.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
/// code | `CHAR(10)` | The account's security code. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "02")]
pub struct CharacterDelete {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub security_code: String,
}

/// `C1:F3:03` - Request for a character selection.
///
/// ## Layout
///
/// Field | Type | Description | Endianess
/// ----- | ---- | ----------- | ---------
/// name | `CHAR(10)` | The character's name. | -
#[derive(Deserialize, MuPacket, Debug)]
#[packet(kind = "C1", code = "F3", subcode = "03")]
pub struct CharacterSelect {
  #[serde(with = "StringFixed::<typenum::U10>")]
  pub name: String,
}
