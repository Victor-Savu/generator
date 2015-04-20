use coroutine;
use std::sync::mpsc;

pub struct Generator<T, U=()> {
    coro: coroutine::coroutine::Handle,
    rx: mpsc::Receiver<T>,
    tx: mpsc::Sender<U>,
}

pub struct Scheduler<T, U=()> {
    tx: mpsc::Sender<T>,
    rx: mpsc::Receiver<U>,
}

impl<T, U> Scheduler<T, U> {
    pub fn new(tx: mpsc::Sender<T>, rx: mpsc::Receiver<U>) -> Self {
        Scheduler{tx: tx, rx: rx}
    }

    pub fn sched(&self, t: T) -> Option<U> {
        self.tx.send(t).unwrap();

        coroutine::sched();

        self.rx.recv().ok()
    }
}


pub struct Iter<'a, T: 'a, U: 'a = ()> {
    gen: &'a Generator<T, U>,
    disconnected: bool,
}

impl<'a, T: 'a, U: 'a> Iter<'a, T, U> {
    fn new(gen: &'a Generator<T, U>) -> Self {
        Iter{ gen: gen, disconnected: false}
    }

    pub fn next_with(&mut self, u: U) -> Option<T> {
        self.gen.tx.send(u).unwrap();
        if self.disconnected {
            None
        } else {
            match self.gen.rx.try_recv() {
                Ok(t) => Some(t),
                Err(mpsc::TryRecvError::Empty) => {
                    match self.gen.coro.resume() {
                        Ok(_) => {
                            match self.gen.rx.recv() {
                                Ok(t) => Some(t),
                                Err(_) => { self.disconnected = true; None }
                            }
                        },
                        _ => {
                            self.disconnected = true;
                            None
                        }
                    }
                },
                Err(mpsc::TryRecvError::Disconnected) => { self.disconnected = true; None}
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.gen.tx.send(()).unwrap();
        if self.disconnected {
            None
        } else {
            match self.gen.rx.try_recv() {
                Ok(t) => Some(t),
                Err(mpsc::TryRecvError::Empty) => {
                    match self.gen.coro.resume() {
                        Ok(_) => {
                            match self.gen.rx.recv() {
                                Ok(t) => Some(t),
                                Err(_) => { self.disconnected = true; None }
                            }
                        },
                        _ => {
                            self.disconnected = true;
                            None
                        }
                    }
                },
                Err(mpsc::TryRecvError::Disconnected) => { self.disconnected = true; None}
            }
        }
    }
}

impl<T: Send + 'static, U: Send + 'static> Generator<T, U> {
    pub fn new<F>(f: F) -> Self  where F: Fn(Scheduler<T, U>) + Send + 'static {
        let (tx, mrx) = mpsc::channel::<T>();
        let (mtx, rx) = mpsc::channel::<U>();
        let m = Scheduler::new(tx, rx);
        let coro = coroutine::spawn(move || f(m)) ;
        Generator{ coro: coro, rx: mrx, tx: mtx }
    }

    pub fn iter(&self) -> Iter<T, U> {
        Iter::new(&self)
    }
}

