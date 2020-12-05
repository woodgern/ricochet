use std::fmt;
use std::fs;
use std::rc::Rc;

use thincollections::thin_vec::ThinVec;

const SIXTEEN_POWER_1: u64 = 16;
const SIXTEEN_POWER_2: u64 = 256;
const SIXTEEN_POWER_3: u64 = 4096;
const SIXTEEN_POWER_4: u64 = 65536;
const SIXTEEN_POWER_5: u64 = 1048576;
const SIXTEEN_POWER_6: u64 = 16777216;
const SIXTEEN_POWER_7: u64 = 268435456;
// const SIXTEEN_POWER_8: u64 = 4294967296;
// const SIXTEEN_POWER_9: u64 = 68719476736;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Eq for Position {}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Copy, Clone)]
pub enum Colour {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    position: Position,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    up_move: Position,
    down_move: Position,
    left_move: Position,
    right_move: Position,
}

impl Tile {
    pub fn get_adjacent_position(&self, direction: Direction) -> Position {
        match direction {
            Direction::UP => {
                Position {x: self.position.x, y: self.position.y - 1}
            },
            Direction::DOWN => {
                Position {x: self.position.x, y: self.position.y + 1}
            },
            Direction::LEFT => {
                Position {x: self.position.x - 1, y: self.position.y}
            },
            Direction::RIGHT => {
                Position {x: self.position.x + 1, y: self.position.y}
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    map: Rc<[[Tile; 16]; 16]>,
    pub red: Position,
    pub green: Position,
    pub blue: Position,
    pub yellow: Position,
    goal: Position,
}

impl Board {

    // public
    pub fn new(
        red: Position,
        green: Position,
        blue: Position,
        yellow: Position,
        goal: Position,
    ) -> Board {
        let map = Rc::new(load_map());
        let new_board = Board {
            red: red,
            green: green,
            blue: blue,
            yellow: yellow,
            goal: goal,
            map: map,
        };
        return new_board
    }

    pub fn get_goal(&self) -> Position {
        return self.goal
    }

    pub fn get_valid_directions(&self, colour: Colour) -> ThinVec<Direction> {
        let robot_position = self.get_robot_by_colour(colour);
        let tile = self.get_tile(robot_position);

        let mut directions = ThinVec::new();
        if tile.up && !self.is_occupied(tile.get_adjacent_position(Direction::UP)) {
            directions.push(Direction::UP);
        }
        if tile.down && !self.is_occupied(tile.get_adjacent_position(Direction::DOWN)) {
            directions.push(Direction::DOWN);
        }
        if tile.left && !self.is_occupied(tile.get_adjacent_position(Direction::LEFT)) {
            directions.push(Direction::LEFT);
        }
        if tile.right && !self.is_occupied(tile.get_adjacent_position(Direction::RIGHT)) {
            directions.push(Direction::RIGHT);
        }
        return directions
    }

    pub fn get_robot_by_colour(&self, colour: Colour) -> Position {
        match colour {
            Colour::RED => self.red,
            Colour::BLUE => self.blue,
            Colour::GREEN => self.green,
            Colour::YELLOW => self.yellow,
        }
    }

    pub fn move_robot(&mut self, colour: Colour, direction: Direction) -> Position {
        let tile = self.get_tile(self.get_robot_by_colour(colour));
        let end_position = match direction {
            Direction::UP => {
                let mut end_position = tile.up_move.clone();
                let mut collision_y = end_position.y;
                if self.red.x == end_position.x && self.red.y < tile.position.y && self.red.y >= end_position.y {
                    if collision_y < self.red.y + 1 {
                        collision_y = self.red.y + 1;
                    }
                }
                if self.green.x == end_position.x && self.green.y < tile.position.y && self.green.y >= end_position.y {
                    if collision_y < self.green.y + 1 {
                        collision_y = self.green.y + 1;
                    }
                }
                if self.blue.x == end_position.x && self.blue.y < tile.position.y && self.blue.y >= end_position.y {
                    if collision_y < self.blue.y + 1 {
                        collision_y = self.blue.y + 1;
                    }
                }
                if self.yellow.x == end_position.x && self.yellow.y < tile.position.y && self.yellow.y >= end_position.y {
                    if collision_y < self.yellow.y + 1 {
                        collision_y = self.yellow.y + 1;
                    }
                }
                end_position.y = collision_y;
                end_position
            },
            Direction::DOWN => {
                let mut end_position = tile.down_move.clone();
                let mut collision_y = end_position.y;
                if self.red.x == end_position.x && self.red.y > tile.position.y && self.red.y <= end_position.y {
                    if collision_y > self.red.y - 1 {
                        collision_y = self.red.y - 1;
                    }
                }
                if self.green.x == end_position.x && self.green.y > tile.position.y && self.green.y <= end_position.y {
                    if collision_y > self.green.y - 1 {
                        collision_y = self.green.y - 1;
                    }
                }
                if self.blue.x == end_position.x && self.blue.y > tile.position.y && self.blue.y <= end_position.y {
                    if collision_y > self.blue.y - 1 {
                        collision_y = self.blue.y - 1;
                    }
                }
                if self.yellow.x == end_position.x && self.yellow.y > tile.position.y && self.yellow.y <= end_position.y {
                    if collision_y > self.yellow.y - 1 {
                        collision_y = self.yellow.y - 1;
                    }
                }
                end_position.y = collision_y;
                end_position
            },
            Direction::LEFT => {
                let mut end_position = tile.left_move.clone();
                let mut collision_x = end_position.x;
                if self.red.y == end_position.y && self.red.x < tile.position.x && self.red.x >= end_position.x {
                    if collision_x < self.red.x + 1 {
                        collision_x = self.red.x + 1;
                    }
                }
                if self.green.y == end_position.y && self.green.x < tile.position.x && self.green.x >= end_position.x {
                    if collision_x < self.green.x + 1 {
                        collision_x = self.green.x + 1;
                    }
                }
                if self.blue.y == end_position.y && self.blue.x < tile.position.x && self.blue.x >= end_position.x {
                    if collision_x < self.blue.x + 1 {
                        collision_x = self.blue.x + 1;
                    }
                }
                if self.yellow.y == end_position.y && self.yellow.x < tile.position.x && self.yellow.x >= end_position.x {
                    if collision_x < self.yellow.x + 1 {
                        collision_x = self.yellow.x + 1;
                    }
                }
                end_position.x = collision_x;
                end_position
            },
            Direction::RIGHT => {
                let mut end_position = tile.right_move.clone();
                let mut collision_x = end_position.x;
                if self.red.y == end_position.y && self.red.x > tile.position.x && self.red.x <= end_position.x {
                    if collision_x > self.red.x - 1 {
                        collision_x = self.red.x - 1;
                    }
                }
                if self.green.y == end_position.y && self.green.x > tile.position.x && self.green.x <= end_position.x {
                    if collision_x > self.green.x - 1 {
                        collision_x = self.green.x - 1;
                    }
                }
                if self.blue.y == end_position.y && self.blue.x > tile.position.x && self.blue.x <= end_position.x {
                    if collision_x > self.blue.x - 1 {
                        collision_x = self.blue.x - 1;
                    }
                }
                if self.yellow.y == end_position.y && self.yellow.x > tile.position.x && self.yellow.x <= end_position.x {
                    if collision_x > self.yellow.x - 1 {
                        collision_x = self.yellow.x - 1;
                    }
                }
                end_position.x = collision_x;
                end_position
            },
        };
        self.set_robot_by_colour(colour, end_position);

        return end_position;
    }

    pub fn is_solved(&self, colour: Colour) -> bool {
        return match colour {
            Colour::RED => self.red == self.goal,
            Colour::BLUE => self.blue == self.goal,
            Colour::YELLOW => self.yellow == self.goal,
            Colour::GREEN => self.green == self.goal,
        }
    }

    pub fn hash(&self) -> u64 {
        return self.red.x as u64 +
            SIXTEEN_POWER_1 * self.red.y as u64 +
            SIXTEEN_POWER_2 * self.green.x as u64 +
            SIXTEEN_POWER_3 * self.green.y as u64 +
            SIXTEEN_POWER_4 * self.blue.x as u64 +
            SIXTEEN_POWER_5 * self.blue.y as u64 +
            SIXTEEN_POWER_6 * self.yellow.x as u64 +
            SIXTEEN_POWER_7 * self.yellow.y as u64

    }

    pub fn permuted_hashes(&self) -> [u64; 6] {
        return [self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.green.x as u64) +
            (SIXTEEN_POWER_3 * self.green.y as u64) +
            (SIXTEEN_POWER_4 * self.blue.x as u64) +
            (SIXTEEN_POWER_5 * self.blue.y as u64) +
            (SIXTEEN_POWER_6 * self.yellow.x as u64) +
            (SIXTEEN_POWER_7 * self.yellow.y as u64),
            self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.green.x as u64) +
            (SIXTEEN_POWER_3 * self.green.y as u64) +
            (SIXTEEN_POWER_4 * self.yellow.x as u64) +
            (SIXTEEN_POWER_5 * self.yellow.y as u64) +
            (SIXTEEN_POWER_6 * self.blue.x as u64) +
            (SIXTEEN_POWER_7 * self.blue.y as u64),
            self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.blue.x as u64) +
            (SIXTEEN_POWER_3 * self.blue.y as u64) +
            (SIXTEEN_POWER_4 * self.green.x as u64) +
            (SIXTEEN_POWER_5 * self.green.y as u64) +
            (SIXTEEN_POWER_6 * self.yellow.x as u64) +
            (SIXTEEN_POWER_7 * self.yellow.y as u64),
            self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.blue.x as u64) +
            (SIXTEEN_POWER_3 * self.blue.y as u64) +
            (SIXTEEN_POWER_4 * self.yellow.x as u64) +
            (SIXTEEN_POWER_5 * self.yellow.y as u64) +
            (SIXTEEN_POWER_6 * self.green.x as u64) +
            (SIXTEEN_POWER_7 * self.green.y as u64),
            self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.yellow.x as u64) +
            (SIXTEEN_POWER_3 * self.yellow.y as u64) +
            (SIXTEEN_POWER_4 * self.blue.x as u64) +
            (SIXTEEN_POWER_5 * self.blue.y as u64) +
            (SIXTEEN_POWER_6 * self.green.x as u64) +
            (SIXTEEN_POWER_7 * self.green.y as u64),
            self.red.x as u64 +
            (SIXTEEN_POWER_1 * self.red.y as u64) +
            (SIXTEEN_POWER_2 * self.yellow.x as u64) +
            (SIXTEEN_POWER_3 * self.yellow.y as u64) +
            (SIXTEEN_POWER_4 * self.green.x as u64) +
            (SIXTEEN_POWER_5 * self.green.y as u64) +
            (SIXTEEN_POWER_6 * self.blue.x as u64) +
            (SIXTEEN_POWER_7 * self.blue.y as u64),
        ];
    }

    pub fn get_robots(&self) -> Vec<Position> {
        return vec![self.red, self.green, self.blue, self.yellow];
    }

    // private
    fn get_tile(&self, position: Position) -> Tile {
        return self.map[position.x as usize][position.y as usize]
    }

    fn set_robot_by_colour(&mut self, colour: Colour, position: Position) {
        match colour {
            Colour::RED => self.red = position,
            Colour::BLUE => self.blue = position,
            Colour::GREEN => self.green = position,
            Colour::YELLOW => self.yellow = position,
        }
    }

    fn is_occupied(&self, position: Position) -> bool {
        return self.red == position || self.blue == position || self.yellow == position || self.green == position
    }

    fn can_move(&self, position: Position, direction: Direction) -> bool {
        let tile = self.get_tile(position);
        let is_blocked_by_wall = match direction {
            Direction::UP => !tile.up,
            Direction::DOWN => !tile.down,
            Direction::LEFT => !tile.left,
            Direction::RIGHT => !tile.right,
        };
        return !is_blocked_by_wall && !self.is_occupied(tile.get_adjacent_position(direction))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EmptyBoard {
    map: [[Tile; 16]; 16],
    goal: Position,
}

impl EmptyBoard {

    // public
    pub fn new(
        goal: Position,
    ) -> EmptyBoard {
        let map = load_map();
        let new_board = EmptyBoard {
            goal: goal,
            map: map,
        };
        return new_board
    }

    pub fn get_valid_directions(&self, position: Position) -> Vec<Direction> {
        let tile = self.get_tile(position);

        let mut directions = Vec::new();
        if tile.up {
            directions.push(Direction::UP);
        }
        if tile.down {
            directions.push(Direction::DOWN);
        }
        if tile.left {
            directions.push(Direction::LEFT);
        }
        if tile.right {
            directions.push(Direction::RIGHT);
        }
        return directions
    }

    pub fn get_tile(&self, position: Position) -> Tile {
        return self.map[position.x as usize][position.y as usize]
    }

    pub fn can_move(&self, position: Position, direction: Direction) -> bool {
        let tile = self.get_tile(position);
        return !match direction {
            Direction::UP => !tile.up,
            Direction::DOWN => !tile.down,
            Direction::LEFT => !tile.left,
            Direction::RIGHT => !tile.right,
        };
    }
}

fn compute_end_position(tile: Tile, direction: Direction, map: [[Tile; 16]; 16]) -> Position {
    let mut moving_tile = tile;
    while can_move(moving_tile, direction) {
        let next_position = moving_tile.get_adjacent_position(direction);
        moving_tile = map[next_position.x as usize][next_position.y as usize];
    }
    return moving_tile.position
}

fn can_move(tile: Tile, direction: Direction) -> bool {
    return !match direction {
        Direction::UP => !tile.up,
        Direction::DOWN => !tile.down,
        Direction::LEFT => !tile.left,
        Direction::RIGHT => !tile.right,
    };
}

fn load_map() -> [[Tile; 16]; 16] {
    let mut map = init_map_array();
    let contents = fs::read_to_string("maps/map1.txt").expect("Error reading map");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut j = 0;
    for line in lines {
        let mut i = 0;
        for tile in line.chars() {
            if ['0', '2', '3', '4', '8', '9', 'A', 'E'].contains(&tile) {
                map[i][j].up = true;
            }
            if ['0', '1', '2', '4', '5', '7', '9', 'C'].contains(&tile) {
                map[i][j].down = true;
            }
            if ['0', '1', '2', '3', '5', '6', '8', 'B'].contains(&tile) {
                map[i][j].left = true;
            }
            if ['0', '1', '3', '4', '6', '7', 'A', 'D'].contains(&tile) {
                map[i][j].right = true;
            }
            i += 1;
        }
        j += 1;
    }
    for j in 0..16 {
        for i in 0..16 {
            map[i][j].up_move = compute_end_position(map[i][j], Direction::UP, map);
            map[i][j].down_move = compute_end_position(map[i][j], Direction::DOWN, map);
            map[i][j].left_move = compute_end_position(map[i][j], Direction::LEFT, map);
            map[i][j].right_move = compute_end_position(map[i][j], Direction::RIGHT, map);
        }
    }
    return map;
}

fn init_map_array() -> [[Tile; 16]; 16] {
    [[Tile {position: Position{x: 0, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 0, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 1, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 1, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 2, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 2, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 3, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 3, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 4, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 4, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 5, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 5, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 6, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 6, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 7, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 7, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 8, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 8, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 9, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 9, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 10, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 10, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 11, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 11, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 12, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 12, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 13, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 13, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 14, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 14, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    [Tile {position: Position{x: 15, y: 0}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 1}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 2}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 3}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 4}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 5}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 6}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 7}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 8}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 9}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 10}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 11}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 12}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 13}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 14}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},
    Tile {position: Position{x: 15, y: 15}, up: false, down: false, left: false, right: false, up_move: Position{x: 0, y: 0}, down_move: Position{x: 0, y: 0}, left_move: Position{x: 0, y: 0}, right_move: Position{x: 0, y: 0}},],
    ]
}