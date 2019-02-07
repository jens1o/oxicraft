use crate::coding::varint::Varint;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

static ENTITY_COUNTER: AtomicUsize = AtomicUsize::new(1);
static TELEPORT_COUNTER: AtomicI32 = AtomicI32::new(1);

/// Returns a new Entity-ID that is unique on the server.
#[inline]
pub fn get_new_eid() -> usize {
    ENTITY_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Returns a new Teleport-ID that is unique on the server.
#[inline]
pub fn get_new_teleport_id() -> Varint {
    Varint(TELEPORT_COUNTER.fetch_add(1, Ordering::SeqCst))
}
