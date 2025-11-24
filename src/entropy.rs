use rand::Rng;

pub fn cast_cowries(count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..2)).collect()
}

pub fn cowrie_to_odu(pattern: &[u32]) -> String {
    format!("Odu from pattern: {:?}", pattern)
}
