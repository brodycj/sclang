use std::fmt::Write;
use std::sync::{Arc, RwLock, Weak};

#[derive(Clone)]
pub struct SCDataRecordRef(StrongSCDataRecordManagerRcRef);

// XXX TBD TEMPORARY ALIASING - XXX TODO IMPROVE INTERNAL struct naming
type StrongSCDataRecordManagerRcRef = OuterCellWrapperRcRef;

type OuterCellWrapperRcRef = RcRef<OuterCellWrapper>;

// NOTE: While these type aliases may be ready to provide multi-threaded safety, the overall design
// of this library is not yet ready to be considered safe for multi-threaded access.
// These type alias may be updated to use types that work & work better with no-std,
// as may be provided by third-party crate(s), someday in the future.
type RwValue<T> = RwLock<T>;
type RcRef<T> = Arc<T>;
type WeakRef<T> = Weak<T>;
type RwWeakRef<T> = RwValue<WeakRef<T>>;

// XXX SHARED DATA - XXX TODO IMPROVE INTERNAL API
static scl_debug: RwValue<bool> = RwValue::new(false);

pub fn is_debug_enabled() -> bool {
    *scl_debug.read().unwrap()
}

// XXX TODO IMPROVE NAMING CONSISTENCY HERE

// XXX TODO EXPLAIN RATIONALE FOR THESE WRAPPERS
// XXX QUICK EXPLANATION OF WRAPPERS:
// OUTER WRAPPER (as aliased to: StrongSCDataRecordManagerRcRef) IS USED BY SCDataRecordRef TO KEEP STRONG REFERENCE & HELP MAINTAIN LIFETIME
// MIDDLE WRAPPER IS ACTUALLY MIDDLE LIFETIME WRAPPER TO HELP MAINTAIN LIFETIME WITHOUT ANY STRONG REFERENCE CYCLES

// XXX TODO NEED TO RECONSIDER BOTH NAMING AND HOW MUCH DESCRIPTIVE TEXT TO KEEP OR UPDATE;
// HOPEFULLY BETTER NAMING CAN REDUCE THE NEED FOR SOME OF THE DESCRIPTIVE TEXT HERE

struct OuterCellWrapper {
    // NOTE: SCDataRecordRef keeps a reference to this "outer wrapper", which also acts as a strong top-level lifetime manager
    // for SC data record objects which are *indirectly* referenced by SCDataRecordRef objects.
    // -- END OF NOTE
    // This wrapper contains the following important fields:
    // ** middle_cell_wrapper - references the outer-most middle lifetime wrapper (OUTER-MIDDLE LIFETIME WRAPPER),
    //    which is expected to keep the strong reference to the SC linkage info
    //    (until it is superceded by a newer outer-most middle lifetime wrapper).
    //    XXX TODO NEED TO RENAME THIS FIELD TO BE MORE CLEAR & DESCRIPTIVE!!
    middle_cell_wrapper: RwValue<MiddleCellWrapperRcRef>,
    // ** inner_sc_info_storage_ref - references (strong reference) where the data (text data) fields are stored,
    //    which in turn does keep a weak reference to the peer linkage
    inner_sc_info_storage_ref: InnerSCInfoStorageRcRef,
}

type MiddleCellWrapperRcRef = RcRef<MiddleCellWrapper>;

// XXX TODO RECONSIDER NAMING FOR THIS LIFETIME MANAGER
struct MiddleCellWrapper {
    // NOTE: This is an object lifetime manager "wrapper" that helps keep data objects alive exactly as long as they are
    // directly or indirectly reachable from the outside via using SCDataRecordRef objects.
    // Keeping multiple levels of lifetime manager wrappers helps avoid strong circular references & allow
    // unreadable SCL data cell objects to be automatically dropped & cleaned up once they are no longer reachable from the outside.
    // XXX TODO NEED GOOD EXPLANATION OF THE STRATEGY FOR THIS !!!
    // --- END OF NOTE
    inner_sc_info_storage: InnerSCInfoStorageRcRef,
    // XXX TODO EXPLAIN RATIONALE FOR THIS STRONG REF OPTION
    // QUICK RATIONALE: outer-most middle lifetime wrapper is expected to keep the strong reference to the SC linkage info
    // (until it is superceded by a newer outer-most middle lifetime wrapper)
    inner_sc_linkage_info_strong_ref: RwValue<Option<RcRef<InnerSCLinkageInfo>>>,
    outer_wrapper_ref: RwWeakRef<OuterCellWrapper>,
    // XXX TODO EXPLAIN HOW THIS WORKS
    // XXX TBD NAMING - perhaps as "previous middle wrapper" / "older middle wrapper" / etc.
    next_middle_wrapper: RwValue<Option<MiddleCellWrapperRcRef>>,
}

