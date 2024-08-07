use std::fmt::Write;
use std::sync::{Arc, RwLock, Weak};

#[derive(Clone)]
pub struct SCLCursor {
    outer_wrapper_ref: OuterCellWrapperRcRef,
}

type OuterCellWrapperRcRef = RcRef<OuterSCWrapper>;

// XXX TODO EXPLAIN RATIONALE FOR THESE PRIVATE TYPE ALIASES
type RwCell<T> = RwLock<T>;
type RcRef<T> = Arc<T>;
type WeakRef<T> = Weak<T>;
type WeakRefCell<T> = RwCell<WeakRef<T>>;

// XXX SHARED DATA - XXX TODO IMPROVE INTERNAL API
static scl_debug: RwCell<bool> = RwCell::new(false);

pub fn is_debug_enabled() -> bool {
    *scl_debug.read().unwrap()
}

// XXX TODO IMPROVE NAMING CONSISTENCY HERE

struct OuterSCWrapper {
    outer_middle_sc_wrapper: RwCell<MiddleSCWrapperRcRef>,
    inner_sc_data_storage: InnerSCDataStorageRcRef,
}

type MiddleSCWrapperRcRef = RcRef<MiddleSCWrapper>;

// XXX TBD RECONSIDER NAMING ??? ???
struct MiddleSCWrapper {
    sc_data_storage: InnerSCDataStorageRcRef,
    // XXX TODO EXPLAIN RATIONALE FOR THIS STRONG REF OPTION
    peer_sc_linkage_info_strong_ref: RwCell<Option<RcRef<SCLinkageInfo>>>,
    outer_wrapper_ref: WeakRefCell<OuterSCWrapper>,
    // XXX TODO "next middle wrapper" ref may lead to retaining excessive middle wrapper objects if SC data is updated over & over (possible memory leakage)
    // XXX TODO LIKELY NEED BETTER SOLUTION FOR THIS
    next_middle_wrapper: RwCell<Option<MiddleSCWrapperRcRef>>,
    // XXX TBD RECONSIDER WHAT STATE TO KEEP HERE - ??? ??? ???
    // extra_prev_middle_wrapper_ref: WeakRefCell<MiddleCellWrapper>,
    // XXX TBD RECONSIDER KEEPING THIS STATE HERE - ??? ??? ???
    inner_middle_wrapper: RwCell<Option<MiddleSCWrapperRcRef>>,
}

// XXX TBD RECONSIDER NAMING ??? ???
struct SCLinkageInfo {
    // XXX TBD RECONSIDER WHAT TO STORE HERE
    sc_data_storage: InnerSCDataStorageRcRef,
    linkage1: (Option<InnerSCDataStorageRcRef>, Option<MiddleSCWrapperRcRef>, Option<MiddleSCWrapperRcRef>),
    linkage2: (Option<InnerSCDataStorageRcRef>, Option<MiddleSCWrapperRcRef>, Option<MiddleSCWrapperRcRef>),
}

type InnerSCDataStorageRcRef = RcRef<InnerSCDataStorage>;

struct InnerSCDataStorage {
    text1: RcRef<RwCell<String>>,
    text2: RcRef<RwCell<String>>,
    outer_wrapper_ref: WeakRefCell<OuterSCWrapper>,
    // XXX TBD NAMING OF THIS ??? ???
    peer_sc_linkage_ref: WeakRefCell<SCLinkageInfo>,
    // XXX TRACK WHERE THE STRONG PEER SC LINKAGE REF IS STORED - XXX TODO IMPROVE NAMING & CLARIFY DESCRIPTION OF THIS FIELD
    peer_sc_linkage_middle_wrapper_ref: WeakRefCell<MiddleSCWrapper>,
    // XXX TBD RECONSIDER KEEPING THIS STATE HERE - ??? ??? ???
    inner_middle_sc_wrapper_ref: WeakRefCell<MiddleSCWrapper>,
}

static drop_sc_data_count: RwCell<i32> = RwCell::new(0);

impl Drop for InnerSCDataStorage {
    fn drop(&mut self) {
        if is_debug_enabled() {
            println!("DROP SC DATA with stored info fields:");
            println!("- text 1: {}", self.text1.read().unwrap());
            println!("- text 2: {}", self.text2.read().unwrap());
        }
        let mut x = drop_sc_data_count.write().unwrap();
        *x = *x + 1;
        drop(x);
        if is_debug_enabled() {
            println!("DROP SC DATA COUNT: {}", get_stat_drop_sc_data_count());
            println!("--- --- ---");
        }
    }
}

