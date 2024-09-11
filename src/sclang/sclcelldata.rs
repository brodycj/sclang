use std::fmt::Write;
use std::sync::{Arc, RwLock, Weak};

#[derive(Clone)]
pub struct SCLRef(PersistentSCManagerRcRef);

type PersistentSCManagerRcRef = RcRef<PersistentSCLifetimeManager>;

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

// XXX TBD ??? ???:
// XXX TODO IMPROVE NAMING CONSISTENCY HERE

// XXX TODO EXPLAIN RATIONALE FOR THESE LIFETIME MANAGERS
// XXX QUICK EXPLANATION:
// PersistentSCLifetimeManagerRef (as referenced by: PersistentSCLifetimeManagerRef) IS USED BY SCL REF TO KEEP STRONG REFERENCE & HELP MAINTAIN LIFETIME
// LinkedSCLifetimeManager - NESTED LIFETIME MANAGER(S) - TO HELP MAINTAIN PROPER SCL DATA LIFETIME WITHOUT ANY STRONG REFERENCE CYCLES

// XXX ??? ??? ???:
// XXX TODO NEED TO RECONSIDER BOTH NAMING AND HOW MUCH DESCRIPTIVE TEXT TO KEEP OR UPDATE;
// HOPEFULLY BETTER NAMING CAN REDUCE THE NEED FOR SOME OF THE DESCRIPTIVE TEXT HERE

// XXX TBD SEVERAL PIECES OF STATE ARE MAINTAINED IN MULTIPLE PLACES - RECONSIDER WHERE TO KEEP THESE PIECES OF STATE

// XXX TBD ADD DESCRIPTION (???)
// NOTE: SCLRef wraps a reference to this outer / top-level lifetime manager
// for SCL data cell objects which are *indirectly* referenced by SCLRef objects.
// While there may be multiple SCLRef objects referencing the same SCL record,
// there should only be a single PersistentSCLifetimeManager corresponding to a given SCL record.
struct PersistentSCLifetimeManager {
    // This field references the outer-most linked SC data lifetime manager
    // which is expected to keep the strong reference to the SC linkage manager
    // (until it is superceded by a newer outer-most linked SC data lifetime manager).
    outer_linked_sc_manager: RwValue<LinkedSCManagerRcRef>,
    // This field references (strong reference) where the data (text data) fields are stored,
    // which in turn does keep a weak reference to the peer linkage
    inner_sc_info_manager: SCInfoManagerRcRef,
}

type LinkedSCManagerRcRef = RcRef<LinkedSCLifetimeManager>;

// XXX TBD IMPROVE NAMING & ADD DESCRIPTION FOR THIS LIFETIME MANAGER
// NOTE: This is an SCL object / data lifetime manager that helps keep data objects alive exactly as long as they are
// directly or indirectly reachable from the outside via using SCLRef objects.
// Keeping multiple levels of lifetime manager wrappers helps avoid strong circular references & allow
// unreadable SCL data cell objects to be automatically dropped & cleaned up once they are no longer reachable from the outside.
// XXX TODO NEED GOOD EXPLANATION OF THE STRATEGY FOR THIS !!!
struct LinkedSCLifetimeManager {
    // XXX TODO EXPLAIN AND/OR CLARIFY RATIONALE FOR THIS STRONG REF OPTION
    // QUICK RATIONALE: outer-most linked SC lifetime manager  is expected to keep the strong reference to the SC linkage manager
    // (until it is superceded by a newer outer-most linked SC lifetime manager)
    sc_linkage_manager_strong_ref: RwValue<Option<RcRef<SCLinkageManager>>>,
    inner_sc_info_storage: SCInfoManagerRcRef,
    persistent_sc_lifetime_manager: RwWeakRef<PersistentSCLifetimeManager>,
    // XXX TODO EXPLAIN HOW THIS WORKS
    // XXX TBD SHORTER NAMING - ???
    next_inner_sc_lifetime_manager: RwValue<Option<LinkedSCManagerRcRef>>,
}

// XXX TBD RECONSIDER NAMING ??? ???
struct SCLinkageManager {
    link1: Option<LinkedSCManagerRcRef>,
    link2: Option<LinkedSCManagerRcRef>,
}

type SCInfoManagerRcRef = RcRef<SCInfoManager>;

