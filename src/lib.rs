#[cfg(test)]
mod tests {

    use rand_pcg::Lcg64Xsh32 as Pcg32;
    use rand_core::{RngCore, SeedableRng};

    use std::thread;
    use std::time::Duration;

    #[test]
    fn hammer() {
        let threads: Vec<_> = (0..100).map(|_| thread::spawn(move || {
            let mut seed = <Pcg32 as SeedableRng>::Seed::default();
            getrandom::getrandom(&mut seed).unwrap();
            let mut rng = Pcg32::from_seed(seed);
            let ms = rng.next_u64() % 10 as u64 + 3;
            thread::sleep(Duration::from_millis(ms));
        })).collect();

        for t in threads {
            t.join().unwrap();
        }
    }
}