// XXX TBD RECONSIDER NAMING ??? ???
struct InnerSCLinkageInfo {
    link1: Option<MiddleCellWrapperRcRef>,
    link2: Option<MiddleCellWrapperRcRef>,
}

type InnerSCInfoStorageRcRef = RcRef<InnerSCInfoStorage>;

struct InnerSCInfoStorage {
    text1: RcRef<RwValue<String>>,
    text2: RcRef<RwValue<String>>,
    outer_wrapper_ref: RwWeakRef<OuterCellWrapper>,
    // XXX TBD NAMING OF THIS ??? ???
    sc_linkage_info_weak_ref: RwWeakRef<InnerSCLinkageInfo>,
    linkage_strong_ref_wrapper: RwWeakRef<MiddleCellWrapper>,
    inner_middle_cell_wrapper_ref: RwWeakRef<MiddleCellWrapper>,
}

static drop_cell_count: RwValue<i32> = RwValue::new(0);

impl Drop for InnerSCInfoStorage {
    fn drop(&mut self) {
        if is_debug_enabled() {
            println!("DROP CELL DATA with info:");
            println!("- text 1: {}", self.text1.read().unwrap());
            println!("- text 2: {}", self.text2.read().unwrap());
        }
        let mut x = drop_cell_count.write().unwrap();
        *x = *x + 1;
        drop(x);
        if is_debug_enabled() {
            println!("DROP CELL COUNT: {}", get_drop_cell_count());
            println!("--- --- ---");
        }
    }
}

pub fn get_drop_cell_count() -> i32 {
    drop_cell_count.read().unwrap().clone()
}

pub fn reset_drop_cell_count() {
    let mut x = drop_cell_count.write().unwrap();
    *x = 0;
}

impl Drop for MiddleCellWrapper {
    fn drop(&mut self) {
        if is_debug_enabled() {
            println!("DROP MIDDLE CELL WRAPPER for CELL DATA with info");
            println!("- text 1: {}", self.inner_sc_info_storage.text1.read().unwrap());
            println!("- text 2: {}", self.inner_sc_info_storage.text2.read().unwrap());
            println!("--- --- ---");
        }

        // XXX TODO: EXPLAIN RATIONALE & HOW THIS WORKS

        // XXX QUICK RATIONALE NEEDS EXPANDING: SHOULD NOT PUT LINKS AT INNER-MOST MIDDLE WRAPPER LAYER IN ORDER TO AVOID (PREVENT) TRULY CIRCULAR REF CYCLES
        let maybe_next_middle_wrapper = self.next_middle_wrapper.read().unwrap();
        if maybe_next_middle_wrapper.is_none() {
            return;
        }

        let maybe_inner_sc_linkage = self.inner_sc_linkage_info_strong_ref.read().unwrap().clone();
        if maybe_inner_sc_linkage.is_none() {
            return;
        }

        let next_middle_wrapper_ref = maybe_next_middle_wrapper.as_ref().unwrap().clone();

        if next_middle_wrapper_ref.next_middle_wrapper.read().unwrap().is_none() {
            return;
        }

        // NOTE: THIS CODE REQUIRES QUICK & UGLY WORKAROUND IN CREATE CELL API FN CODE FURTHER BELOW - XXX TODO NEED TO EXPLAIN THIS
        // XXX TODO LOOK FOR A WAY TO IMPROVE THIS

        let inner_sc_linkage_ref = RcRef::new(InnerSCLinkageInfo {
            // XXX TODO UTILITY FN
            link1: maybe_inner_sc_linkage
                .clone()
                .unwrap()
                .link1
                .clone()
                .unwrap()
                .inner_sc_info_storage
                .inner_middle_cell_wrapper_ref
                .read()
                .unwrap()
                .upgrade(),
            link2: maybe_inner_sc_linkage
                .clone()
                .unwrap()
                .link2
                .clone()
                .unwrap()
                .inner_sc_info_storage
                .inner_middle_cell_wrapper_ref
                .read()
                .unwrap()
                .upgrade(),
        });

        *next_middle_wrapper_ref.inner_sc_linkage_info_strong_ref.write().unwrap() = Some(inner_sc_linkage_ref.clone());

        *self.inner_sc_info_storage.sc_linkage_info_weak_ref.write().unwrap() = RcRef::downgrade(&inner_sc_linkage_ref);
    }
}

