use crate::trigger_string::TriggerStringRef;
use flo_util::binary::*;
use flo_util::dword_string::DwordString;
use flo_util::{BinDecode, BinEncode};

#[derive(Debug, BinEncode, BinDecode, PartialEq, PartialOrd, Clone, Copy)]
#[bin(enum_repr(u32))]
pub enum MapFormatVersion {
  #[bin(value = 18)]
  ROC,
  #[bin(value = 25)]
  TFT,
  #[bin(value = 28)]
  TFT131,
  #[bin(value = 31)]
  Reforged,
  UnknownValue(u32),
}

#[derive(Debug, BinDecode)]
pub struct MapInfo {
  pub version: MapFormatVersion,
  #[bin(condition = "version >= MapFormatVersion::ROC")]
  pub save_count: Option<u32>,
  #[bin(condition = "version >= MapFormatVersion::ROC")]
  pub editor_version: Option<u32>,
  #[bin(condition = "version >= MapFormatVersion::TFT131")]
  pub game_version: Option<GameVersion>,
  pub name: TriggerStringRef,
  pub author: TriggerStringRef,
  pub description: TriggerStringRef,
  pub suggested_players: TriggerStringRef,
  pub camera_bounds: CameraBounds,
  pub width: u32,
  pub height: u32,
  pub flags: u32,
  pub tile_set: u8,
  pub ls_background: i32,
  #[bin(condition = "version != MapFormatVersion::ROC")]
  pub ls_path: Option<TriggerStringRef>,
  pub ls_text: TriggerStringRef,
  pub ls_title: TriggerStringRef,
  pub ls_sub_title: TriggerStringRef,
  #[bin(condition = "version >= MapFormatVersion::ROC")]
  pub data_set: Option<u32>,
  #[bin(condition = "version != MapFormatVersion::ROC")]
  pub ps_path: Option<TriggerStringRef>,
  pub ps_text: TriggerStringRef,
  pub ps_title: TriggerStringRef,
  pub ps_sub_title: TriggerStringRef,
  #[bin(condition = "version >= MapFormatVersion::TFT")]
  pub env: Option<GameEnv>,
  #[bin(condition = "version >= MapFormatVersion::TFT131")]
  pub code_format: Option<CodeLanguage>,
  #[bin(condition = "version >= MapFormatVersion::Reforged")]
  pub asset_modes: Option<AssetMode>,
  #[bin(condition = "version >= MapFormatVersion::Reforged")]
  pub data_version: Option<GameDataVersion>,
  pub num_players: u32,
  #[bin(condition = "version < MapFormatVersion::Reforged")]
  #[bin(repeat = "num_players")]
  pub players_classic: Option<Vec<ClassicPlayer>>,
  #[bin(condition = "version >= MapFormatVersion::Reforged")]
  #[bin(repeat = "num_players")]
  pub players_reforged: Option<Vec<ReforgedPlayer>>,
  pub num_forces: u32,
  #[bin(repeat = "num_forces")]
  pub forces: Vec<Force>,
}

#[derive(Debug, Clone, BinDecode)]
pub struct GameVersion {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
  pub commit: u32,
}

#[derive(Debug, Clone, BinDecode)]
pub struct CameraBounds {
  pub bounds: [f32; 8],
  pub complements: [u32; 4],
}

#[derive(Debug, Clone, BinDecode)]
pub struct GameEnv {
  pub fog: u32,
  pub fog_start: f32,
  pub fog_end: f32,
  pub fog_density: f32,
  pub fog_color: [u8; 4],
  pub weather_id: DwordString,
  pub sound_env: TriggerStringRef,
  pub light_env: u8,
  pub water_color: [u8; 4],
}

#[derive(Debug, BinEncode, BinDecode, PartialEq, PartialOrd, Clone, Copy)]
#[bin(enum_repr(u32))]
pub enum CodeLanguage {
  #[bin(value = 0)]
  Jass,
  #[bin(value = 1)]
  Lua,
  UnknownValue(u32),
}

#[derive(Debug, BinEncode, BinDecode, PartialEq, PartialOrd, Clone, Copy)]
#[bin(enum_repr(u32))]
pub enum AssetMode {
  #[bin(value = 1)]
  SD,
  #[bin(value = 2)]
  HD,
  #[bin(value = 3)]
  SDAndHD,
  UnknownValue(u32),
}

