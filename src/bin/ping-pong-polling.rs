use std::{
    sync::mpsc::{self, TryRecvError},
    thread,
    time::Instant,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || thread_main(rx));

    loop {
        let begin_at = Instant::now();

        let (otx, orx) = oneshot::channel();
        tx.send(otx).unwrap();
        loop {
            match orx.try_recv() {
                Ok(()) => break,
                Err(oneshot::TryRecvError::Empty) => continue,
                Err(oneshot::TryRecvError::Disconnected) => panic!(),
            }
        }

        println!("{:?}", begin_at.elapsed());
    }
}

fn thread_main(rx: mpsc::Receiver<oneshot::Sender<()>>) {
    loop {
        match rx.try_recv() {
            Ok(cb) => cb.send(()).unwrap(),
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => break,
        }
    }
}
