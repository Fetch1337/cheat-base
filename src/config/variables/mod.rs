use serde::{Deserialize, Serialize};

use crate::utilities::input::{BindType, KeyBinds};

#[derive(Serialize, Deserialize, Clone)]
pub struct Variables {
    pub menu_key: KeyBinds,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            menu_key: KeyBinds::new(0x2D, BindType::Toggle),
        }
    }
}
