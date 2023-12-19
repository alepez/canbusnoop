use std::time::Duration;

use canbusnoop_core::Frame;

pub(crate) struct Reader {
    iteration: usize,
}

impl Reader {
    pub(crate) fn new() -> Self {
        Reader { iteration: 0 }
    }

    pub(crate) async fn read(&mut self) -> Option<Frame> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let id = 0x12345678;
        let data = vec![1, 2, 3, 4, 5, 6, 7, self.iteration as u8];
        let frame = Frame::new(id, data);
        Some(frame)
    }
}
