use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
}

pub struct Field(HashSet<Cell>);

type Counts = HashMap<Cell, u64>;
type Neighbors = [Cell; 8];


impl Cell {
    fn neighbors(&self, neighbors: &mut Neighbors) {
        let mut i = 0;
        for x in self.x - 1..self.x + 2 {
            for y in self.y - 1..self.y + 2 {
                if (x, y) != (self.x, self.y) {
                    neighbors[i] = Cell { x: x, y: y };
                    i += 1;
                }
            }
        }
    }
}

impl Field {
    fn empty() -> Field {
        return Field(HashSet::new());
    }

    pub fn new(desc: &str) -> Field {
        let mut field = Field(HashSet::new());

        for (y, line) in desc.split('\n').enumerate() {
            for (x, elem) in line.chars().enumerate() {
                if elem == 'X' {
                    field.0.insert(Cell {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        return field;
    }

    fn add(&mut self, cell: Cell) {
        self.0.insert(cell);
    }

    fn neighbor_counts(&self) -> Counts {
        let mut counts: Counts = HashMap::new();

        let mut neighbors: Neighbors = [Cell { x: 10, y: 10 }; 8];
        for cell in &self.0 {
            cell.neighbors(&mut neighbors);
            for neighbor in &neighbors {
                let found = match counts.get_mut(neighbor) {
                    Some(count) => {
                        *count += 1;
                        true
                    }
                    None => false,
                };
                if !found {
                    counts.insert(*neighbor, 1);
                }
            }
        }

        return counts;
    }

    pub fn step(&mut self) -> Field {
        let mut field = Field::empty();

        for (cell, count) in self.neighbor_counts() {
            if count == 3 || self.0.contains(&cell) && count == 2 {
                field.add(cell);
            }
        }

        return field;
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "empty");
        }

        let padding = 2;

        let mut minx = i64::max_value();
        let mut maxx = i64::min_value();
        let mut miny = i64::max_value();
        let mut maxy = i64::min_value();

        for cell in &self.0 {
            if cell.x < minx {
                minx = cell.x;
            }
            if cell.x > maxx {
                maxx = cell.x;
            }
            if cell.y < miny {
                miny = cell.y;
            }
            if cell.y > maxy {
                maxy = cell.y;
            }
        }

        for y in miny - padding..maxy + 1 + padding {
            for x in minx - padding..maxx + 1 + padding {
                if self.0.contains(&Cell { x: x, y: y }) {
                    try!(f.write_char('X'));
                } else {
                    try!(f.write_char('.'));
                }
            }
            try!(f.write_char('\n'));
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Cell;
    use super::Field;

    #[test]
    fn create_cell() {
        let x = 1;
        let y = 2;
        let cell = Cell { x: x, y: y };
        assert_eq!(x, cell.x);
        assert_eq!(y, cell.y);
    }

    #[test]
    fn generate_board() {
        let description = "X..\n.XX";
        let mut expected = Field::empty();
        expected.add(Cell { x: 0, y: 0 });
        expected.add(Cell { x: 1, y: 1 });
        expected.add(Cell { x: 2, y: 1 });

        let actual = Field::new(description);
        assert_eq!(expected.0, actual.0);
    }

    #[test]
    fn neighbors_works() {
        let cell = Cell { x: 0, y: 1 };
        let mut actual = [Cell { x: 10, y: 10 }; 8];
        let expected = [Cell { x: -1, y: 0 },
                                   Cell { x: -1, y: 1 },
                                   Cell { x: -1, y: 2 },
                                   Cell { x: 0, y: 0 },
                                   Cell { x: 0, y: 2 },
                                   Cell { x: 1, y: 0 },
                                   Cell { x: 1, y: 1 },
                                   Cell { x: 1, y: 2 }];
        cell.neighbors(&mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn neighbor_counts_work() {
        let field = Field::new("X.\n.X");

        let actual = field.neighbor_counts();
        let mut expected = HashMap::new();

        expected.insert(Cell { x: 2, y: 2 }, 1);
        expected.insert(Cell { x: 2, y: 0 }, 1);
        expected.insert(Cell { x: 0, y: -1 }, 1);
        expected.insert(Cell { x: 1, y: 0 }, 2);
        expected.insert(Cell { x: 0, y: 1 }, 2);
        expected.insert(Cell { x: 0, y: 0 }, 1);
        expected.insert(Cell { x: 1, y: 2 }, 1);
        expected.insert(Cell { x: 1, y: -1 }, 1);
        expected.insert(Cell { x: 2, y: 1 }, 1);
        expected.insert(Cell { x: 1, y: 1 }, 1);
        expected.insert(Cell { x: -1, y: 0 }, 1);
        expected.insert(Cell { x: 0, y: 2 }, 1);
        expected.insert(Cell { x: -1, y: 1 }, 1);
        expected.insert(Cell { x: -1, y: -1 }, 1);

        assert_eq!(14, actual.len());
        assert_eq!(expected, actual);
    }
}
