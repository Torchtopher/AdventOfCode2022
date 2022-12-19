import numpy as np
# enums library
from enum import Enum

# define the different types of pieces
class Piece(Enum):
    horizontal = 1
    vetical = 2
    inv_L = 3
    square = 4
    cross = 5

class Tetris:
    def __init__(self) -> None:
        self.board = np.zeros((4*2022, 7 ))
        self.current_piece = None
        self.order = [Piece.horizontal, Piece.cross, Piece.inv_L, Piece.vetical, Piece.square]
        self.current_piece_index = 0
    
    def find_lowest_empty_row(self):
        nonzero_rows, nonzero_cols = np.where(self.board != 0)  # find non-zero elements
        max_nonzero_row = np.amax(nonzero_rows)  # find maximum row index among non-zero elements
        lowest_empty_row = self.board.shape[0] - 1 - max_nonzero_row  # subtract from total number of rows
        lowest_empty_row = 4*2022 - lowest_empty_row # need to invert
        return lowest_empty_row

    def spawn_piece(self):
        self.current_piece = self.order[self.current_piece_index]

        print(self.board)
        y = self.find_lowest_empty_row() + 2 
        x = 2
        # Looks like ####
        if self.current_piece == Piece.horizontal:
            self.board[y][x] = 1
            self.board[y][x+1] = 1
            self.board[y][x+2] = 1
            self.board[y][x+3] = 1

        # looks like #
                     #
                     #
                     #
        elif self.current_piece == Piece.vetical:
            self.board[y][x] = 1
            self.board[y+1][x] = 1
            self.board[y+2][x] = 1
            self.board[y+3][x] = 1

        # looks like #
                     #
                   ###
        elif self.current_piece == Piece.inv_L:
            

        # looks like ##
                     ##
        elif self.current_piece == Piece.square:
            pass
        # looks like #
                    ###
                     #
        elif self.current_piece == Piece.cross:
            pass

        self.current_piece_index += 1
        self.current_piece_index %= len(self.order)

board = Tetris()
board.spawn_piece()