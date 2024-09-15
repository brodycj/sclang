mod sc_record_manager;

pub mod sclang;

pub mod load_test;

pub mod scl {
    pub use crate::sc_record_manager::enable_feature;
    pub use crate::sc_record_manager::is_debug_enabled;
    pub use crate::sc_record_manager::SCRecordRef;
}
