mod board;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time::SystemTime;

use itertools::Itertools;
use rand::Rng;
use hashbrown::HashMap;
use thincollections::thin_vec::ThinVec;

use board::Board;
use board::Colour;
use board::Direction;
use board::EmptyBoard;
use board::Position;


const FOUR: u64 = 4;

struct PathOption {
    priority: i16,
    distance: i16,
    board: Board,
    directions: u64,
    colours: u64,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    colour: Colour,
}

struct Solution {
    length: i16,
    path: PathOption,
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

fn to_int (direction: Direction) -> u64 {
    match direction {
        Direction::UP => 0,
        Direction::RIGHT => 1,
        Direction::DOWN => 2,
        Direction::LEFT => 3,
    }
}

fn to_direction(num: u64) -> Direction {
    match num {
        0 => Direction::UP,
        1 => Direction::RIGHT,
        2 => Direction::DOWN,
        3 => Direction::LEFT,
        _ => panic!("dumb bumb"),
    }
}

fn colour_to_int (colour: Colour) -> u64 {
    match colour {
        Colour::RED => 0,
        Colour::GREEN => 1,
        Colour::BLUE => 2,
        Colour::YELLOW => 3,
    }
}

fn to_colour(num: u64) -> Colour {
    match num {
        0 => Colour::RED,
        1 => Colour::GREEN,
        2 => Colour::BLUE,
        3 => Colour::YELLOW,
        _ => panic!("dumb bumb"),
    }
}

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

// #[inline(never)]
// fn get_neighbours(option: PathOption, flooded_map: &[[i8; 16]; 16], target_colour: Colour, known_solutions: &HashMap<u64, i16>) -> ThinVec<PathOption> {
//     let mut neighbours = ThinVec::new();
//     for colour in vec![Colour::RED, Colour::BLUE, Colour::YELLOW, Colour::GREEN] {
//         for direction in option.board.get_valid_directions(colour) {
//             let mut neighbour_board = option.board.clone();
//             let new_position = neighbour_board.move_robot(colour, direction);
//             let mut heuristic = flooded_map[new_position.x as usize][new_position.y as usize] as i16;

//             match known_solutions.get(&neighbour_board.hash()) {
//                 Some(distance) => {
//                     heuristic = *distance;
//                 },
//                 None => {},
//             }

//             let neighbour = PathOption{
//                 priority: option.distance + heuristic + 1,
//                 distance: option.distance + 1,
//                 board: neighbour_board,
//                 directions: option.directions + FOUR.pow(option.distance as u32 + 1) * to_int(direction),
//                 colours: option.colours + FOUR.pow(option.distance as u32 + 1) * colour_to_int(colour),
//             };
//             neighbours.push(neighbour);
//         }
//     }
//     println!("Length: {}", neighbours.len());
//     return neighbours
// }

#[inline(never)]
fn solve(board: Board, target_colour: Colour, known_solutions: &HashMap<u64, i16>, flooded_map: &[[i8; 16]; 16]) -> Solution {
    let mut heap = BinaryHeap::new();
    let mut visited_board_states = HashMap::new();
    heap.push(Reverse(PathOption{
        priority: 0,
        distance: 0,
        board: board,
        directions: 0,
        colours: 0,
    }));

    loop {
        let some_option = heap.pop();
        match some_option {
            Some(Reverse(option)) => {
                if option.board.is_solved(target_colour) {
                    return Solution {
                        length: option.distance,
                        path: option,
                    }
                }

                for colour in vec![Colour::RED, Colour::BLUE, Colour::YELLOW, Colour::GREEN] {
                    for direction in option.board.get_valid_directions(colour) {
                        let mut neighbour_board = option.board.clone();
                        let new_position = neighbour_board.move_robot(colour, direction);
                        let mut heuristic = flooded_map[new_position.x as usize][new_position.y as usize] as i16;

                        match known_solutions.get(&neighbour_board.hash()) {
                            Some(distance) => {
                                heuristic = *distance;
                            },
                            None => {},
                        }

                        let hashed_board = neighbour_board.hash();
                        let mut push_state = false;
                        match visited_board_states.get(&hashed_board) {
                            Some(distance) => {
                                if *distance > option.distance + 1 {
                                    push_state = true;
                                }
                            },
                            None => {
                                push_state = true;
                            },
                        }
                        if push_state {
                            visited_board_states.insert(hashed_board, option.distance + 1);
                            heap.push(Reverse(
                                PathOption{
                                    priority: option.distance + heuristic + 1,
                                    distance: option.distance + 1,
                                    board: neighbour_board,
                                    directions: option.directions + FOUR.pow(option.distance as u32 + 1) * to_int(direction),
                                    colours: option.colours + FOUR.pow(option.distance as u32 + 1) * colour_to_int(colour),
                                }
                            ));
                        }
                    }
                }
            },
            None => continue,
        }
    }
}

fn main() {
    // let mut known_solutions: HashMap<u64, i16> = HashMap::new();
    // let mut board = Board::new(
    //     Position { x: 0, y: 0 }, Position { x: 2, y: 0 }, Position { x: 3, y: 0 }, Position { x: 4, y: 0 },
    //     Position{x: 6, y: 14},
    // );
    // let flooded_board = flood_fill(Position{x: 6, y: 14});

    // let now = SystemTime::now();
    // let result = solve(board.clone(), Colour::RED, &known_solutions, &flooded_board);
    // println!("result: {}", result.length);
    // match now.elapsed() {
    //     Ok(elapsed) => {
    //         println!("Time: {:?}", elapsed);
    //     },
    //     Err(_) => println!("It broke somehow"),
    // }

    

    let unsolveable_positions = vec![
        Position{x: 7, y: 7},
        Position{x: 7, y: 8},
        Position{x: 8, y: 7},
        Position{x: 8, y: 8},
    ];
    let mut board_positions = vec![];
    for y in 0..16 {
        for x in 0..16 {
            let position = Position{x: x, y: y};
            if !unsolveable_positions.contains(&position) {
                board_positions.push(position);
            }
        }
    }
    let goal = Position{x: 6, y: 14};
    let mut known_solutions: HashMap<u64, i16> = HashMap::new();
    let mut longest_solution = 0;
    let mut longest_solution_start: Vec<Position> = Vec::new();
    let mut count: u64 = 0; 

    let start = SystemTime::now();
    let flooded_board = flood_fill(goal);
    for robots in board_positions.into_iter().combinations(4) {
        count += 1;
        let mut board = Board::new(
            robots[0],
            robots[1],
            robots[2],
            robots[3],
            goal,
        );

        if known_solutions.contains_key(&board.hash()) {
            continue;
        }

        let result = solve(board.clone(), Colour::RED, &known_solutions, &flooded_board);

        let mut idx = result.length;
        let mut direction_path = result.path.directions;
        let mut colour_path = result.path.colours;

        for hash in &board.permuted_hashes() {
            known_solutions.insert(*hash, idx);
        }
        while direction_path > 0 {
            let factor = FOUR.pow(idx as u32);
            let direction_num = direction_path / factor;
            direction_path = direction_path - (direction_num * factor);
            let direction = to_direction(direction_num);

            let colour_num = colour_path / factor;
            colour_path = colour_path - (colour_num * factor);
            let colour = to_colour(colour_num);
            idx -= 1;

            board.move_robot(colour, direction);
            for hash in &board.permuted_hashes() {
                known_solutions.insert(*hash, idx);
            }
        }

        if result.length > longest_solution {
            println!("New longest: {}", result.length);
            longest_solution = result.length;
            longest_solution_start = robots.clone();
        }
        if count % 10000 == 0 {
            match start.elapsed() {
                Ok(elapsed) => {
                    println!("Time: {:?}", elapsed);
                    println!("Average {}ms", elapsed.as_millis() as f64 / count as f64);
                },
                Err(_) => println!("It broke somehow"),
            }
            println!("Checked {} boards", count);
        }
    }
    println!("Final longest: {}, {:?}", longest_solution, longest_solution_start);




    // let unsolveable_positions = vec![
    //     Position{x: 7, y: 7},
    //     Position{x: 7, y: 8},
    //     Position{x: 8, y: 7},
    //     Position{x: 8, y: 8},
    // ];
    // let mut total: u128 = 0;
    // let mut count: u128 = 0;
    // let mut total_solve_length: u128 = 0;
    // let mut known_solutions: HashMap<u64, i16> = HashMap::new();
    // let flooded_board = flood_fill(Position{x: 6, y: 14});
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
    //         Position{x: 6, y: 14},
    //     );
    //     let now = SystemTime::now();
    //     let result = solve(board, Colour::RED, &known_solutions, &flooded_board);
    //     match now.elapsed() {
    //         Ok(elapsed) => {
    //             println!("{}, {:?}", result.length, elapsed.as_millis());
    //             total += elapsed.as_millis();
    //             total_solve_length += result.length as u128;
    //             count += 1;
    //         },
    //         Err(_) => println!("It broke somehow"),
    //     }
    // }
    // println!("Average solve time: {}", total/count);
    // println!("Average solve length: {}", total_solve_length/count);
}
