#[derive(Serialize, Deserialize)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
