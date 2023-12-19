use std::time::Duration;

use anyhow::Result;
use canbusnoop_core::Frame;

pub(crate) struct DemoBusReader {
    iteration: usize,
}

impl DemoBusReader {
    pub(crate) fn new() -> Result<DemoBusReader> {
        Ok(DemoBusReader { iteration: 0 })
    }

    pub(crate) async fn read(&mut self) -> Option<Frame> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let id = 0x12345678;
        let data = vec![1, 2, 3, 4, 5, 6, 7, self.iteration as u8];
        let frame = Frame::new(id, data);
        Some(frame)
    }
}
