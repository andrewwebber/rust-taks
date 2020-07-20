use futures::{future::FutureExt, pin_mut, select};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

async fn add(counter: Arc<AtomicU16>, incr: u16) {
    counter.fetch_add(incr, Ordering::SeqCst);
    time::delay_for(Duration::from_millis(1000u64 * incr as u64)).await;
    println!("add {}", incr);
}

async fn async_main(counter: Arc<AtomicU16>) {
    let f1 = add(counter.clone(), 1).fuse();
    let f2 = add(counter.clone(), 3).fuse();
    let f3 = add(counter.clone(), 2).fuse();
    // join!(f1, f2, f3);
    pin_mut!(f1, f2, f3);
    select! {
        () = f1 => println!("f1"),
        () = f2 => println!("f2"),
        () = f3 => println!("f3"),
    }
}

#[tokio::main]
async fn main() {
    let counter = AtomicU16::new(1);
    let ar = Arc::new(counter);
    async_main(ar.clone()).await;
    println!("value {}", ar.load(Ordering::SeqCst));
}
