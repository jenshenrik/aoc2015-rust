use std::collections::HashMap;

enum Direction {
    North,
    South,
    East,
    West,
}

fn from_char(c: char) -> Option<Direction> {
    match c {
        '<' => Some(Direction::West),
        '>' => Some(Direction::East),
        '^' => Some(Direction::North),
        'v' => Some(Direction::South),
        _ => None,
    }
}

pub struct Santa {
    x: i32,
    y: i32,
    visited_houses: HashMap<(i32, i32), i32>,
}

impl Santa {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            visited_houses: [((0, 0), 1)].iter().cloned().collect(),
        }
    }

    pub fn visits(&self) -> usize {
        self.visited_houses.len()
    }

    pub fn move_santa(&mut self, d: &str) {
        for c in d.chars() {
            self.move_santa_single(c);
        }
    }

    pub fn move_santa_single(&mut self, d: char) {
        let direction_option = from_char(d);

        if let Some(direction) = direction_option {
            let (dx, dy) = match direction {
                Direction::West => (-1, 0),
                Direction::East => (1, 0),
                Direction::North => (0, 1),
                Direction::South => (0, -1),
            };

            self.x += dx;
            self.y += dy;
            self.visited_houses.entry((self.x, self.y)).or_insert(1);
        }
    }
}

impl Default for Santa {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::santa::Santa;

    #[test]
    fn test_move_santa_1() {
        let mut santa = Santa::new();
        santa.move_santa(">");
        let result = santa.visited_houses.len();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_move_santa_2() {
        let mut santa = Santa::new();
        santa.move_santa("^>v<");
        let result = santa.visited_houses.len();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_move_santa_3() {
        let mut santa = Santa::new();
        santa.move_santa("^v^v^v^v^v");
        let result = santa.visited_houses.len();

        assert_eq!(result, 2);
    }
}
