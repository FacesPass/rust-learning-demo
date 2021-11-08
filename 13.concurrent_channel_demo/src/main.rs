use std::sync::mpsc::{channel, sync_channel};
use std::thread;

fn main() {
    // 异步接收管道，无限空间大小，直到超出系统内存
    // let (sender, receiver) = channel();
    // let join_handle = thread::spawn(move || {
    //     while let Ok(n) = receiver.recv() {
    //         println!("Received: {}", n);
    //     }
    // });

    // for i in 0..10 {
    //     sender.send(i).unwrap();
    // }

    // join_handle.join().unwrap();

    // 同步管道，有限空间大小，填满时send会被阻塞，直到管道中出现更多的空间
    let (sender, receiver) = sync_channel(1);
    let sender_clone = sender.clone();
    let _ = sender.send(0);
    thread::spawn(move || {
        let _ = sender.send(1);
    });

    thread::spawn(move || {
        let _ = sender_clone.send(2);
    });

    println!("Received {} via the channel", receiver.recv().unwrap());
    println!("Received {} via the channel", receiver.recv().unwrap());
    println!("Received {} via the channel", receiver.recv().unwrap());
    //对于这两种通道类型，如果通道是空的，那么 recv 调用会返回 Err 值
    println!("Received {:?} via the channel", receiver.recv());
}
