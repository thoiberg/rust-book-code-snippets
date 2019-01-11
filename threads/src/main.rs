use std::thread;
use std::time::Duration;

// any spawned threads will die if the parent thread ends
// spawning a thread returns a JoinHandle, which we can use to keep the parent
//  thread alive until the spawned thread completes
// unwrapping the thread will block the main thread until it completes

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
