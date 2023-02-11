use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

    });

    loop
    {
        match rx.try_recv() {
            Ok(val) => {
                println!("Received: {val}");
                break;
            },
            Err(e) => println!("{e}"),
        }
    }
}