pub fn get_stat_drop_sc_data_count() -> i32 {
    drop_sc_data_count.read().unwrap().clone()
}

pub fn reset_stat_drop_sc_data_count() {
    let mut x = drop_sc_data_count.write().unwrap();
    *x = 0;
}

impl Drop for MiddleSCWrapper {
    fn drop(&mut self) {
        if is_debug_enabled() {
            println!("DROP MIDDLE CELL WRAPPER for CELL DATA with info");
            println!("- text 1: {}", self.sc_data_storage.text1.read().unwrap());
            println!("- text 2: {}", self.sc_data_storage.text2.read().unwrap());
            println!("--- --- ---");
        }

        // XXX TODO: EXPLAIN RATIONALE & HOW THIS WORKS

        let maybe_next_middle_wrapper = self.next_middle_wrapper.read().unwrap();
        if maybe_next_middle_wrapper.is_none() {
            return;
        }

        let maybe_inner_sc_linkage = self.peer_sc_linkage_info_strong_ref.read().unwrap().clone();
        if maybe_inner_sc_linkage.is_none() {
            return;
        }

        let next_middle_wrapper_ref = maybe_next_middle_wrapper.as_ref().unwrap().clone();

        if next_middle_wrapper_ref.next_middle_wrapper.read().unwrap().is_none() {
            return;
        }

        // NOTE: THIS CODE REQUIRES QUICK & UGLY WORKAROUND IN CREATE CELL API FN CODE FURTHER BELOW - XXX TODO NEED TO EXPLAIN THIS
        // XXX TODO LOOK FOR A WAY TO IMPROVE THIS

        // XXX TBD NOTE: THIS LINKED INFO STORAGE FETCH IS SIMPLIFIED BY STORING THIS INFO IN THE LINKAGE - XXX TBD IS THIS INFO WORTH STORING ??? ??? ????
        let linked_sc_data_storage_ref1 = maybe_inner_sc_linkage.clone().unwrap().linkage1.clone().0.unwrap();
        let linked_sc_data_storage_ref2 = maybe_inner_sc_linkage.clone().unwrap().linkage2.clone().0.unwrap();

        let inner_sc_linkage_ref = RcRef::new(SCLinkageInfo {
            // XXX TODO UTILITY FN
            sc_data_storage: self.sc_data_storage.clone(),
            linkage1: (
                Some(linked_sc_data_storage_ref1.clone()),
                linked_sc_data_storage_ref1.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
                linked_sc_data_storage_ref1.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
            ),
            linkage2: (
                Some(linked_sc_data_storage_ref2.clone()),
                linked_sc_data_storage_ref2.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
                linked_sc_data_storage_ref2.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
            ),
        });

        *next_middle_wrapper_ref.peer_sc_linkage_info_strong_ref.write().unwrap() = Some(inner_sc_linkage_ref.clone());

        *self.sc_data_storage.peer_sc_linkage_ref.write().unwrap() = RcRef::downgrade(&inner_sc_linkage_ref);
    }
}

