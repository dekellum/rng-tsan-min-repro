#[cfg(test)]
mod tests {

    use rand_pcg::Lcg64Xsh32 as Pcg32;
    use rand_core::{RngCore, SeedableRng};

    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn hammer() {
        let threads: Vec<_> = (0..100).map(|i| thread::spawn(move || {
            let rnd_state = RandomState::new();
            let mut hasher = rnd_state.build_hasher();
            i.hash(&mut hasher);
            let hash: u64 = hasher.finish();
            let mut rng = Pcg32::seed_from_u64(hash);
            let ms = rng.next_u64() % 10 as u64 + 3;
            thread::sleep(Duration::from_millis(ms));
        })).collect();

        for t in threads {
            t.join().unwrap();
        }
    }
}
