// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Name Characteristic.
pub type Name = Characteristic<String>;

/// Creates a new Name Characteristic.
pub fn new() -> Name {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::Name,
        format: Format::String,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}