impl OuterSCWrapper {
    fn create_with_cell_data(text1: &str, text2: &str, link1: Option<MiddleSCWrapperRcRef>, link2: Option<MiddleSCWrapperRcRef>) -> OuterCellWrapperRcRef {
        let middle_sc_wrapper_ref = MiddleSCWrapper::create_with_inner_cell_data(text1, text2, link1, link2);
        let outer_wrapper_ref = RcRef::new(OuterSCWrapper {
            outer_middle_sc_wrapper: RwCell::new(middle_sc_wrapper_ref.clone()),
            inner_sc_data_storage: middle_sc_wrapper_ref.sc_data_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_sc_wrapper_writer = middle_sc_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *middle_sc_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn create_with_middlewrapper(middle_sc_wrapper_ref: MiddleSCWrapperRcRef) -> OuterCellWrapperRcRef {
        let outer_wrapper_ref = RcRef::new(OuterSCWrapper {
            outer_middle_sc_wrapper: RwCell::new(middle_sc_wrapper_ref.clone()),
            inner_sc_data_storage: middle_sc_wrapper_ref.sc_data_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_sc_wrapper_writer = middle_sc_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *middle_sc_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn update_sc_linkage(outer_sc_wrapper_ref: OuterCellWrapperRcRef, link1: Option<MiddleSCWrapperRcRef>, link2: Option<MiddleSCWrapperRcRef>) {
        let middle_sc_wrapper_ref =
            MiddleSCWrapper::create_with_next_middle_sc_wrapper_data(outer_sc_wrapper_ref.outer_middle_sc_wrapper.read().unwrap().clone(), link1, link2);
        // XXX TBD RECONSIDER EXTRA REF CLONE HERE
        let next_middle_sc_wrapper_ref = middle_sc_wrapper_ref.clone();
        let mut next_middle_sc_wrapper_writer = next_middle_sc_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *next_middle_sc_wrapper_writer = RcRef::downgrade(&outer_sc_wrapper_ref);
        let mut middle_sc_wrapper_writer = outer_sc_wrapper_ref.outer_middle_sc_wrapper.write().unwrap();
        *middle_sc_wrapper_writer = middle_sc_wrapper_ref.clone();
    }

    fn ref_middle_sc_wrapper_ref(middle_sc_wrapper_ref: MiddleSCWrapperRcRef) -> OuterCellWrapperRcRef {
        // XXX TODO USE MATCH INSTEAD HERE
        let mut my_outer_wrapper_ref = middle_sc_wrapper_ref.outer_wrapper_ref.read().unwrap().upgrade();
        if my_outer_wrapper_ref.is_none() {
            OuterSCWrapper::create_with_middlewrapper(middle_sc_wrapper_ref)
        } else {
            my_outer_wrapper_ref.unwrap()
        }
    }
}

impl MiddleSCWrapper {
    fn create_with_inner_cell_data(text1: &str, text2: &str, link1: Option<MiddleSCWrapperRcRef>, link2: Option<MiddleSCWrapperRcRef>) -> MiddleSCWrapperRcRef {
        // XXX TBD MUTABLE - ??? ??? ???
        let inner_sc_data_storage = InnerSCDataStorage::create_with_inner_text_fields(text1, text2);

        // XXX TBD RECONSIDER EXTRA CLONE - ???
        let inner_sc_data_storage_ref = inner_sc_data_storage.clone();

        let cell_linkage_strong_ref = RwCell::new(Some(SCLinkageInfo::create_with_full_info(
            inner_sc_data_storage_ref.clone(),
            link1.clone(),
            link2.clone(),
        )));

        // KEEP XXX XXX INFO IN SYNC HERE
        let mut cell_linkage_weak_writer = inner_sc_data_storage_ref.peer_sc_linkage_ref.write().unwrap();
        *cell_linkage_weak_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_cw_ref = RcRef::new(MiddleSCWrapper {
            sc_data_storage: inner_sc_data_storage.clone(),
            peer_sc_linkage_info_strong_ref: cell_linkage_strong_ref,
            outer_wrapper_ref: RwCell::new(WeakRef::new()),
            next_middle_wrapper: RwCell::new(None),
            // extra_prev_middle_wrapper_ref: RwCell::new(WeakRef::new()),
            inner_middle_wrapper: RwCell::new(None),
        });

        *inner_sc_data_storage.peer_sc_linkage_middle_wrapper_ref.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());
        *inner_sc_data_storage.inner_middle_sc_wrapper_ref.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());

        return middle_cw_ref;
    }

    fn create_with_next_middle_sc_wrapper_data(
        next_middle_wrapper: MiddleSCWrapperRcRef,
        link1: Option<MiddleSCWrapperRcRef>,
        link2: Option<MiddleSCWrapperRcRef>,
    ) -> MiddleSCWrapperRcRef {
        let inner_sc_data_storage = next_middle_wrapper.clone().sc_data_storage.clone();

        // XXX TODO RECONSIDER EXTRA REF CLONE HERE
        let inner_sc_data_storage_ref = inner_sc_data_storage.clone();

        let cell_linkage_strong_ref = RwCell::new(Some(SCLinkageInfo::create_with_full_info(
            inner_sc_data_storage_ref.clone(),
            link1.clone(),
            link2.clone(),
        )));

        let inner_middle_wrapper = inner_sc_data_storage_ref.inner_middle_sc_wrapper_ref.read().unwrap().upgrade();

        // KEEP XXX XXX INFO IN SYNC HERE
        let mut cell_linkage_weak_ref_writer = inner_sc_data_storage_ref.peer_sc_linkage_ref.write().unwrap();
        *cell_linkage_weak_ref_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_wrapper_ref = RcRef::new(MiddleSCWrapper {
            sc_data_storage: inner_sc_data_storage.clone(),
            peer_sc_linkage_info_strong_ref: cell_linkage_strong_ref,
            outer_wrapper_ref: RwLock::new(next_middle_wrapper.clone().outer_wrapper_ref.read().unwrap().clone()),
            // extra_prev_middle_wrapper_ref: RwLock::new(WeakRef::new()),
            next_middle_wrapper: RwCell::new(Some(next_middle_wrapper.clone())),
            inner_middle_wrapper: RwCell::new(inner_middle_wrapper),
        });

        let old_linkage_strong_ref_wrapper = inner_sc_data_storage_ref.peer_sc_linkage_middle_wrapper_ref.read().unwrap().upgrade().clone();
        *inner_sc_data_storage_ref.peer_sc_linkage_middle_wrapper_ref.write().unwrap() = RcRef::downgrade(&middle_wrapper_ref.clone());

        // clear out XXX XXX
        if old_linkage_strong_ref_wrapper.is_some() {
            *old_linkage_strong_ref_wrapper.unwrap().peer_sc_linkage_info_strong_ref.write().unwrap() = None;
        }

        // XXX TBD FUTURE CONSIDERATION ??? ??? ???
        // UPDATE XXX XXX XXX
        // *next_middle_wrapper.clone().extra_prev_middle_wrapper_ref.write().unwrap() = RcRef::downgrade(&middle_wrapper_ref);

        middle_wrapper_ref
    }

    fn get_text1(&self) -> String {
        self.sc_data_storage.text1.read().unwrap().clone()
    }

    fn get_text2(&self) -> String {
        self.sc_data_storage.text2.read().unwrap().clone()
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    fn update_cell_text_data(&self, text1: &str, text2: &str) {
        let mut xxx1 = self.sc_data_storage.text1.write().unwrap();
        *xxx1 = String::from(text1);
        let mut xxx2 = self.sc_data_storage.text2.write().unwrap();
        *xxx2 = String::from(text2);
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    fn clear_inner_sc_linkage_strong_ref(&self) {
        let mut inner_sc_linkage_strong_ref_writer = self.peer_sc_linkage_info_strong_ref.write().unwrap();
        *inner_sc_linkage_strong_ref_writer = None;
    }
}

impl SCLinkageInfo {
    // XXX TBD SUPPORT CREATE API FN WITH EMPTY LINKS ???
    fn create_with_full_info(
        inner_sc_data_storage_ref: InnerSCDataStorageRcRef,
        middle_wrapper_link1: Option<MiddleSCWrapperRcRef>,
        middle_wrapper_link2: Option<MiddleSCWrapperRcRef>,
    ) -> RcRef<SCLinkageInfo> {
        // XXX TBD ARE THESE SEPARATE REF CLONES REALLY NEEDED ??? ??? ???
        let keep_middle_wrapper_link1 = middle_wrapper_link1.clone();
        let keep_middle_wrapper_link2 = middle_wrapper_link2.clone();
        // XXX TBD STORE THESE FOR NOW ... ... XXX TBD ??? ??? / XXX XXX SHOULD COMBINE MATCH STATEMENTS TOGETHER IF PRACTICAL
        let linked_sc_data_storage_ref1 = match middle_wrapper_link1.clone() {
            None => None,
            Some(m) => Some(m.sc_data_storage.clone()),
        };
        let linked_sc_data_storage_ref2 = match middle_wrapper_link2.clone() {
            None => None,
            Some(m) => Some(m.sc_data_storage.clone()),
        };
        // XXX TBD UTIL FN ???s
        let inner_middle_sc_wrapper_link1 = match middle_wrapper_link1 {
            None => None,
            Some(m) => m.sc_data_storage.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
        };
        let inner_middle_sc_wrapper_link2 = match middle_wrapper_link2 {
            None => None,
            Some(m) => m.sc_data_storage.inner_middle_sc_wrapper_ref.read().unwrap().upgrade(),
        };
        // XXX TODO NOTE: XXX XXX XXX
        RcRef::new(SCLinkageInfo {
            sc_data_storage: inner_sc_data_storage_ref,
            linkage1: (linked_sc_data_storage_ref1, inner_middle_sc_wrapper_link1, keep_middle_wrapper_link1),
            linkage2: (linked_sc_data_storage_ref2, inner_middle_sc_wrapper_link2, keep_middle_wrapper_link2),
        })
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_middle_wrapper_link1(&self) -> Option<MiddleSCWrapperRcRef> {
        self.linkage1.1.clone()
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_middle_wrapper_link2(&self) -> Option<MiddleSCWrapperRcRef> {
        self.linkage2.1.clone()
    }
}

impl InnerSCDataStorage {
    fn create_with_inner_text_fields(text1: &str, text2: &str) -> InnerSCDataStorageRcRef {
        RcRef::new(InnerSCDataStorage {
            text1: RcRef::new(RwCell::new(String::from(text1))),
            text2: RcRef::new(RwCell::new(String::from(text2))),
            outer_wrapper_ref: RwCell::new(WeakRef::new()),
            peer_sc_linkage_ref: RwCell::new(WeakRef::new()),
            peer_sc_linkage_middle_wrapper_ref: RwCell::new(WeakRef::new()),
            inner_middle_sc_wrapper_ref: RwCell::new(WeakRef::new()),
        })
    }

    // XXX TBD KEEP & USE THIS ??? ???
    fn get_text1(&self) -> String {
        self.text1.read().unwrap().clone()
    }

    // XXX TBD KEEP & USE THIS ??? ???
    fn get_text2(&self) -> String {
        self.text2.read().unwrap().clone()
    }
}

impl SCLCursor {
    pub fn get_text1(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.outer_wrapper_ref.inner_sc_data_storage.get_text1()
    }

    pub fn get_text2(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.outer_wrapper_ref.inner_sc_data_storage.get_text2()
    }

    pub fn get_link1(&self) -> Option<SCLCursor> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE
        let sc_linkage_info_ref = self.outer_wrapper_ref.inner_sc_data_storage.peer_sc_linkage_ref.read().unwrap().upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_sc_wrapper_ref = sc_linkage_info_ref.unwrap().linkage1.1.clone();
        match maybe_linked_middle_sc_wrapper_ref {
            None => None,
            Some(middle_sc_wrapper_ref) => Some(SCLCursor::from_outer_sc_wrapper(OuterSCWrapper::ref_middle_sc_wrapper_ref(
                middle_sc_wrapper_ref,
            ))),
        }
    }

    pub fn get_link2(&self) -> Option<SCLCursor> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE
        let sc_linkage_info_ref = self.outer_wrapper_ref.inner_sc_data_storage.peer_sc_linkage_ref.read().unwrap().upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_sc_wrapper_ref = sc_linkage_info_ref.unwrap().linkage2.1.clone();
        match maybe_linked_middle_sc_wrapper_ref {
            None => None,
            Some(middle_sc_wrapper_ref) => Some(SCLCursor::from_outer_sc_wrapper(OuterSCWrapper::ref_middle_sc_wrapper_ref(
                middle_sc_wrapper_ref,
            ))),
        }
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    pub fn update_data(&self, text1: &str, text2: &str, link1: Option<SCLCursor>, link2: Option<SCLCursor>) {
        let my_middle_sc_wrapper_ref = self.get_middle_sc_wrapper();

        my_middle_sc_wrapper_ref.update_cell_text_data(text1, text2);

        let my_outer_wrapper_ref = self.outer_wrapper_ref.clone();
        OuterSCWrapper::update_sc_linkage(
            my_outer_wrapper_ref,
            match link1 {
                Some(r) => Some(r.get_middle_sc_wrapper()),
                None => None,
            },
            match link2 {
                Some(r) => Some(r.get_middle_sc_wrapper()),
                None => None,
            },
        );
    }

    pub fn get_dump(&self) -> String {
        let mut dump = String::new();
        // XXX TODO REFACTOR TO AVOID REPEATED XXX XXX
        // XXX TODO CAPTURE OR EXPLICITLY IGNORE WRITE RESULT
        writeln!(&mut dump, "- text 1: {}", self.get_text1());
        writeln!(&mut dump, "- text 2: {}", self.get_text2());
        if self.get_link1().is_some() {
            let link1 = self.get_link1().unwrap();
            writeln!(&mut dump, "- link 1 info:");
            writeln!(&mut dump, "  link 1 info - text 1: {}", link1.get_text1());
            writeln!(&mut dump, "  link 1 info - text 2: {}", link1.get_text2());
            if link1.get_link1().is_some() {
                writeln!(&mut dump, "  - link 1 -> link 1 info - text only:");
                writeln!(&mut dump, "    link 1 -> link 1 info - text 1: {}", link1.get_link1().unwrap().get_text1());
                writeln!(&mut dump, "    link 1 -> link 1 info - text 2: {}", link1.get_link1().unwrap().get_text2());
            } else {
                writeln!(&mut dump, "  - link 1 -> link 1 - empty");
            }
            if link1.get_link2().is_some() {
                writeln!(&mut dump, "  - link 1 -> link 2 info - text only:");
                writeln!(&mut dump, "    link 1 -> link 2 info - text 1: {}", link1.get_link2().unwrap().get_text1());
                writeln!(&mut dump, "    link 1 -> link 2 info - text 2: {}", link1.get_link2().unwrap().get_text2());
            } else {
                writeln!(&mut dump, "  - link 1 -> link 2 - empty");
            }
        } else {
            writeln!(&mut dump, "- link 1 - empty");
        }
        if self.get_link2().is_some() {
            let link2 = self.get_link2().unwrap();
            writeln!(&mut dump, "- link 2 info:");
            writeln!(&mut dump, "  link 2 info - text 1: {}", link2.get_text1());
            writeln!(&mut dump, "  link 2 info - text 2: {}", link2.get_text2());
            if link2.get_link1().is_some() {
                writeln!(&mut dump, "  - link 2 -> link 1 info - text only:");
                writeln!(&mut dump, "    link 2 -> link 1 info - text 1: {}", link2.get_link1().unwrap().get_text1());
                writeln!(&mut dump, "    link 2 -> link 1 info - text 2: {}", link2.get_link1().unwrap().get_text2());
            } else {
                writeln!(&mut dump, "  - link 2 -> link 1 - empty");
            }
            if link2.get_link2().is_some() {
                writeln!(&mut dump, "  - link 2 -> link 2 info - text only:");
                writeln!(&mut dump, "    link 2 -> link 2 info - text 1: {}", link2.get_link2().unwrap().get_text1());
                writeln!(&mut dump, "    link 2 -> link 2 info - text 2: {}", link2.get_link2().unwrap().get_text2());
            } else {
                writeln!(&mut dump, "  - link 2 -> link 2 - empty");
            }
        } else {
            writeln!(&mut dump, "- link 2 - empty");
        }
        dump
    }

    fn get_middle_sc_wrapper(&self) -> MiddleSCWrapperRcRef {
        self.outer_wrapper_ref.outer_middle_sc_wrapper.read().unwrap().clone()
    }

    fn from_outer_sc_wrapper(outer_wrapper_ref: OuterCellWrapperRcRef) -> SCLCursor {
        SCLCursor { outer_wrapper_ref }
    }
}

pub fn create_cell_with_text_only(text1: &str, text2: &str) -> SCLCursor {
    // XXX QUICK & UGLY WORKAROUND FOR XXX XXX IN MIDDLE CELL WRAPPER DROP FUNCTION ABOVE
    let x = SCLCursor::from_outer_sc_wrapper(OuterSCWrapper::create_with_cell_data(text1, text2, None, None));
    x.update_data(text1, text2, None, None);
    x
}

pub fn create_cell_with_links(text1: &str, text2: &str, link1: SCLCursor, link2: SCLCursor) -> SCLCursor {
    // XXX QUICK & UGLY WORKAROUND FOR XXX XXX IN MIDDLE CELL WRAPPER DROP FUNCTION ABOVE
    // XXX (NO NEED TO STORE LINK UNTIL UPDATING STORED DATA WITH THIS UGLY WORKAROUND)
    // XXX TODO USE UTIL FN HERE
    let x = SCLCursor::from_outer_sc_wrapper(OuterSCWrapper::create_with_cell_data(text1, text2, None, None));
    x.update_data(text1, text2, Some(link1), Some(link2));
    x
}

pub fn enable_feature(feature_name: &str) {
    match feature_name {
        "debug" => {
            *scl_debug.write().unwrap() = true;
            println!("DEBUG ENABLED");
        }
        _ => {
            println!("UNKNOWN FEATURE - IGNORED: {}", feature_name);
        }
    }
}
