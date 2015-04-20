extern crate coroutine;

pub mod gen;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use coroutine;

#[test]
    fn channels() {
        let (tx, rx) = mpsc::channel();

        // Spawn a new coroutine
        let coro = coroutine::spawn(move|| {
            tx.send("Hello in coroutine!").unwrap();

            // Yield back to it's parent
            coroutine::sched();

            tx.send("We are back!!").unwrap();

            let c = tx.clone();

            // Spawn a new coroutine
            coroutine::spawn(move|| {
                c.send("Hello inside").unwrap();
            }).join().unwrap();

            tx.send("Good bye").unwrap();
        });

        coro.resume().unwrap();

        {
            let msg = rx.recv().unwrap();
            assert_eq!(msg, "Hello in coroutine!");
        }

        // Resume the coroutine
        coro.resume().unwrap();

        for m in &["We are back!!", "Hello inside", "Good bye"] {
            let msg = rx.recv().unwrap();
            assert_eq!(&msg, m); 
        }
    }

#[test]
    fn counter() {
        let c = gen::Generator::<i64>::new(|s| {
            let mut i = 0i64;
            loop {
                s.sched(i);
                i = i+1;
            }
        });

        let v: Vec<_> = c.iter().take(10).collect();
        let w: Vec<i64> = (0..10).collect();
        assert_eq!(v, w);
        assert!(true);
    }

    #[test]
    fn comm() {
        let c = gen::Generator::<i64, i64>::new(|s| {
            let mut i = 0i64;
            while let Some(j) = s.sched(i) {
                i = i + j;
            }
        });

        let mut ci = c.iter();
        assert!((0..10).map(move |i| ci.next_with(i).unwrap())
                       .zip([0, 0, 1, 3, 6, 10, 15, 21, 28, 36].iter())
                       .all(|(i, j)| i == *j));
    }
}
