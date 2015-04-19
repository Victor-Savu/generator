use coroutine;
use std::sync::mpsc;

pub struct Generator<T> {
    coro: coroutine::coroutine::Handle,
    rx: mpsc::Receiver<T>,
}

pub struct Messenger<T> {
    tx: mpsc::Sender<T>,
}

impl<T> Messenger<T> {
    pub fn new(tx: mpsc::Sender<T>) -> Self {
        Messenger{tx: tx}
    }

    pub fn y(&self, t: T) {
        self.tx.send(t).unwrap();
        coroutine::sched();
    }
}


pub struct Iter<'a, T: 'a> {
    gen: &'a Generator<T>,
    disconnected: bool,
}

impl<'a, T: 'a> Iter<'a, T> {
    fn new(gen: &'a Generator<T>) -> Self {
        Iter{ gen: gen, disconnected: false}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("Getting the next value.");

        if self.disconnected {
            None
        } else {
            println!("Trying to receive some.");
            match self.gen.rx.try_recv() {
                Ok(t) => Some(t),
                Err(mpsc::TryRecvError::Empty) => {
                    println!("Queue is empty. Resuming coroutine.");
                    match self.gen.coro.resume() {
                        Ok(_) => {
                            println!("Coroutine resumed. Now we are bound to wait for a result.");
                            match self.gen.rx.recv() {
                                Ok(t) => Some(t),
                                Err(_) => { self.disconnected = true; None }
                            }
                        },
                        _ => { self.disconnected = true; None }
                    }
                },
                Err(mpsc::TryRecvError::Disconnected) => { self.disconnected = true; None}
            }
        }
    }
}

impl<T: Send + 'static> Generator<T> {
    pub fn new<F>(f: F) -> Self  where F: Fn(Messenger<T>) + Send + 'static {
        let (tx, rx) = mpsc::channel::<T>();
        let m = Messenger::<T>::new(tx);
        let coro = coroutine::spawn(move || f(m)) ;
        Generator{ coro: coro, rx: rx }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::<T>::new(&self)
    }
}
