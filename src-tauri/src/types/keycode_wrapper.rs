use keybind::Keycode;
use serde::{self, Deserialize, Deserializer};
use std::collections::HashMap;

pub struct MyKeycode(Keycode);
impl MyKeycode {
    pub fn keycode(&self) -> &Keycode {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MyKeycode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let keycode =
            map_string_to_keycode(&s).ok_or(serde::de::Error::custom("Invalid keycode"))?;
        Ok(MyKeycode(keycode))
    }
}

fn map_string_to_keycode(s: &str) -> Option<Keycode> {
    let mut map = HashMap::new();
    map.insert("A", Keycode::A);
    map.insert("B", Keycode::B);
    // Add the rest of the keys here...
    map.get(s).cloned()
}
