use crate::cursor::Cursor;

struct Piece {
    pub which: char,
    pub start: usize,
    pub len: usize,
}

impl Piece {
    pub fn new(which: char, start: usize, len: usize) -> Piece {
        Piece { which, start, len }
    }
}

pub struct BufferHandler {
    pub original_buffer: String, // TODO: Change this to &str when I'm smart enough to figure iout
    pub add_buffer: String,
    pub cursor: Cursor,

    piece_table: Vec<Piece>,
    modified: bool,
}

impl BufferHandler {
    pub fn from(original_buffer: String) -> BufferHandler {
        BufferHandler {
            original_buffer,
            add_buffer: String::new(),
            piece_table: vec![],
            modified: false,
            cursor: Cursor::new(),
        }
    }

    pub fn write(&mut self, content: &str) {
        self.add_buffer.push_str(content);
        self.modified = true;

        // Inserting at the end of the table
        match self.piece_table.last_mut() {
            None => self.piece_table.push(Piece::new('a', 0, 1)),
            Some(piece) => {
                if piece.which == 'a' {
                    piece.len += 1;
                    return;
                }

                self.piece_table.push(Piece::new('a', 0, 1));
            }
        }
    }

    pub fn pop(&mut self) {
        self.add_buffer.pop();
        self.modified = true;

        // Popping at the end of table
        let piece = self.piece_table.last_mut().unwrap();
        piece.len -= 1;
    }
}
