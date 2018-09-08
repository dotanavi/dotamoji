
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Info {
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
}

impl Info {
    pub fn new(left_id: u16, right_id: u16, cost: i16) -> Self {
        Info {
            left_id,
            right_id,
            cost,
        }
    }
}
