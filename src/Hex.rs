pub struct Hex {    
    column: i32,
    row: i32,
    elevation: f32,
}

impl Hex {
    fn new(column: i32, row:i32) -> Self {
        Hex {
            column,
            row,
            elevation: -1.0,
        }
    }
}



