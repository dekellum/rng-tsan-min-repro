#[cfg(test)]
mod tests {

    use rand::thread_rng;
    use rand::Rng;

    use std::thread;
    use std::time::Duration;

    #[test]
    fn hammer() {
        let threads: Vec<_> = (0..100).map(|_| thread::spawn(|| {
            let ms = thread_rng().gen_range(3, 12);
            thread::sleep(Duration::from_millis(ms));
        })).collect();

        for t in threads {
            t.join().unwrap();
        }
    }
}
