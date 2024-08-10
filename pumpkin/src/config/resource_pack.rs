use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResourcePack {
    pub enabled: bool,
    /// The path to the resource pack.
    pub resource_pack_url: String,
    /// The SHA1 hash (40) of the resource pack.
    pub resource_pack_sha1: String,
    /// Custom propmt Text component, Leave blank for none
    pub prompt_message: String,
    /// Will force the Player to accept the resource pack
    pub force: bool,
}

impl ResourcePack {
    pub fn validate(&self) {
        assert_eq!(
            !self.resource_pack_url.is_empty(),
            !self.resource_pack_sha1.is_empty(),
            "Resource Pack path or Sha1 hash is missing"
        );
        assert!(
            self.resource_pack_sha1.len() <= 40,
            "Resource pack sha1 hash is too long (max. 40)"
        )
    }
}

impl Default for ResourcePack {
    fn default() -> Self {
        Self {
            enabled: false,
            resource_pack_url: "".into(),
            resource_pack_sha1: "".into(),
            force: false,
            prompt_message: "".into(),
        }
    }
}