struct SCInfoManager {
    text1: RcRef<RwValue<String>>,
    text2: RcRef<RwValue<String>>,
    persistent_sc_lifetime_manager_ref: RwWeakRef<PersistentSCLifetimeManager>,
    // XXX TBD NAMING OF THIS ??? ???
    sc_linkage_manager_weak_ref: RwWeakRef<SCLinkageManager>,
    // XXX TBD RECONSIDER NAMING - outer-most linked SC lifetime manager which keeps strong reference to the SC linkage manager
    outer_linked_sc_manager_weak_ref: RwWeakRef<LinkedSCLifetimeManager>,
    // XXX TBD / TODO clarify motivation for this:
    inner_persistent_sc_manager: RwWeakRef<LinkedSCLifetimeManager>,
}

// XXX TODO outdated variable names & naming in comments below - XXX TODO NEED TO FIX THESE

static drop_cell_count: RwValue<i32> = RwValue::new(0);

impl Drop for SCInfoManager {
    fn drop(&mut self) {
        if is_debug_enabled() {
            // XXX TODO UPDATE THIS INFO TEXT
            println!("DROP CELL DATA with info:");
            println!("- text 1: {}", self.text1.read().unwrap());
            println!("- text 2: {}", self.text2.read().unwrap());
        }
        let mut x = drop_cell_count.write().unwrap();
        *x = *x + 1;
        drop(x);
        if is_debug_enabled() {
            // XXX TODO UPDATE THIS INFO TEXT
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

impl Drop for LinkedSCLifetimeManager {
    fn drop(&mut self) {
        if is_debug_enabled() {
            // XXX TODO UPDATE THIS INFO TEXT
            println!("DROP MIDDLE CELL WRAPPER for CELL DATA with info");
            println!("- text 1: {}", self.inner_sc_info_storage.text1.read().unwrap());
            println!("- text 2: {}", self.inner_sc_info_storage.text2.read().unwrap());
            println!("--- --- ---");
        }

        // XXX TODO: EXPLAIN RATIONALE & HOW THIS WORKS

        // XXX QUICK RATIONALE NEEDS EXPANDING: SHOULD NOT PUT LINKS AT INNER-MOST MIDDLE WRAPPER LAYER IN ORDER TO AVOID (PREVENT) TRULY CIRCULAR REF CYCLES
        let maybe_next_middle_wrapper = self.next_inner_sc_lifetime_manager.read().unwrap();
        if maybe_next_middle_wrapper.is_none() {
            return;
        }

        let maybe_inner_sc_linkage = self.sc_linkage_manager_strong_ref.read().unwrap().clone();
        if maybe_inner_sc_linkage.is_none() {
            return;
        }

        let next_middle_wrapper_ref = maybe_next_middle_wrapper.as_ref().unwrap().clone();

        if next_middle_wrapper_ref.next_inner_sc_lifetime_manager.read().unwrap().is_none() {
            return;
        }

        // NOTE: THIS CODE REQUIRES QUICK & UGLY WORKAROUND IN CREATE CELL API FN CODE FURTHER BELOW - XXX TODO NEED TO EXPLAIN THIS
        // XXX TODO LOOK FOR A WAY TO IMPROVE THIS

        let inner_sc_linkage_ref = RcRef::new(SCLinkageManager {
            // XXX TODO UTILITY FN
            link1: maybe_inner_sc_linkage
                .clone()
                .unwrap()
                .link1
                .clone()
                .unwrap()
                .inner_sc_info_storage
                .inner_persistent_sc_manager
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
                .inner_persistent_sc_manager
                .read()
                .unwrap()
                .upgrade(),
        });

        *next_middle_wrapper_ref.sc_linkage_manager_strong_ref.write().unwrap() = Some(inner_sc_linkage_ref.clone());

        *self.inner_sc_info_storage.sc_linkage_manager_weak_ref.write().unwrap() = RcRef::downgrade(&inner_sc_linkage_ref);
    }
}

impl PersistentSCLifetimeManager {
    fn create_with_cell_data(text1: &str, text2: &str, link1: Option<LinkedSCManagerRcRef>, link2: Option<LinkedSCManagerRcRef>) -> PersistentSCManagerRcRef {
        let middle_cell_wrapper_ref = LinkedSCLifetimeManager::create_with_inner_cell_data(text1, text2, link1, link2);
        let outer_wrapper_ref = RcRef::new(PersistentSCLifetimeManager {
            outer_linked_sc_manager: RwValue::new(middle_cell_wrapper_ref.clone()),
            inner_sc_info_manager: middle_cell_wrapper_ref.inner_sc_info_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_cell_wrapper_writer = middle_cell_wrapper_ref.persistent_sc_lifetime_manager.write().unwrap();
        *middle_cell_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn create_with_middle_wrapper_ref(middle_cell_wrapper_ref: LinkedSCManagerRcRef) -> PersistentSCManagerRcRef {
        let outer_wrapper_ref = RcRef::new(PersistentSCLifetimeManager {
            outer_linked_sc_manager: RwValue::new(middle_cell_wrapper_ref.clone()),
            inner_sc_info_manager: middle_cell_wrapper_ref.inner_sc_info_storage.clone(),
        });
        // XXX TODO UTIL FN - REPEATED CODE
        let mut middle_cell_wrapper_writer = middle_cell_wrapper_ref.persistent_sc_lifetime_manager.write().unwrap();
        *middle_cell_wrapper_writer = RcRef::downgrade(&outer_wrapper_ref);
        outer_wrapper_ref
    }

    fn update_sc_linkage(outer_cell_wrapper_ref: PersistentSCManagerRcRef, link1: Option<LinkedSCManagerRcRef>, link2: Option<LinkedSCManagerRcRef>) {
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
            .outer_linked_sc_manager
            .read()
            .unwrap()
            .next_inner_sc_lifetime_manager
            .read()
            .unwrap()
            .is_none()
            || outer_cell_wrapper_ref
                .outer_linked_sc_manager
                .read()
                .unwrap()
                .next_inner_sc_lifetime_manager
                .read()
                .unwrap()
                .clone()
                .unwrap()
                .next_inner_sc_lifetime_manager
                .read()
                .unwrap()
                .is_none()
        {
            outer_cell_wrapper_ref.outer_linked_sc_manager.read().unwrap().clone()
        } else {
            outer_cell_wrapper_ref
                .outer_linked_sc_manager
                .read()
                .unwrap()
                .next_inner_sc_lifetime_manager
                .read()
                .unwrap()
                .clone()
                .unwrap()
        };

        // XXX NEW OUTER-MOST MIDDLE LIFETIME WRAPPER - XXX TODO RENAME THIS
        let middle_cell_wrapper_ref = LinkedSCLifetimeManager::create_with_next_middle_cell_wrapper_data(next_middle_cell_wrapper_ref.clone(), link1, link2);

        // XXX TODO KEEP IN SINGLE STATEMENT LIKE THIS:
        // *next_middle_cell_wrapper_ref.outer_wrapper_ref.write().unwrap() = RcRef::downgrade(&outer_cell_wrapper_ref);
        let mut next_middle_cell_wrapper_writer = next_middle_cell_wrapper_ref.persistent_sc_lifetime_manager.write().unwrap();
        *next_middle_cell_wrapper_writer = RcRef::downgrade(&outer_cell_wrapper_ref);

        // XXX TODO KEEP IN SINGLE STATEMENT LIKE THIS:
        // *outer_cell_wrapper_ref.middle_cell_wrapper.write().unwrap() = middle_cell_wrapper_ref;
        let mut middle_cell_wrapper_writer = outer_cell_wrapper_ref.outer_linked_sc_manager.write().unwrap();
        *middle_cell_wrapper_writer = middle_cell_wrapper_ref.clone();
    }

    fn ref_middle_cell_wrapper_ref(middle_cell_wrapper_ref: LinkedSCManagerRcRef) -> PersistentSCManagerRcRef {
        // XXX TODO USE MATCH INSTEAD HERE
        let mut my_outer_wrapper_ref = middle_cell_wrapper_ref.persistent_sc_lifetime_manager.read().unwrap().upgrade();
        if my_outer_wrapper_ref.is_none() {
            PersistentSCLifetimeManager::create_with_middle_wrapper_ref(middle_cell_wrapper_ref)
        } else {
            my_outer_wrapper_ref.unwrap()
        }
    }
}

impl LinkedSCLifetimeManager {
    fn create_with_inner_cell_data(text1: &str, text2: &str, link1: Option<LinkedSCManagerRcRef>, link2: Option<LinkedSCManagerRcRef>) -> LinkedSCManagerRcRef {
        let mut inner_sc_info_storage = SCInfoManager::create_with_inner_text_fields(text1, text2);

        let cell_linkage_strong_ref = RwValue::new(Some(SCLinkageManager::create_with_middle_cw_links(link1.clone(), link2.clone())));

        // KEEP XXX XXX INFO IN SYNC HERE
        let inner_sc_info_storage_ref = inner_sc_info_storage.clone();
        let mut cell_linkage_weak_writer = inner_sc_info_storage_ref.sc_linkage_manager_weak_ref.write().unwrap();
        *cell_linkage_weak_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_cw_ref = RcRef::new(LinkedSCLifetimeManager {
            inner_sc_info_storage: inner_sc_info_storage.clone(),
            sc_linkage_manager_strong_ref: cell_linkage_strong_ref,
            persistent_sc_lifetime_manager: RwValue::new(WeakRef::new()),
            next_inner_sc_lifetime_manager: RwValue::new(None),
        });

        *inner_sc_info_storage.outer_linked_sc_manager_weak_ref.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());
        *inner_sc_info_storage.inner_persistent_sc_manager.write().unwrap() = RcRef::downgrade(&middle_cw_ref.clone());

        return middle_cw_ref;
    }

    fn create_with_next_middle_cell_wrapper_data(
        next_middle_wrapper: LinkedSCManagerRcRef,
        link1: Option<LinkedSCManagerRcRef>,
        link2: Option<LinkedSCManagerRcRef>,
    ) -> LinkedSCManagerRcRef {
        let inner_sc_info_storage = next_middle_wrapper.clone().inner_sc_info_storage.clone();

        let cell_linkage_strong_ref = RwValue::new(Some(SCLinkageManager::create_with_middle_cw_links(link1.clone(), link2.clone())));

        // XXX TODO RECONSIDER EXTRA REF CLONE HERE
        let inner_sc_info_storage_ref = inner_sc_info_storage.clone();

        // KEEP XXX XXX INFO IN SYNC HERE
        let mut cell_linkage_weak_ref_writer = inner_sc_info_storage_ref.sc_linkage_manager_weak_ref.write().unwrap();
        *cell_linkage_weak_ref_writer = RcRef::downgrade(&cell_linkage_strong_ref.read().unwrap().clone().unwrap());

        let middle_wrapper_ref = RcRef::new(LinkedSCLifetimeManager {
            inner_sc_info_storage: inner_sc_info_storage.clone(),
            sc_linkage_manager_strong_ref: cell_linkage_strong_ref,
            persistent_sc_lifetime_manager: RwValue::new(next_middle_wrapper.clone().persistent_sc_lifetime_manager.read().unwrap().clone()),
            next_inner_sc_lifetime_manager: RwValue::new(Some(next_middle_wrapper.clone())),
        });

        let old_linkage_strong_ref_wrapper = inner_sc_info_storage_ref.outer_linked_sc_manager_weak_ref.read().unwrap().upgrade().clone();
        *inner_sc_info_storage_ref.outer_linked_sc_manager_weak_ref.write().unwrap() = RcRef::downgrade(&middle_wrapper_ref.clone());

        // clear out XXX XXX
        if old_linkage_strong_ref_wrapper.is_some() {
            *old_linkage_strong_ref_wrapper.unwrap().sc_linkage_manager_strong_ref.write().unwrap() = None;
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
        let mut inner_sc_linkage_strong_ref_writer = self.sc_linkage_manager_strong_ref.write().unwrap();
        *inner_sc_linkage_strong_ref_writer = None;
    }
}

impl SCLinkageManager {
    // XXX TBD SUPPORT CREATE API FN WITH EMPTY LINKS ???
    fn create_with_middle_cw_links(link1: Option<LinkedSCManagerRcRef>, link2: Option<LinkedSCManagerRcRef>) -> RcRef<SCLinkageManager> {
        RcRef::new(SCLinkageManager { link1, link2 })
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_link1(&self) -> Option<LinkedSCManagerRcRef> {
        self.link1.clone()
    }

    // XXX TBD API - KEEP THIS XXX ??? ???
    fn get_link2(&self) -> Option<LinkedSCManagerRcRef> {
        self.link2.clone()
    }
}

impl SCInfoManager {
    fn create_with_inner_text_fields(text1: &str, text2: &str) -> SCInfoManagerRcRef {
        RcRef::new(SCInfoManager {
            text1: RcRef::new(RwValue::new(String::from(text1))),
            text2: RcRef::new(RwValue::new(String::from(text2))),
            persistent_sc_lifetime_manager_ref: RwValue::new(WeakRef::new()),
            sc_linkage_manager_weak_ref: RwValue::new(WeakRef::new()),
            outer_linked_sc_manager_weak_ref: RwValue::new(WeakRef::new()),
            inner_persistent_sc_manager: RwValue::new(WeakRef::new()),
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

impl SCLRef {
    pub fn get_text1(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.0.inner_sc_info_manager.get_text1()
    }

    pub fn get_text2(&self) -> String {
        // XXX TBD ADD EASIER UTIL FN ???
        self.0.inner_sc_info_manager.get_text2()
    }

    pub fn get_link1(&self) -> Option<SCLRef> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE (IF POSSIBLE WITHOUT SIGNIFICANT IMPACT ON ANY BENCHMARKS)
        let sc_linkage_info_ref = self
            .0 // PersistentSCManagerRef aka OuterCellWrapperRcRef
            .inner_sc_info_manager
            .sc_linkage_manager_weak_ref
            .read()
            .unwrap()
            .upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_cell_wrapper_ref = sc_linkage_info_ref.unwrap().link1.clone();
        match maybe_linked_middle_cell_wrapper_ref {
            None => None,
            Some(middle_cell_wrapper_ref) => Some(SCLRef::from_outer_cell_wrapper(PersistentSCLifetimeManager::ref_middle_cell_wrapper_ref(
                middle_cell_wrapper_ref,
            ))),
        }
    }

    pub fn get_link2(&self) -> Option<SCLRef> {
        // XXX TODO ADD & USE HELPER FN FOR THIS MATCH HERE (IF POSSIBLE WITHOUT SIGNIFICANT IMPACT ON ANY BENCHMARKS)
        let sc_linkage_info_ref = self
            .0 // PersistentSCManagerRef aka OuterCellWrapperRcRef
            .inner_sc_info_manager
            .sc_linkage_manager_weak_ref
            .read()
            .unwrap()
            .upgrade();
        if sc_linkage_info_ref.is_none() {
            return None;
        };
        let maybe_linked_middle_cell_wrapper_ref = sc_linkage_info_ref.unwrap().link2.clone();
        match maybe_linked_middle_cell_wrapper_ref {
            None => None,
            Some(middle_cell_wrapper_ref) => Some(SCLRef::from_outer_cell_wrapper(PersistentSCLifetimeManager::ref_middle_cell_wrapper_ref(
                middle_cell_wrapper_ref,
            ))),
        }
    }

    // XXX TBD SHOULD THIS TAKE &mut self ???
    pub fn update_data(&self, text1: &str, text2: &str, link1: Option<SCLRef>, link2: Option<SCLRef>) {
        let my_middle_cell_wrapper_ref = self.get_middle_cell_wrapper();

        my_middle_cell_wrapper_ref.update_cell_text_data(text1, text2);

        let my_outer_wrapper_ref = self.0.clone();
        // NOTE: This should update the SC linkage so that it links to the outer-most middle lifetime wrapper for both linked SC peers.
        PersistentSCLifetimeManager::update_sc_linkage(
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

    fn get_middle_cell_wrapper(&self) -> LinkedSCManagerRcRef {
        self.0.outer_linked_sc_manager.read().unwrap().clone()
    }

    fn from_outer_cell_wrapper(outer_wrapper_ref: PersistentSCManagerRcRef) -> SCLRef {
        SCLRef(outer_wrapper_ref)
    }
}

pub fn create_cell_with_text_only(text1: &str, text2: &str) -> SCLRef {
    // XXX QUICK & UGLY WORKAROUND FOR XXX XXX IN MIDDLE CELL LIFETIME WRAPPER DROP FUNCTION ABOVE
    let x = SCLRef::from_outer_cell_wrapper(PersistentSCLifetimeManager::create_with_cell_data(text1, text2, None, None));
    // XXX TODO THIS UPDATE IS NOT NEEDED AS THERE ARE NO PEERS TO LINK TO AT THIS POINT
    x.update_data(text1, text2, None, None);
    x
}

pub fn create_cell_with_links(text1: &str, text2: &str, link1: SCLRef, link2: SCLRef) -> SCLRef {
    // XXX TODO USE UTIL FN HERE
    let cw = PersistentSCLifetimeManager::create_with_cell_data(
        text1,
        text2,
        // XXX TBD SHOULD BE NO NEED TO CREATE WITH THE LINKS AT THIS POINT
        // AS THE UPDATE BELOW WILL INCLUDE THE PEER LINKS NEEDED
        // (NEED TO CHECK IMPACT ON BENCHMARKS WHEN REMOVING THE LINKS FROM THIS PART)
        Some(link1.clone().0.outer_linked_sc_manager.read().unwrap().clone()),
        Some(link2.clone().0.outer_linked_sc_manager.read().unwrap().clone()),
    );

    // XXX QUICK & UGLY WORKAROUND FOR XXX XXX IN MIDDLE CELL LIFETIME WRAPPER DROP FUNCTION ABOVE
    // XXX QUICK RATIONALE NEEDS EXPANDING: SHOULD NOT KEEP LINKS AT INNER-MOST MIDDLE WRAPPER LAYER IN ORDER TO AVOID (PREVENT) TRULY CIRCULAR REF CYCLES
    let x = SCLRef::from_outer_cell_wrapper(cw);
    // XXX TODO IMPROVE NOTE: THIS CREATES ANOTHER MIDDLE LIFETIME WRAPPER WITH STRONG REFERENCE TO THE PEER LINKS
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
