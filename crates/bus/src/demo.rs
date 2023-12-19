use std::time::Duration;

use canbusnoop_core::Frame;

pub(crate) struct Reader {
    iteration: usize,
    prng: oorandom::Rand32,
}

impl Reader {
    pub(crate) fn new() -> Self {
        Reader {
            prng: oorandom::Rand32::new(0),
            iteration: 0,
        }
    }

    pub(crate) async fn read(&mut self) -> Option<Frame> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let rand_id_index = self.prng.rand_range(0..(IDS.len() as u32)) as usize;
        let id = IDS[rand_id_index];
        let data = vec![1, 2, 3, 4, 5, 6, 7, self.iteration as u8];
        let frame = Frame::new(id, data);
        Some(frame)
    }
}

const IDS: [u32; 8] = [
    0x12345678, 0x11223344, 0xabcddcba, 0xdeadbeef, 0xc0cac01a, 0x10101010, 0x00000000, 0xffffffff,
];
