use futures::{executor::block_on, future::*, *};
use std::future::Future;

// The keyword “async” provides syntactic sugar
// An async fn returns a Future
// https://doc.rust-lang.org/std/future/trait.Future.html
// A value that may not have finished computing yet
// Makes it possible for a thread to continue doing useful work while it waits for the value
async fn increment(n: u64) -> u64 {
    n + 1
}

// This is equivalent to increment
fn increment_desugared(n: u64) -> impl Future<Output = u64> {
    ready(n + 1)
}

// Futures are lazy, they do nothing until driven into completion
async fn panic_buyer() {
    panic!("I need more toilet paper!");
}

// You need to .await the Future to make it happen
// The .await keyword can be used only in async functions or blocks
#[allow(dead_code)]
async fn activate_panic_buyer() {
    panic_buyer().await;
}

async fn try_plus(x: u8, y: u8) -> Result<u8, ()> {
    x.checked_add(y).ok_or(())
}

// Vanilla main() fn cannot be marked async
// Try to do so and see compiler error
// There are crates that provide solutions
fn main() {
    // You can use block_on as a workaround, it runs a Future to completion on the current thread
    let result = block_on(increment(4));
    let result2 = block_on(increment_desugared(4));
    assert_eq!(result, result2);

    // This won't panic
    let _result = panic_buyer();

    // Uncomment the line below to make it panic
    // let _result2 = block_on(activate_panic_buyer());

    // With async, you have new ways of control flow
    // The futures crate provides a handful of options
    // https://docs.rs/futures/0.3.13/

    // Adapters (trait futures::future::FutureExt)
    // .map is one example, see API docs for more
    let increment_and_square = increment(4).map(|n| n * n);
    let result3 = block_on(increment_and_square);
    assert_eq!(25, result3);

    // The join! macro
    // Polls multiple futures simultaneously, returning a tuple of all results once complete
    // See more realistic example with benchmark at https://github.com/bsoptei/regression_compare/
    block_on(async {
        let (result4, result5) = join![increment(3), increment_desugared(1)];
        assert_eq!((4, 2), (result4, result5));
    });

    // The  try_join! macro
    // It is similar to join!, but completes immediately if any of the futures return an error
    block_on(async {
        let result6 = try_join![try_plus(1, 1), try_plus(200, 200)];
        assert!(result6.is_err());
    });

    // The select! macro
    // It runs multiple futures simultaneously, allowing the user to respond as soon as any future completes
    block_on(async {
        let result7 = select![
            a = increment(3).fuse() => a,
            b = try_plus(1, 1).fuse() => b.unwrap() as u64
        ];
        dbg!(result7);
    });
}

#[cfg(test)]
mod tests {
    use crate::*;
    // You can use block_on in tests, however async crates can provide other solutions
    #[test]
    fn test_increment() {
        let n = 3;
        let result = block_on(increment(n));
        assert_eq!(4, result);
    }
}
