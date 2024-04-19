use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // create a thread
    thread::spawn(move || {
        // everything in here runs in a separate thread
        for i in 0..10 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
        let val = String::from("Hello");
        let s= tx.send(val);
        println!("{:?}", s);
        match s {
           Ok(()) =>print!("succes sending"),
           Err(e)=>print!("error sending error:{}",e.to_string())
        }
      
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));

    }
    let recieve=rx.recv();
   match recieve{
    Ok(val)=>println!("recieved value:{}",val),
    Err(e)=>print!("error recieving error:{}",e.to_string())
   }
  
}
use std::sync::{Arc};
use std::thread;






fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("spawned thread");
            println!("printed data in the spawned thread {:?}", data);
        });
        handles.push(handle);
    }

    println!("main thread {:?}", data);
    println!("handles:{:#?}", handles[0]);
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}


