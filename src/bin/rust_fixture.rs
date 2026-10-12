use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Counting;
static CALLS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(l) };
        if !p.is_null() {
            CALLS.fetch_add(1, Ordering::SeqCst);
            BYTES.fetch_add(l.size(), Ordering::SeqCst);
            let live = LIVE.fetch_add(l.size(), Ordering::SeqCst) + l.size();
            PEAK.fetch_max(live, Ordering::SeqCst);
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Ordering::SeqCst);
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static ALLOC: Counting = Counting;
fn main() {
    let mb: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if !(1..=64).contains(&mb) {
        std::process::exit(2)
    }
    let mut data = vec![0u8; mb * 1024 * 1024];
    for x in data.iter_mut() {
        *x = 1
    }
    std::hint::black_box(&data);
    let checksum = data[0] as u64 + data[data.len() - 1] as u64;
    let calls = CALLS.load(Ordering::SeqCst);
    let bytes = BYTES.load(Ordering::SeqCst);
    let peak = PEAK.load(Ordering::SeqCst);
    println!(
        "{{\"allocation_calls\":{calls},\"allocated_bytes\":{bytes},\"peak_live_bytes\":{peak},\"checksum\":{checksum}}}"
    );
}
