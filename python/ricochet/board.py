from hashlib import sha256


class Board:
    UP = 1
    RIGHT = 2
    DOWN = 3
    LEFT = 4

    def __init__(self, robots, goal, map=None):
        if map is None:
            self._load_map()
        else:
            self.map = map
        self._place_robots(robots)
        self._set_goal(goal)

    def _set_goal(self, goal):
        self.goal = goal
        x, y = goal
        self.map[x][y].goal = True

    def _load_map(self):
        self.map = [[Tile((i, j)) for j in range(16)] for i in range(16)]
        with open("maps/map1.txt") as f:
            lines = f.readlines()
        for j, line in enumerate(lines):
            for i, tile_number in enumerate(line):
                if tile_number in ["0", "2", "3", "4", "8", "9", "A", "E"]:
                    self.map[i][j].up = self.map[i][j - 1]
                if tile_number in ["0", "1", "2", "4", "5", "7", "9", "C"]:
                    self.map[i][j].down = self.map[i][j + 1]
                if tile_number in ["0", "1", "2", "3", "5", "6", "8", "B"]:
                    self.map[i][j].left = self.map[i - 1][j]
                if tile_number in ["0", "1", "3", "4", "6", "7", "A", "D"]:
                    self.map[i][j].right = self.map[i + 1][j]

    def _place_robots(self, robots):
        self.robots = {}
        for colour, position in robots.items():
            x, y = position
            self.robots[colour] = Robot(self.map[x][y], colour, position)

    def _is_occupied(self, tile):
        return len([robot for robot in self.robots.values() if robot.position == tile.position]) > 0

    def get_robot_by_colour(self, colour):
        return self.robots[colour]

    def get_end_position(self, start_position, direction):
        tile = self.map[start_position[0]][start_position[1]]
        while tile.get_adjacent_tile(direction) is not None and not self._is_occupied(tile.get_adjacent_tile(direction)):
            tile = tile.get_adjacent_tile(direction)
        return tile.position

    def move_robot(self, direction, colour):
        robot = self.get_robot_by_colour(colour)
        while robot.tile.get_adjacent_tile(direction) is not None and not self._is_occupied(robot.tile.get_adjacent_tile(direction)):
            previous_tile = robot.tile
            robot.set_tile(previous_tile.get_adjacent_tile(direction))

    def can_move(self, start_position, direction):
        tile = self.map[start_position[0]][start_position[1]]
        if tile.get_adjacent_tile(direction) is not None and not self._is_occupied(tile.get_adjacent_tile(direction)):
            return True
        return False

    def get_valid_directions(self, tile):
        valid_directions = []
        if tile.up is not None and not self._is_occupied(tile.up):
            valid_directions.append(Board.UP)
        if tile.down is not None and not self._is_occupied(tile.down):
            valid_directions.append(Board.DOWN)
        if tile.left is not None and not self._is_occupied(tile.left):
            valid_directions.append(Board.LEFT)
        if tile.right is not None and not self._is_occupied(tile.right):
            valid_directions.append(Board.RIGHT)
        return valid_directions

    def get_valid_reverse_directions(self, tile):
        valid_directions = []
        if tile.up is not None and not self._is_occupied(tile.up) and (tile.down is None or self._is_occupied(tile.down)):
            valid_directions.append(Board.UP)
        if tile.down is not None and not self._is_occupied(tile.down) and (tile.up is None or self._is_occupied(tile.up)):
            valid_directions.append(Board.DOWN)
        if tile.left is not None and not self._is_occupied(tile.left) and (tile.right is None or self._is_occupied(tile.right)):
            valid_directions.append(Board.LEFT)
        if tile.right is not None and not self._is_occupied(tile.right) and (tile.left is None or self._is_occupied(tile.left)):
            valid_directions.append(Board.RIGHT)
        return valid_directions

    def is_solved(self, colour):
        return self.get_robot_by_colour(colour).position == self.goal

    def to_string(self):
        green_position = self.robots['green'].position
        red_position = self.robots['red'].position
        blue_position = self.robots['blue'].position
        yellow_position = self.robots['yellow'].position
        black_position = self.robots['black'].position
        return f"{green_position},{red_position},{blue_position},{yellow_position},{black_position}"


    def hash(self):
        return sha256(self.to_string().encode('utf-8')).hexdigest()

    def clone(self):
        cloned_robots = {
            "red": self.robots["red"].position,
            "green": self.robots["green"].position,
            "blue": self.robots["blue"].position,
            "yellow": self.robots["yellow"].position,
            "black": self.robots["black"].position,
        }
        new_board = Board(
            map=self.map,
            goal=self.goal,
            robots=cloned_robots,
        )
        new_board.goal = self.goal
        return new_board

class Robot:
    def __init__(self, tile, colour, position):
        self.tile = tile
        self.colour = colour
        self.position = position

    def set_tile(self, tile):
        self.tile = tile
        self.position = tile.position

    def is_at_goal(self):
        return self.tile.goal

    def clone(self):
        return Robot(self.tile, self.colour, self.position)

class Tile:
    def __init__(self, position):
        self.up = None
        self.down = None
        self.left = None
        self.right = None
        self.goal = False
        self.position = position

    def get_adjacent_tile(self, direction):
        if direction == Board.UP:
            return self.up
        elif direction == Board.DOWN:
            return self.down
        elif direction == Board.LEFT:
            return self.left
        elif direction == Board.RIGHT:
            return self.right
