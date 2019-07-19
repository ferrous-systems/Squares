#[derive(Serialize, Deserialize)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    pub row: i32,
    pub column: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub direction: i32,
    pub length: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ApiGrid {
    pub zero_row: i32,
    pub zero_column: i32,
    pub api_grid: Vec<Vec<RGB>>,
}

#[derive(Serialize, Deserialize)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
