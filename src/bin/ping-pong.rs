use std::{sync::mpsc, thread, time::Instant};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || thread_main(rx));

    loop {
        let begin_at = Instant::now();

        let (otx, orx) = oneshot::channel();
        tx.send(otx).unwrap();
        orx.recv().unwrap();

        println!("{:?}", begin_at.elapsed());
    }
}

fn thread_main(rx: mpsc::Receiver<oneshot::Sender<()>>) {
    loop {
        let cb = rx.recv().unwrap();
        cb.send(()).unwrap();
    }
}
