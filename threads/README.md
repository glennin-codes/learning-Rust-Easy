# Threads
- In Rust, threads allow you to split the computation in your program into multiple threads, enabling concurrent execution of tasks. This can improve the performance of your code by utilizing multiple cores of a CPU.
- To create a new thread in Rust, you use the thread::spawn() function from the std module. This function takes a closure as an argument, which contains the code to be executed in the new thread. Here's a simple example:
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // create a thread
    thread::spawn(|| {
        // everything in here runs in a separate thread
        for i in 0..10 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

```
- In this example, a new thread is spawned that prints numbers from 0 to 9, while the main thread prints numbers from 0 to 4. The thread::sleep function is used to pause the execution of a thread for a specified duration, allowing other threads to run.

## communication
- Rust also provides channels for communication between threads. Channels allow threads to send and receive messages, which is useful for safely passing data between threads:
example
```rust 
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("Hello");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Received: {}", received);

```

