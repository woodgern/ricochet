mod board;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time::SystemTime;

use rand::Rng;

use board::Board;
use board::Colour;
use board::EmptyBoard;
use board::Position;


struct PathOption {
    priority: i16,
    distance: i16,
    board: Board,
}

impl Ord for PathOption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PathOption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathOption {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PathOption {}

fn _flood_fill(current_position: Position, mut map: [[i8; 16]; 16], board: EmptyBoard, mut count: i8) -> [[i8; 16]; 16] {
    if count >= map[current_position.x as usize][current_position.y as usize] {
        return map;
    }
    map[current_position.x as usize][current_position.y as usize] = count;
    count += 1;

    for direction in board.get_valid_directions(current_position) {
        let mut position = current_position.clone();
        while board.can_move(position, direction) {
            position = board.get_tile(position).get_adjacent_position(direction);
            map = _flood_fill(position.clone(), map, board, count);
        }
    }
    return map
}

fn flood_fill(goal: Position) -> [[i8; 16]; 16] {
    let map = [[std::i8::MAX; 16]; 16];
    let board = EmptyBoard::new(goal.clone());
    return _flood_fill(goal, map, board, 0);
}

fn get_neighbours(option: PathOption, flooded_map: [[i8; 16]; 16]) -> Vec<PathOption> {
    let mut neighbours = Vec::new();
    for colour in vec![Colour::RED, Colour::BLUE, Colour::YELLOW, Colour::GREEN, Colour::BLACK] {
        for direction in option.board.get_valid_directions(colour) {
            let mut neighbour_board = option.board.clone();
            neighbour_board.move_robot(colour, direction);
            let new_position = neighbour_board.get_robot_by_colour(colour);

            let heuristic = flooded_map[new_position.x as usize][new_position.y as usize];
            let neighbour = PathOption{
                priority: option.distance + heuristic as i16,
                distance: option.distance + 1,
                board: neighbour_board,
            };
            neighbours.push(neighbour);
        }
    }
    return neighbours
}

fn solve(board: Board, target_colour: Colour) -> i16 {
    let mut heap = BinaryHeap::new();
    let mut visited_board_states = HashSet::new();
    let flooded_map = flood_fill(board.get_goal());
    heap.push(Reverse(PathOption{
        priority: 0,
        distance: 0,
        board: board,
    }));

    loop {
        let some_option = heap.pop();
        match some_option {
            Some(Reverse(option)) => {
                if option.board.is_solved(target_colour) {
                    return option.distance;
                }

                let neighbours = get_neighbours(option, flooded_map);
                for neighbour in neighbours {
                    let hashed_board = neighbour.board.hash();
                    if !visited_board_states.contains(&hashed_board) {
                        visited_board_states.insert(hashed_board);
                        heap.push(Reverse(neighbour));
                    }
                }
            },
            None => return std::i16::MAX,
        }
    }
}

fn main() {

    // 13 moves
    let red = Position{x: 1, y: 6};
    let green = Position{x: 0, y: 3};
    let blue = Position{x: 1, y: 7};
    let yellow = Position{x: 1, y: 8};
    let black = Position{x: 1, y: 9};

    let goal = Position{x: 6, y: 14};

    let board = Board::new(
        red,
        green,
        blue,
        yellow,
        black,
        goal,
    );

    let now = SystemTime::now();
    let result = solve(board, Colour::GREEN);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}, {:?}", result, elapsed.as_millis());
        },
        Err(_) => println!("It broke somehow"),
    }


    // let unsolveable_positions = vec![
    //     Position{x: 7, y: 7},
    //     Position{x: 7, y: 8},
    //     Position{x: 8, y: 7},
    //     Position{x: 8, y: 8},
    // ];
    // let mut total: u128 = 0;
    // let mut count: u128 = 0;
    // let mut total_solve_length: u128 = 0;
    // for _ in 0..1000 {
    //     let mut rng = rand::thread_rng();
    //     let pos1 = Position{x: rng.gen_range(0, 16), y: rng.gen_range(0, 16)};
    //     let pos2 = Position{x: rng.gen_range(0, 16), y: rng.gen_range(0, 16)};
    //     let pos3 = Position{x: rng.gen_range(0, 16), y: rng.gen_range(0, 16)};
    //     let pos4 = Position{x: rng.gen_range(0, 16), y: rng.gen_range(0, 16)};
    //     let pos5 = Position{x: rng.gen_range(0, 16), y: rng.gen_range(0, 16)};
    //     let mut robot_positions = HashSet::new();
    //     robot_positions.insert(pos1);
    //     robot_positions.insert(pos2);
    //     robot_positions.insert(pos3);
    //     robot_positions.insert(pos4);
    //     robot_positions.insert(pos5);
    //     if robot_positions.len() != 5 || unsolveable_positions.contains(&pos1) || unsolveable_positions.contains(&pos2) || unsolveable_positions.contains(&pos3) || unsolveable_positions.contains(&pos4) || unsolveable_positions.contains(&pos5) {
    //         continue;
    //     }
    //     let board = Board::new(
    //         pos1,
    //         pos2,
    //         pos3,
    //         pos4,
    //         pos5,
    //         Position{x: 6, y: 14},
    //     );
    //     let now = SystemTime::now();
    //     let result = solve(board, Colour::GREEN);
    //     match now.elapsed() {
    //         Ok(elapsed) => {
    //             println!("{}, {:?}", result, elapsed.as_millis());
    //             total += elapsed.as_millis();
    //             total_solve_length += result as u128;
    //             count += 1;
    //         },
    //         Err(_) => println!("It broke somehow"),
    //     }
    // }
    // println!("Average solve time: {}", total/count);
    // println!("Average solve length: {}", total_solve_length/count);
}