impl OuterCellWrapper {
    fn create_with_cell_data(text1: &str, text2: &str, link1: Option<MiddleCellWrapperRcRef>, link2: Option<MiddleCellWrapperRcRef>) -> OuterCellWrapperRcRef {
        let middle_cell_wrapper_ref = MiddleCellWrapper::create_with_inner_cell_data(text1, text2, link1, link2);
        let outer_wrapper_ref = RcRef::new(OuterCellWrapper {
            middle_cell_wrapper: RwValue::new(middle_cell_wrapper_ref.clone()),
            inner_sc_info_storage_ref: middle_cell_wrapper_ref.inner_sc_info_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_cell_wrapper_writer = middle_cell_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *middle_cell_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn create_with_middle_wrapper_ref(middle_cell_wrapper_ref: MiddleCellWrapperRcRef) -> OuterCellWrapperRcRef {
        let outer_wrapper_ref = RcRef::new(OuterCellWrapper {
            middle_cell_wrapper: RwValue::new(middle_cell_wrapper_ref.clone()),
            inner_sc_info_storage_ref: middle_cell_wrapper_ref.inner_sc_info_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_cell_wrapper_writer = middle_cell_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *middle_cell_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn update_sc_linkage(outer_cell_wrapper_ref: OuterCellWrapperRcRef, link1: Option<MiddleCellWrapperRcRef>, link2: Option<MiddleCellWrapperRcRef>) {
        // XXX TODO BOTH THIS CODE & THESE COMMENTS ARE WAY TO MUCH - NEED TO COMPLETELY REWRITE BOTH THIS CODE HERE & THESE COMMENTS
        // NOTE: This should create a new middle lifetime wrapper, which contains linkage for both peer links.
        // In case there is both inner middle lifetime wrapper & one more middle lifetime wrapper,
        // as needed for drop middle lifetime wrapper to work properly,
        // then DO NOT KEEP REFERENCE TO previous outer-middle lifetime wrapper,
        // in order to avoid (prevent) build-up of more & more middle lifetime wrappers
        // as the peer linkage may be updated over time.
        // OTHERWISE, NEED TO KEEP REFERENCE to previous outer-most middle lifetime wrapper.
        // XXX XXX TODO NEED BETTER EXPLANATION HERE
        // XXX TODO IMPROVE CODE QUALITY HERE - NEEDS TO BE MORE CLEAR & LESS REPETITIVE
        // XXX TODO REMOVE DUPLICATED CODE HERE
        // XXX TODO do match instead here - should be able to help avoid / remove some duplicated code here
        let next_middle_cell_wrapper_ref = if outer_cell_wrapper_ref
            .middle_cell_wrapper
            .read()
            .unwrap()
            .next_middle_wrapper
            .read()
            .unwrap()
            .is_none()
            || outer_cell_wrapper_ref
                .middle_cell_wrapper
                .read()
                .unwrap()
                .next_middle_wrapper
                .read()
                .unwrap()
                .clone()
                .unwrap()
                .next_middle_wrapper
                .read()
                .unwrap()
                .is_none()
        {
            outer_cell_wrapper_ref.middle_cell_wrapper.read().unwrap().clone()
        } else {
            outer_cell_wrapper_ref
                .middle_cell_wrapper
                .read()
                .unwrap()
                .next_middle_wrapper
                .read()
                .unwrap()
                .clone()
                .unwrap()
        };

        // XXX NEW OUTER-MOST MIDDLE LIFETIME WRAPPER - XXX TODO RENAME THIS
        let middle_cell_wrapper_ref = MiddleCellWrapper::create_with_next_middle_cell_wrapper_data(next_middle_cell_wrapper_ref.clone(), link1, link2);

        // XXX TODO KEEP IN SINGLE STATEMENT LIKE THIS:
        // *next_middle_cell_wrapper_ref.outer_wrapper_ref.write().unwrap() = RcRef::downgrade(&outer_cell_wrapper_ref);
        let mut next_middle_cell_wrapper_writer = next_middle_cell_wrapper_ref.outer_wrapper_ref.write().unwrap();
        *next_middle_cell_wrapper_writer = RcRef::downgrade(&outer_cell_wrapper_ref);

        // XXX TODO KEEP IN SINGLE STATEMENT LIKE THIS:
        // *outer_cell_wrapper_ref.middle_cell_wrapper.write().unwrap() = middle_cell_wrapper_ref;
        let mut middle_cell_wrapper_writer = outer_cell_wrapper_ref.middle_cell_wrapper.write().unwrap();
        *middle_cell_wrapper_writer = middle_cell_wrapper_ref.clone();
    }

    fn ref_middle_cell_wrapper_ref(middle_cell_wrapper_ref: MiddleCellWrapperRcRef) -> OuterCellWrapperRcRef {
        // XXX TODO USE MATCH INSTEAD HERE
        let mut my_outer_wrapper_ref = middle_cell_wrapper_ref.outer_wrapper_ref.read().unwrap().upgrade();
        if my_outer_wrapper_ref.is_none() {
            OuterCellWrapper::create_with_middle_wrapper_ref(middle_cell_wrapper_ref)
        } else {
            my_outer_wrapper_ref.unwrap()
        }
    }
}

impl MiddleCellWrapper {
    fn create_with_inner_cell_data(
        text1: &str,
        text2: &str,
        link1: Option<MiddleCellWrapperRcRef>,
        link2: Option<MiddleCellWrapperRcRef>,
    ) -> MiddleCellWrapperRcRef {
        let mut inner_sc_info_storage = InnerSCInfoStorage::create_with_inner_text_fields(text1, text2);

        let cell_linkage_strong_ref = RwValue::new(Some(InnerSCLinkageInfo::create_with_middle_cw_links(link1.clone(), link2.clone())));

        // KEEP XXX XXX INFO IN SYNC HERE
        let inner_sc_info_storage_ref = inner_sc_info_storage.clone();
        let mut cell_linkage_weak_writer = inner_sc_info_storage_ref.sc_linkage_info_weak_ref.write().unwrap();
        *cell_linkage_weak_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_cw_ref = RcRef::new(MiddleCellWrapper {
            inner_sc_info_storage: inner_sc_info_storage.clone(),
            inner_sc_linkage_info_strong_ref: cell_linkage_strong_ref,
            outer_wrapper_ref: RwValue::new(WeakRef::new()),
            next_middle_wrapper: RwValue::new(None),
        });

        *inner_sc_info_storage.linkage_strong_ref_wrapper.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());
        *inner_sc_info_storage.inner_middle_cell_wrapper_ref.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());

        return middle_cw_ref;
    }

    fn create_with_next_middle_cell_wrapper_data(
        next_middle_wrapper: MiddleCellWrapperRcRef,
        link1: Option<MiddleCellWrapperRcRef>,
        link2: Option<MiddleCellWrapperRcRef>,
    ) -> MiddleCellWrapperRcRef {
        let inner_sc_info_storage = next_middle_wrapper.clone().inner_sc_info_storage.clone();

        let cell_linkage_strong_ref = RwValue::new(Some(InnerSCLinkageInfo::create_with_middle_cw_links(link1.clone(), link2.clone())));

        // XXX TODO RECONSIDER EXTRA REF CLONE HERE
        let inner_sc_info_storage_ref = inner_sc_info_storage.clone();

        // KEEP XXX XXX INFO IN SYNC HERE
        let mut cell_linkage_weak_ref_writer = inner_sc_info_storage_ref.sc_linkage_info_weak_ref.write().unwrap();
        *cell_linkage_weak_ref_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_wrapper_ref = RcRef::new(MiddleCellWrapper {
            inner_sc_info_storage: inner_sc_info_storage.clone(),
            inner_sc_linkage_info_strong_ref: cell_linkage_strong_ref,
            outer_wrapper_ref: RwValue::new(next_middle_wrapper.clone().outer_wrapper_ref.read().unwrap().clone()),
            next_middle_wrapper: RwValue::new(Some(next_middle_wrapper.clone())),
        });

        let old_linkage_strong_ref_wrapper = inner_sc_info_storage_ref.linkage_strong_ref_wrapper.read().unwrap().upgrade().clone();
        *inner_sc_info_storage_ref.linkage_strong_ref_wrapper.write().unwrap() = RcRef::downgrade(&middle_wrapper_ref.clone());

        // clear out XXX XXX
        if old_linkage_strong_ref_wrapper.is_some() {
            *old_linkage_strong_ref_wrapper.unwrap().inner_sc_linkage_info_strong_ref.write().unwrap() = None;
        }

        middle_wrapper_ref
    }

    fn get_text1(&self) -> String {
        self.inner_sc_info_storage.text1.read().unwrap().clone()
    }

    fn get_text2(&self) -> String {
        self.inner_sc_info_storage.text2.read().unwrap().clone()
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    fn update_cell_text_data(&self, text1: &str, text2: &str) {
        let mut xxx1 = self.inner_sc_info_storage.text1.write().unwrap();
        *xxx1 = String::from(text1);
        let mut xxx2 = self.inner_sc_info_storage.text2.write().unwrap();
        *xxx2 = String::from(text2);
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    fn clear_inner_sc_linkage_strong_ref(&self) {
        let mut inner_sc_linkage_strong_ref_writer = self.inner_sc_linkage_info_strong_ref.write().unwrap();
        *inner_sc_linkage_strong_ref_writer = None;
    }
}

impl InnerSCLinkageInfo {
    // XXX TBD SUPPORT CREATE API FN WITH EMPTY LINKS ???
    fn create_with_middle_cw_links(link1: Option<MiddleCellWrapperRcRef>, link2: Option<MiddleCellWrapperRcRef>) -> RcRef<InnerSCLinkageInfo> {
        RcRef::new(InnerSCLinkageInfo { link1, link2 })
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_link1(&self) -> Option<MiddleCellWrapperRcRef> {
        self.link1.clone()
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_link2(&self) -> Option<MiddleCellWrapperRcRef> {
        self.link2.clone()
    }
}

impl InnerSCInfoStorage {
    fn create_with_inner_text_fields(text1: &str, text2: &str) -> InnerSCInfoStorageRcRef {
        RcRef::new(InnerSCInfoStorage {
            text1: RcRef::new(RwValue::new(String::from(text1))),
            text2: RcRef::new(RwValue::new(String::from(text2))),
            outer_wrapper_ref: RwValue::new(WeakRef::new()),
            sc_linkage_info_weak_ref: RwValue::new(WeakRef::new()),
            linkage_strong_ref_wrapper: RwValue::new(WeakRef::new()),
            inner_middle_cell_wrapper_ref: RwValue::new(WeakRef::new()),
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

impl SCDataRecordRef {
    pub fn new(text1: &str, text2: &str, link1: Option<SCDataRecordRef>, link2: Option<SCDataRecordRef>) -> SCDataRecordRef {
        create_cell_with_links(text1, text2, link1, link2)
    }

    pub fn get_text1(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.0.inner_sc_info_storage_ref.get_text1()
    }

    pub fn get_text2(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.0.inner_sc_info_storage_ref.get_text2()
    }

    pub fn get_link1(&self) -> Option<SCDataRecordRef> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE (IF POSSIBLE WITHOUT SIGNIFICANT IMPACT ON ANY BENCHMARKS)
        let sc_linkage_info_ref = self
            .0 // StrongSCDataRecordManagerRcRef aka OuterCellWrapperRcRef
            .inner_sc_info_storage_ref
            .sc_linkage_info_weak_ref
            .read()
            .unwrap()
            .upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_cell_wrapper_ref = sc_linkage_info_ref.unwrap().link1.clone();
        match maybe_linked_middle_cell_wrapper_ref {
            None => None,
            Some(middle_cell_wrapper_ref) => Some(SCDataRecordRef::from_outer_cell_wrapper(OuterCellWrapper::ref_middle_cell_wrapper_ref(
                middle_cell_wrapper_ref,
            ))),
        }
    }

    pub fn get_link2(&self) -> Option<SCDataRecordRef> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE (IF POSSIBLE WITHOUT SIGNIFICANT IMPACT ON ANY BENCHMARKS)
        let sc_linkage_info_ref = self
            .0 // StrongSCDataRecordManagerRcRef aka OuterCellWrapperRcRef
            .inner_sc_info_storage_ref
            .sc_linkage_info_weak_ref
            .read()
            .unwrap()
            .upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_cell_wrapper_ref = sc_linkage_info_ref.unwrap().link2.clone();
        match maybe_linked_middle_cell_wrapper_ref {
            None => None,
            Some(middle_cell_wrapper_ref) => Some(SCDataRecordRef::from_outer_cell_wrapper(OuterCellWrapper::ref_middle_cell_wrapper_ref(
                middle_cell_wrapper_ref,
            ))),
        }
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    pub fn update_data(&self, text1: &str, text2: &str, link1: Option<SCDataRecordRef>, link2: Option<SCDataRecordRef>) {
        let my_middle_cell_wrapper_ref = self.get_middle_cell_wrapper();

        my_middle_cell_wrapper_ref.update_cell_text_data(text1, text2);

        let my_outer_wrapper_ref = self.0.clone();
        // NOTE: This should update the SC linkage so that it links to the outer-most middle lifetime wrapper for both linked SC peers.
        OuterCellWrapper::update_sc_linkage(
            my_outer_wrapper_ref,
            match link1 {
                Some(r) => Some(r.get_middle_cell_wrapper()),
                None => None,
            },
            match link2 {
                Some(r) => Some(r.get_middle_cell_wrapper()),
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

    fn get_middle_cell_wrapper(&self) -> MiddleCellWrapperRcRef {
        self.0.middle_cell_wrapper.read().unwrap().clone()
    }

    fn from_outer_cell_wrapper(outer_wrapper_ref: OuterCellWrapperRcRef) -> SCDataRecordRef {
        SCDataRecordRef(outer_wrapper_ref)
    }
}

// XXX TODO MOVE THIS (???)
fn create_cell_with_links(text1: &str, text2: &str, link1: Option<SCDataRecordRef>, link2: Option<SCDataRecordRef>) -> SCDataRecordRef {
    // XXX TODO ADD EXPLANATION FOR THIS:
    let cw = OuterCellWrapper::create_with_cell_data(text1, text2, None, None);

    // XXX QUICK & UGLY WORKAROUND FOR XXX XXX IN MIDDLE CELL LIFETIME WRAPPER DROP FUNCTION ABOVE
    // XXX QUICK RATIONALE NEEDS EXPANDING: SHOULD NOT KEEP LINKS AT INNER-MOST MIDDLE WRAPPER LAYER IN ORDER TO AVOID (PREVENT) TRULY CIRCULAR REF CYCLES
    let x = SCDataRecordRef::from_outer_cell_wrapper(cw);
    // XXX TODO IMPROVE NOTE: THIS CREATES ANOTHER MIDDLE LIFETIME WRAPPER WITH STRONG REFERENCE TO THE PEER LINKS
    x.update_data(text1, text2, link1, link2);
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
