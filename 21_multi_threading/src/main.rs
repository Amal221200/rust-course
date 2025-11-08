use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // Spawning thread
    // thread::spawn(|| {
    //   for i in 1..100 {
    //     println!("printed {i} from another thread");
    //     thread::sleep(Duration::from_millis(1));
    //   }
    // });

    // for i in 1..100 {
    //   println!("printed {i} from main thread");
    //   thread::sleep(Duration::from_millis(1));
    // }

    // Awaiting for a thread to finish
    // let handle = thread::spawn(|| {
    //   for i in 1..100 {
    //     println!("printed {i} from another thread");
    //     thread::sleep(Duration::from_millis(1));
    //   }
    // });
    // handle.join().unwrap();  // Waiting for the thread to finish

    // for i in 1..100 {
    //   println!("printed {i} from main thread");
    //   thread::sleep(Duration::from_millis(1));
    // }

    // Moving ownership to another thread
    // let nums = vec![1, 2, 4];
    // thread::spawn(move || {
    //   println!("{:?}", nums);
    // });
    //
    // Message passing
    // Simple example
    // let (tx, rc) = mpsc::channel();
    // thread::spawn(move || {
    //   tx.send("Hello World").unwrap();
    // });
    
    // println!("{}", rc.recv().unwrap());
    
    // Complex example
    let (tx, rc) = mpsc::channel();
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut result: u64 = 0;
            for j in (i * 10000000)..(((i + 1) * 10000000) - 1) {
                result += j;
            }
            producer.send(result).unwrap();
        });
    }
    drop(tx);
    let mut final_sum: u64 = 0;
    for val in rc {
      final_sum += val;
    }
    println!("{final_sum}");
}
