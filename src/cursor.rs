use ratatui::layout::Position;

pub struct Cursor {
    pub position: Position
}

impl Cursor {

    /// Instantiates a new cursor
    pub fn new() -> Self {
        Cursor {
            position: Position::new(0, 0)
        }
    }

    /// Moves the cursor one position to the left
    pub fn move_left(&mut self) {
        if self.position.x == 0 { return };

        self.position.x -= 1;
    }

    /// Moves the cursor one position to the right
    pub fn move_right(&mut self) {
        self.position.x += 1;
    }

    /// Moves the cursor one line above
    pub fn move_up(&mut self) {
        if self.position.y == 0 { return };

        self.position.y -= 1;
    }

    /// Moves the cursor one line below
    pub fn move_down(&mut self) {
        self.position.y += 1;
    }

    /// Adds an offset to the current cursor position
    pub fn add_offset(&mut self, x: u16, y: u16) {
        self.position.x += x;
        self.position.y += y;
    }
}