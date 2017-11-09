









use fieldobject::Position;






#[derive(Debug)]
pub struct Field {
    width: u8,
    height: u8,
    pub p1start: Position,
    pub p2start: Position,
}












impl Field {
    pub fn new() -> Field {
        Field {
            width: 8,
            height: 8,
            p1start: Position {
                x: 0,
                y: 4,
            },
            p2start: Position {
                x: 7,
                y: 3,
            }
        }
    }
}
