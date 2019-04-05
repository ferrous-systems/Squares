use std::sync::{Arc, Mutex};

pub struct Grid {
    pub grid: Arc<Mutex<Vec<Vec<[u8; 3]>>>>

}
