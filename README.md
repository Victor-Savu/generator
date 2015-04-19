# generator
Generators (semi-coroutines) in Rust

[Generators](https://en.wikipedia.org/w/index.php?title=Generator_%28computer_programming%29) are like quesadillas -> they are very good!

However, this is *not* (**yet**) the best possible implementation of generators. Since my assembly skills are very limited, I went for the heavyweight approach and combined context switching from the [coroutine](https://crates.io/crates/coroutine) library with Rust's ```std::sync::mpsc``` channels.

This is very much a work-in-progress, so please join in on the fun!

---
If you are reading this and you could teach me assembly, please drop a comment.
