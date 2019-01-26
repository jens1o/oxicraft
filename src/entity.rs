use std::sync::atomic::{AtomicUsize, Ordering};

static ENTITY_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Returns a new Entity-ID that is unique on the server.
#[inline]
pub fn get_new_eid() -> usize {
    ENTITY_COUNTER.fetch_add(1, Ordering::SeqCst)
}
