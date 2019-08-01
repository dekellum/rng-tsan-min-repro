#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use std::thread;

    use std::time::Duration;

    lazy_static! {
        static ref LAZY_STATE: u64 = 2;
    }

    #[test]
    fn hammer() {
        let threads: Vec<_> = (0..100).map(|i| thread::spawn(move || {
            let j: u64 = *LAZY_STATE;
            thread::sleep(Duration::from_micros(i + j));
        })).collect();

        for t in threads {
            t.join().unwrap();
        }
    }
}
