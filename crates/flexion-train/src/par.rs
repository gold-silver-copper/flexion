//! Work shared out over threads: one map, on at most [`THREADS`] threads.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Threads the learner uses at most.
pub const THREADS: usize = 8;

/// `f` of every item, in the items' order, on at most [`THREADS`] threads,
/// each taking the next item when it finishes one (so a run of slow items
/// doesn't hold back one thread's share). Call it at the top of a stage,
/// never inside another map: the threads would multiply. A thread that
/// panics panics the caller, with the same payload: a lost item is an
/// error, never a shifted result.
pub fn map<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let threads = THREADS.min(items.len()).max(1);
    let (f, next) = (&f, &next);
    let done: Vec<Vec<(usize, R)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(move || {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(i) else {
                            return out;
                        };
                        out.push((i, f(item)));
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_else(|e| std::panic::resume_unwind(e)))
            .collect()
    });
    let mut all: Vec<(usize, R)> = done.into_iter().flatten().collect();
    all.sort_unstable_by_key(|(i, _)| *i);
    all.into_iter().map(|(_, r)| r).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_item_comes_back_in_its_place() {
        // Items of very different cost, so threads finish out of order.
        let items: Vec<u64> = (0..1000).collect();
        let out = map(&items, |&i| {
            let mut x = i;
            for _ in 0..(i % 7) * 1000 {
                x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            }
            (i, x)
        });
        assert_eq!(out.len(), items.len());
        assert!(out.iter().enumerate().all(|(at, &(i, _))| at as u64 == i));
    }
}
