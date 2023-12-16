mod tcp_client;
mod tcp_server;
mod udp_server;
mod udp_client;
mod net;

use std::{sync::mpsc, thread};

fn main() {
    let rhs = vec![0, 10, 20, 30, 40, 50];
    let lhs = vec![0, 1, 2, 3, 4, 5];
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    for i in 0..rhs.len() {
        let rhs = rhs.clone();
        let lhs = lhs.clone();
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let s = format!(
                "Thread {} added {} and {}, result {}",
                i,
                rhs[i],
                lhs[i],
                rhs[i] + lhs[i]
            );
            tx.send(s).unwrap();
        });
        handles.push(handle);
    }

    // let _ = handles.into_iter().map(|h| h.join());
    drop(tx);
    for result in rx {
        println!("{}", result);
    }
}
