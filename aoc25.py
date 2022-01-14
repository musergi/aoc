def update_board(dims, right, down):
    right_mov = {r: new_pos(dims, r, (1, 0)) for r in right}
    right_mov = dict(filter(lambda p: p not in (right | down), right_mov))
    new_right = (right - right_mov.keys()) | set(right_mov.values())
    down_mov = {d: new_pos(dims, d, (0, 1)) for d in down}
    down_mov = dict(filter(lambda p: p not in (new_right | down), down_mov))
    new_down = (down - down_mov.keys()) | set(down_mov.values())
    return dims, new_right, new_down


def diff(board1, board2):
    return board1[1] != board2[1] or board1[2] != board2[2]


class Board:
    def __init__(self, dims, right, down):
        self.row_count, self.col_count = dims
        self.right = right
        self.down = down

    def moved_right(self, position):
        row, col = position
        return row, (col + 1) % self.col_count

    def moved_down(self, position):
        row, col = position
        return (row + 1) % self.row_count, col

    @staticmethod
    def from_file(filepath):
        right, down= set(), set()
        with open(filepath) as fp:
            for row, line in enumerate(fp):
                for col, char in enumerate(line):
                    if char == 'v':
                        down.add((row, col))
                    elif char == '>':
                        right.add((row, col))
        return Board((row + 1, col), right, down)

    def dims(self):
        return self.row_count, self.col_count

    def update(self):
        move = {}
        for position in self.right:
            destination = self.moved_right(position)
            if destination not in self.right and destination not in self.down:
                move[position] = destination
        new_right = (self.right - move.keys()) | set(move.values())
        
        move = {}
        for position in self.down:
            destination = self.moved_down(position)
            if destination not in new_right and destination not in self.down:
                move[position] = destination
        new_down = (self.down - move.keys()) | set(move.values())

        return Board(self.dims(), new_right, new_down)

    def __eq__(self, other):
        return self.right == other.right and self.down == other.down

    def __repr__(self):
        return f'Board({self.dims()}, {len(self.right)}, {len(self.down)})'

    def __str__(self):
        s = ''
        for row in range(self.row_count):
            for col in range(self.col_count):
                if (row, col) in self.right:
                    s += '>'
                elif (row, col) in self.down:
                    s += 'v'
                else:
                    s += '.'
            s += '\n'
        return s
                

def main():
    board = Board.from_file('input25.txt')
    previous = None
    i = 0
    while previous is None or board != previous:
        previous = board
        board = board.update()
        i += 1
    print(i)


if __name__ == '__main__':
    main()