#[derive(Debug, BinEncode, BinDecode, PartialEq, PartialOrd, Clone, Copy)]
#[bin(enum_repr(u32))]
pub enum GameDataVersion {
  #[bin(value = 1)]
  ROC,
  #[bin(value = 2)]
  TFT,
  UnknownValue(u32),
}

#[derive(Debug, Clone, BinDecode)]
pub struct ClassicPlayer {
  pub id: u32,
  pub type_: u32,
  pub race: u32,
  pub flags: u32,
  pub name: TriggerStringRef,
  pub start_pos_x: f32,
  pub start_pos_y: f32,
  pub ally_prio_low: u32,
  pub ally_prio_high: u32,
}

#[derive(Debug, Clone, BinDecode)]
pub struct ReforgedPlayer {
  pub id: u32,
  pub type_: u32,
  pub race: u32,
  pub flags: u32,
  pub name: TriggerStringRef,
  pub start_pos_x: f32,
  pub start_pos_y: f32,
  pub ally_prio_low: u32,
  pub ally_prio_high: u32,
  pub enemy_prio_low: u32,
  pub enemy_prio_high: u32,
}

#[derive(Debug, Clone, BinDecode)]
pub struct Force {
  pub flags: u32,
  pub player_set: u32,
  pub name: TriggerStringRef,
}

#[test]
fn test_parse_w3i_reforged() {
  let mut map = crate::open_archive(flo_util::sample_path!("map", "(2)ConcealedHill.w3x")).unwrap();
  let bytes = map.open_file("war3map.w3i").unwrap().read_all().unwrap();
  let mut buf = bytes.as_slice();
  let info = MapInfo::decode(&mut buf).unwrap();
  assert_eq!(info.version, MapFormatVersion::Reforged);
  assert_eq!(info.num_players, 2);
  assert_eq!(info.num_forces, 1);
  dbg!("{:#?}", info);
}

#[test]
fn test_parse_w3i_roc() {
  let mut map = crate::open_archive(flo_util::sample_path!("map", "test_roc.w3m")).unwrap();
  let bytes = map.open_file("war3map.w3i").unwrap().read_all().unwrap();
  let mut buf = bytes.as_slice();
  let info = MapInfo::decode(&mut buf).unwrap();
  assert_eq!(info.version, MapFormatVersion::ROC);
  //assert_eq!(info.num_players, 0);
  assert_eq!(info.num_forces, 1);
  dbg!("{:#?}", info);
}

#[test]
fn test_parse_w3i_tft() {
  let mut map = crate::open_archive(flo_util::sample_path!("map", "test_tft.w3x")).unwrap();
  let bytes = map.open_file("war3map.w3i").unwrap().read_all().unwrap();
  let mut buf = bytes.as_slice();
  let info = MapInfo::decode(&mut buf).unwrap();
  assert_eq!(info.version, MapFormatVersion::TFT);
  //assert_eq!(info.num_players, 0);
  assert_eq!(info.num_forces, 1);
  dbg!("{:#?}", info);
}

#[test]
fn test_parse_custom() {
  let mut map = crate::open_archive(flo_util::sample_path!(
    "map",
    "Impossible.Bosses.v1.10.5.w3x"
  ))
  .unwrap();
  let bytes = map.open_file("war3map.w3i").unwrap().read_all().unwrap();
  let mut buf = bytes.as_slice();
  let info = MapInfo::decode(&mut buf).unwrap();
  // assert_eq!(info.version, MapFormatVersion::TFT);
  // assert_eq!(info.num_players, 0);
  // assert_eq!(info.num_forces, 1);
  dbg!("{:#?}", info);
}

#[test]
fn test_fixed_player() {
  let mut map = crate::open_archive(flo_util::sample_path!(
    "map",
    "fixed_player.w3m"
  ))
  .unwrap();
  let bytes = map.open_file("war3map.w3i").unwrap().read_all().unwrap();
  let mut buf = bytes.as_slice();
  let info = MapInfo::decode(&mut buf).unwrap();
  // assert_eq!(info.version, MapFormatVersion::TFT);
  // assert_eq!(info.num_players, 0);
  // assert_eq!(info.num_forces, 1);
  dbg!("{:#?}", info);
}