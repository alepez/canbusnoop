/// CAN Frame
#[derive(Debug)]
pub struct Frame {
    /// 32 bit CAN_ID + EFF/RTR/ERR flags
    id: u32,

    /// buffer for data
    data: Vec<u8>,
}

impl Frame {
    pub fn new(id: u32, data: Vec<u8>) -> Frame {
        Frame { id, data }
    }

    /// Returns the 32 bit CAN_ID + EFF/RTR/ERR flags
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the data
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
