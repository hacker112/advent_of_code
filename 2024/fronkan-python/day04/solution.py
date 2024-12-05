from argparse import ArgumentParser
from pathlib import Path


def puzzle1(input_file: Path):
    WORD = list("XMAS")
    REV_WORD = list("SAMX")
    WORD_LEN = 4
    data = input_file.read_text()
    puzzle = [
        [c for c in line]
        for line in data.splitlines()
    ]
    height = len(puzzle)
    width = len(puzzle[0])
    total = 0

    # puzzle_searched = [
    #     ["." for _ in line]
    #     for line in data.splitlines()
    # ]

    # horizontal
    for _,row in enumerate(puzzle):
        for i in range(width-WORD_LEN+1):
            tokens = row[i:i+WORD_LEN]
            if WORD == tokens:
                total += 1
            if REV_WORD == tokens:
                total += 1

            # for t in range(WORD_LEN):
            #     puzzle_searched[_][i+t] = "h"
            # _print_puzzle(puzzle_searched)
        
    # vertical
    for y in range(height-WORD_LEN+1):
        for x in range(width):
            tokens = [
                puzzle[y+i][x]
                for i in range(WORD_LEN)
            ]
            if WORD == tokens:
                total += 1
            if REV_WORD == tokens:
                total += 1

            # for i in range(WORD_LEN):
            #     puzzle_searched[y+i][x] = "v"
            # _print_puzzle(puzzle_searched)

    for y in range(height-WORD_LEN+1):
        for x in range(width-WORD_LEN+1):
            tokens = [
                puzzle[y+i][x+i]
                for i in range(WORD_LEN)
            ]
            if WORD == tokens:
                total += 1
            if REV_WORD == tokens:
                total += 1

            # for i in range(WORD_LEN):
            #     puzzle_searched[y+i][x+i] = "d"
            # _print_puzzle(puzzle_searched)
    
    for y in range(WORD_LEN-1, height):
        for x in range(width-WORD_LEN+1):
            tokens = [
                puzzle[y-i][x+i]
                for i in range(WORD_LEN)
            ]
            if WORD == tokens:
                total += 1
            if REV_WORD == tokens:
                total += 1

            # for i in range(WORD_LEN):
            #     puzzle_searched[y-i][x+i] = "*"
            # _print_puzzle(puzzle_searched)

    
    return total

def _print_puzzle(puzzle):
    print("\n".join("".join(row) for row in puzzle), end="\n\n")


def puzzle2(input_file: Path):
    WORD = list("MAS")
    REV_WORD = list("SAM")
    data = input_file.read_text()
    puzzle = [
        [c for c in line]
        for line in data.splitlines()
    ]
    height = len(puzzle)
    width = len(puzzle[0])
    total = 0

    # puzzle_searched = [
    #     ["." for _ in line]
    #     for line in data.splitlines()
    # ]
    
    # use cross center as the target
    for y in range(1, height-1):
        for x in range(1, width-1):

            # ..M
            # .A.
            # S..
            tokens_forward_slash = [
                puzzle[y-1][x+1], # M
                puzzle[y][x], # A
                puzzle[y+1][x-1], # S
            ]
            word_forward_slash_found = tokens_forward_slash == WORD or tokens_forward_slash == REV_WORD

            # M..
            # .A.
            # ..S            
            tokens_back_slash = [
                puzzle[y-1][x-1], # M
                puzzle[y][x], # A
                puzzle[y+1][x+1], # S
            ]

            word_back_slash_found = tokens_back_slash == WORD or tokens_back_slash == REV_WORD

            if word_forward_slash_found and word_back_slash_found:
                total += 1
                # puzzle_searched[y-1][x-1] = puzzle[y-1][x-1]
                # puzzle_searched[y-1][x+1] = puzzle[y-1][x+1]
                # puzzle_searched[y][x] = puzzle[y][x]
                # puzzle_searched[y+1][x-1] = puzzle[y+1][x-1]
                # puzzle_searched[y+1][x+1] = puzzle[y+1][x+1]
                # # _print_puzzle(puzzle_searched)
                # puzzle_searched[y-1][x-1] = "."
                # puzzle_searched[y-1][x+1] = "."
                # puzzle_searched[y][x] = "."
                # puzzle_searched[y+1][x-1] = "."
                # puzzle_searched[y+1][x+1] = "."
    return total


if __name__ == "__main__":
    print("Day 4")

    parser = ArgumentParser()
    parser.add_argument("--example", action="store_true", default=False)
    args = parser.parse_args()

    if args.example:
        print("running example file")
        input_file = Path(__file__).parent / "example_input.txt"
    else:
        input_file = Path(__file__).parent / "input.txt"
    try:
        print("Puzzle 1:", puzzle1(input_file))
    except NotImplementedError:
        print("puzzle 1 not implemented yet")
    try:
        print("Puzzle 2:", puzzle2(input_file))
    except NotImplementedError:
        print("puzzle 2 not implemented yet")