use std::time::Duration;

use canbusnoop_core::Frame;

pub(crate) struct Reader {
    prng: oorandom::Rand32,
    ids: Vec<u32>,
}

impl Reader {
    pub(crate) fn new() -> Self {
        let mut prng = oorandom::Rand32::new(0);
        let ids = generate_ids(&mut prng);
        Reader { prng, ids }
    }

    pub(crate) async fn read(&mut self) -> Option<Frame> {
        let delay = self.prng.rand_range(1..100).into();
        tokio::time::sleep(Duration::from_millis(delay)).await;
        let rand_id_index = self.prng.rand_range(0..(self.ids.len() as u32)) as usize;
        let id = self.ids[rand_id_index];
        let data = generate_data(&mut self.prng);
        let frame = Frame::new(id, data);
        Some(frame)
    }
}

fn generate_ids(prng: &mut oorandom::Rand32) -> Vec<u32> {
    let n = prng.rand_range(8..16);
    (0..n).map(|_| prng.rand_u32()).collect()
}

fn generate_data(prng: &mut oorandom::Rand32) -> Vec<u8> {
    let n = prng.rand_range(1..8);
    (0..n).map(|_| prng.rand_u32() as u8).collect()
}
