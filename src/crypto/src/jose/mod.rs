pub mod error;
pub mod header;
pub mod jwe;
pub mod jwk;
pub mod jws;
pub mod util;
use std::fmt::Debug;

use serde_json::Value;

pub trait JoseHeader: Send + Sync + Debug {
    fn len(&self) -> usize;

    fn claim(&self, key: &str) -> Option<&Value>;

    fn box_clone(&self) -> Box<dyn JoseHeader>;
}

impl Clone for Box<dyn JoseHeader> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub trait Jose {
    fn header(&self) -> &dyn JoseHeader;
}
