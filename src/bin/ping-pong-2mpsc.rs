use std::{sync::mpsc, thread, time::Instant};

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || thread_main(rx1, tx2));

    loop {
        let begin_at = Instant::now();

        tx1.send(()).unwrap();
        rx2.recv().unwrap();

        println!("{:?}", begin_at.elapsed());
    }
}

fn thread_main(rx1: mpsc::Receiver<()>, tx2: mpsc::Sender<()>) {
    loop {
        rx1.recv().unwrap();
        tx2.send(()).unwrap();
    }
}
