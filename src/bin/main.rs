extern crate generator;

#[cfg(not(test))]
fn main() {
    use generator::gen;

    let c = gen::Generator::new(|s| {
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

    let c = gen::Generator::<i64, i64>::new(|s| {
        let mut i = 0i64;
        while let Some(j) = s.sched(i) {
            i = i + j;
        }
    });


    let mut ci = c.iter();
    (0..10).map(|i| ci.next_with(i).unwrap())
}
