/*
 * Fun with threading and practice using Mutex with Arc
 * Also practice with multithreading and mpsc channels
 */

use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let tc: u32 = 10;
    let wlist: Vec<String> = Vec::new();
    let wlist_tg = Arc::new(Mutex::new(wlist));
    let mut thandles = vec![];
    let (tx, rx) = mpsc::channel();

    for i in 0..tc {
        let schan = mpsc::Sender::clone(&tx);
        let wlist_tg = Arc::clone(&wlist_tg);
        let handle = thread::spawn(move || {
            work_func(i,wlist_tg,schan);
        });
        thandles.push(handle);
    }

    let mut ti: u32 = 0;
    for recvd in rx {
        if ti < 9 {
            println!("[!] {}",recvd);
            ti += 1;
        } else if ti == 9{
            println!("[!] {}", recvd);
            break;
        }
    }

    println!("Iterating through worked on vector!!!");
    let entries = wlist_tg.lock().unwrap();
    let mut count = 1;
    for e in entries.iter() {
        println!("[{}] {}",count,e);
        count += 1;
    }
}

fn work_func(tn: u32,wlist_tg: Arc<Mutex<Vec<String>>>,chan_send: mpsc::Sender<String>) {
    let mut c: u32 = 0; 
    loop {
        if c < 10 {
            println!("Hello! from thread {}",tn+1);
            let mut entry = wlist_tg.lock().unwrap();
            entry.push(format!("I am {} from thread {}",c+1,tn+1));
            c += 1;
        } else {
            match chan_send.send(format!("Thread {} shutting down!",tn+1)) {
                Ok(_) => {break},
                Err(_) => {println!("Receiver not listening! Dying!");return}
            };
        }
    }
}
