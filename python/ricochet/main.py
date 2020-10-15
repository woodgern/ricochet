from dataclasses import dataclass, field
from datetime import datetime
from queue import PriorityQueue
from random import randint
from sys import maxsize
from typing import Any

from board import Board


@dataclass(order=True)
class PathOption:
    priority: int
    distance: Any=field(compare=False)
    board: Any=field(compare=False)
    path: Any=field(compare=False)
    def __init__(self, priority, distance, board, path):
        self.priority = priority
        self.distance = distance
        self.board = board
        self.path = path


def flood_fill_reachable_states(map):
    board = Board(
        robots={},
        goal=(6, 14),
    )
    _flood_fill_reachable_states(board, map, (0, 0))

def _flood_fill_reachable_states(board, map, position):
    if map[position[0]][position[1]] == '.':
        return

    map[position[0]][position[1]] = '.'

    for direction in board.get_valid_directions(board.map[position[0]][position[1]]):
        next_position = board.get_end_position(position, direction)
        _flood_fill_reachable_states(board, map, next_position)

def _flood_fill(board, map, position, count):
    if count >= map[position[0]][position[1]]:
        return

    map[position[0]][position[1]] = count

    count += 1

    for direction in board.get_valid_directions(board.map[position[0]][position[1]]):
        cur_position = position
        while board.can_move(cur_position, direction):
            cur_position = board.map[cur_position[0]][cur_position[1]].get_adjacent_tile(direction).position
            if count < map[cur_position[0]][cur_position[1]]:
                _flood_fill(board, map, cur_position, count)


def flood_fill(map):
    board = Board(
        robots={},
        goal=(6, 14),
    )
    _flood_fill(board, map, board.goal, 0)

def _flood_fill(board, map, position, count):
    if count >= map[position[0]][position[1]]:
        return

    map[position[0]][position[1]] = count

    count += 1

    for direction in board.get_valid_directions(board.map[position[0]][position[1]]):
        cur_position = position
        distance_travelled = 0
        while board.can_move(cur_position, direction):
            cur_position = board.map[cur_position[0]][cur_position[1]].get_adjacent_tile(direction).position
            if count < map[cur_position[0]][cur_position[1]]:
                _flood_fill(board, map, cur_position, count)


def _calculate_minimum_distance(board, position, flooded_map):
    x, y = position
    return flooded_map[x][y]

def get_neighbours(option, track_path, flooded_map):
    neighbours = []
    for robot in option.board.robots.values():
        for direction in option.board.get_valid_directions(robot.tile):
            new_board = option.board.clone()
            new_board.move_robot(direction, robot.colour)
            new_position = new_board.get_robot_by_colour(robot.colour).position
            minimum_distance = _calculate_minimum_distance(board, new_position, flooded_map)

            path = []
            if track_path:
                path = option.path.copy()
                path.append((robot.colour, direction))
            neighbours.append(PathOption(option.distance + minimum_distance, option.distance + 1, new_board, path))
    return neighbours

def solve(target_colour, board, track_path=True):
    flooded_map = [[maxsize for _ in range(16)] for _ in range(16)]
    flood_fill(flooded_map)

    move_queue = PriorityQueue()
    move_queue.put(PathOption(0, 0, board, []))
    checked_states = set()
    while True:
        option = move_queue.get()

        if option.board.is_solved(target_colour):
            return (option.distance, option.path)

        neighbours = get_neighbours(option, track_path, flooded_map)
        for neighbour in neighbours:
            hashed_state = neighbour.board.hash()
            if not hashed_state in checked_states:
                move_queue.put(neighbour)
                checked_states.add(hashed_state)

# count = 0
# map = [['x' for _ in range(16)] for _ in range(16)]
# flood_fill_reachable_states(map)
# for y in range(16):
#     for x in range(16):
#         if map[x][y] == '.':
#             count += 1
#         print(map[x][y], end='')
#     print()
# print(count)

# middle_tiles = {(7, 7), (7, 8), (8, 7), (8, 8)}
# checked_boards = set()
# most_moves = 0
# most_moves_set = {}
# for red_x in range(16):
#     for red_y in range(16):
#         for green_x in range(16):
#             for green_y in range(16):
#                 for blue_x in range(16):
#                     for blue_y in range(16):
#                         for yellow_x in range(16):
#                             for yellow_y in range(16):
#                                 for black_x in range(16):
#                                     for black_y in range(16):
#                                         robot_set = frozenset({
#                                             (red_x, red_y),
#                                             (green_x, green_y),
#                                             (blue_x, blue_y),
#                                             (yellow_x, yellow_y),
#                                             (black_x, black_y),
#                                         })
#                                         if len(robot_set) != 5 or len(middle_tiles.intersection(robot_set)) != 0:
#                                             continue
#                                         board_other_robot_set = frozenset({
#                                             (red_x, red_y),
#                                             (blue_x, blue_y),
#                                             (yellow_x, yellow_y),
#                                             (black_x, black_y),
#                                         })
#                                         board_placements = ((green_x, green_y), board_other_robot_set)
#                                         board_placements_hash = hash(board_placements)
#                                         if board_placements_hash not in checked_boards:
#                                             print(len(checked_boards))
#                                             print(board_placements)
#                                             board = Board(
#                                                 robots={
#                                                     "red": (red_x, red_y),
#                                                     "green": (green_x, green_y),
#                                                     "blue": (blue_x, blue_y),
#                                                     "yellow": (yellow_x, yellow_y),
#                                                     "black": (black_x, black_y),
#                                                 },
#                                                 goal=(6, 14),
#                                             )
#                                             start = datetime.now()
#                                             moves = solve("green", board, track_path=False)
#                                             print(datetime.now() - start)
#                                             print(f"moves: {moves}")
#                                             if moves[0] > most_moves:
#                                                 most_moves = moves[0]
#                                                 most_moves_set = board_placements
#                                             checked_boards.add(board_placements_hash)

# print(f"FINAL: {most_moves} moves from state {most_moves_set}")
# print(len(checked_boards))

red = (1, 6)
green = (0, 3)
blue = (1, 7)
yellow = (1, 8)
black = (1, 9)

goal = (6, 14)

board = Board(
    robots={
        "red": red,
        "green": green,
        "blue": blue,
        "yellow": yellow,
        "black": black,
    },
    goal=goal,
)
start = datetime.now()
moves = solve("green", board, track_path=False)
print(f"Moves: {moves[0]}, Time: {datetime.now()-start}")
