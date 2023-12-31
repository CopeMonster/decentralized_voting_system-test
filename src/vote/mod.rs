use dvslib::u32_bytes;

#[derive(Debug)]
pub struct Vote {
    pub voted_id: u32,
    pub vote: String
}

impl Vote {
    pub fn new(voted_id: u32, vote: &str) -> Vote {
        Vote {
            voted_id,
            vote: vote.to_string(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(u32_bytes(self.voted_id));
        bytes.extend(self.vote.as_bytes());

        bytes
    }
}