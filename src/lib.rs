#[cfg(test)]
mod tests {

    use rand_pcg::Lcg64Xsh32 as Pcg32;
    use rand_core::{RngCore, SeedableRng};

    use std::thread;
    use std::time::Duration;

    #[test]
    fn hammer() {
        let threads: Vec<_> = (0..100).map(|i| thread::spawn(move || {
            let mut rng = Pcg32::seed_from_u64(i);
            let ms = rng.next_u64() % 10 as u64 + 3;
            thread::sleep(Duration::from_millis(ms));
        })).collect();

        for t in threads {
            t.join().unwrap();
        }
    }
}
