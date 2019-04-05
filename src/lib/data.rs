use std::sync::{Arc, Mutex};

pub struct Grid {
    pub grid: Arc<Mutex<Vec<Vec<RGB>>>>

}

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
