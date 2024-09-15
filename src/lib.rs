mod sclmanager;

pub mod sclang;

pub mod load_test;

pub mod scl {
    pub use crate::sclmanager::enable_feature;
    pub use crate::sclmanager::is_debug_enabled;
    pub use crate::sclmanager::SCRecordRef;
}
