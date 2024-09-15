mod sclmanager;

pub mod sclang;

pub mod load_test;

pub mod scl {
    pub use crate::sclmanager::create_cell_with_links;
    pub use crate::sclmanager::create_cell_with_text_only;
    pub use crate::sclmanager::enable_feature;
    pub use crate::sclmanager::SCLRef;
}
