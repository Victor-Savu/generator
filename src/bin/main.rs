extern crate generator;

#[cfg(not(test))]
fn main() {
    use generator::gen;

    println!("Creating the generator.");
    let c = gen::Generator::<i64>::new(|m| {
        println!("Entered the generator lambda");
        let mut i = 0i64;
        loop {
            println!("Yielding form the generator lambda.");
            m.y(i);
            i = i+1;
        }
    });

    println!("Collecting 10 results.");
    let v: Vec<_> = c.iter().take(10).collect();
    let w: Vec<i64> = (0..10).collect();
    assert_eq!(v, w);
    assert!(true);
}
