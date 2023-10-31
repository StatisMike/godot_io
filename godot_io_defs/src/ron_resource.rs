use godot::{
  prelude::{GodotClass, GodotString, Variant, Gd, godot_print, ToGodot, Inherits, Resource}, 
  obj::dom::UserDomain, 
  engine::{file_access::ModeFlags, FileAccess, global::Error}
};
use ron::{ser, de};
use serde::{Serialize, Deserialize};

use crate::{GD_RON_START, GD_RON_END};

struct GdRonResourceHeader {
  pub name: String,
  pub uid: String,
}

impl GdRonResourceHeader {
  const HEADER_PATTERN: &str = "gd=[{}]=uid=[{}]";

  pub fn new(name: String, uid: String) -> Self {
    Self {
      name,
      uid
    }
  }

  // pub fn from_str(line: &str) -> Self {
    
  // }
}

fn create_gd_ron_resource_header(name: &str, uid: &str) -> String {
  format!("gd=[{}]=uid=[{}]", name, uid)
}

/// Trait which provides methods to serialize and deserialize
/// rust-defined [Resource](godot::engine::Resource) to `gdron` files, 
/// based on [ron]
pub trait GdRonResource
where 
Self: Serialize + for<'de> Deserialize<'de> + GodotClass<Declarer = UserDomain> + Inherits<Resource> {


  /// Struct identifier included in `gdron` file
  const RON_FILE_HEAD_IDENT: &'static str;

  /// Save object to a file located at `path` in [ron] format
  /// ## Arguments
  /// - `path`: [GodotString] - path to the file
  fn save_ron(&self, path: GodotString) -> Error {
    if let Some(access) = &mut FileAccess::open(path.clone(), ModeFlags::WRITE) {

      if let Ok(serialized) = ser::to_string_pretty(self, ser::PrettyConfig::default()) {
        access.store_string(GodotString::from(format!("{}{}{}\n", GD_RON_START, Self::RON_FILE_HEAD_IDENT, GD_RON_END)));
        access.store_string(GodotString::from(serialized));
        access.close();
        return Error::OK;
      } 
      return Error::ERR_CANT_CREATE;
    } 
    Error::ERR_FILE_CANT_WRITE
  }

  /// Load object from a file located at `path` in [ron] format
  /// ## Arguments
  /// - `path`: [GodotString] - path to the file
  fn load_ron(path: GodotString) -> Variant {
    if let Some(access) = FileAccess::open(path.clone(), ModeFlags::READ) {
      let serialized = access.get_as_text().to_string();
      let end_line = serialized.find('\n').unwrap();
      let res = de::from_str::<Self>(&serialized[end_line+1..serialized.len()]);
      match res {
        Ok(loaded) => return Gd::new(loaded).to_variant(),
        Err(error) => {
          godot_print!("{}", error);
          return Error::ERR_FILE_CANT_READ.to_variant();
        },
      }
    }
    Error::ERR_FILE_CANT_OPEN.to_variant()
  }
}