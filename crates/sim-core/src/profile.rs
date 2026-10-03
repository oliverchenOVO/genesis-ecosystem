//! Optional wall-clock diagnostics, entirely outside State and RNG.
use std::{cell::RefCell, collections::BTreeMap, time::Instant};
thread_local! { static TIMINGS:RefCell<BTreeMap<&'static str,(u128,u64)>>=const { RefCell::new(BTreeMap::new()) }; }
pub fn mark(label: &'static str, stamp: &mut Instant) {
    let elapsed = stamp.elapsed().as_nanos();
    *stamp = Instant::now();
    TIMINGS.with(|t| {
        let mut t = t.borrow_mut();
        let entry = t.entry(label).or_default();
        entry.0 += elapsed;
        entry.1 += 1;
    });
}
pub fn report() -> Vec<(String, u128, u64)> {
    TIMINGS.with(|t| {
        t.borrow()
            .iter()
            .map(|(k, (ns, count))| (k.to_string(), *ns, *count))
            .collect()
    })
}
