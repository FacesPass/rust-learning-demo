use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // 使用互斥锁
    let vec = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0..5 {
        let mut v = vec.clone();
        let t = thread::spawn(move || {
            let mut v = v.lock().unwrap();
            v.push(i);
        });
        childs.push(t);
    }

    for t in childs {
        t.join().unwrap();
    }

    println!("{:?}", vec);

    //使用读写锁
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        *m.write().unwrap() += 1;
        let updated = *m.read().unwrap();
        updated
    });

    let updated = c.join().unwrap();
    println!("{:?}", updated);
}
