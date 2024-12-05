from advent_of_code.utils.input_parser import parse_integers, parse_str_lines, parse_grid

def check_level(row):
    safe_row = True
    increasing = None
    for ix, item in enumerate(row):
        if ix < (len(row) - 1):
            if item < row[ix + 1]:
                if increasing is None or increasing:
                    increasing = True
                else:
                    safe_row = False
                    break
            if item > row[ix + 1]:
                if increasing is None or not increasing:
                    increasing = False
                else:
                    safe_row = False
                    break
            diff = abs(item - row[ix + 1])
            if 1 <= diff <= 3:
                pass
            else:
                safe_row = False
                break
    return safe_row

def solve(levels):
    sum1 = 0
    sum2 = 0
    for ix, level in enumerate(levels):
        new_level = list(level)
        if (check_level(new_level)):
            sum1 += 1
        else:
            for ix, item in enumerate(level):
                new_level.pop(ix)
                if (check_level(new_level)):
                    sum2 += 1
                    break
                else:
                    new_level = list(level)
    return sum1, sum2

def part1(input_data: str) -> int:
    """
    Solve Part 1 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 1
    """

    levels = parse_integers(input_data)
    sum = solve(levels)
    return sum[0]


def part2(input_data: str) -> int:
    """
    Solve Part 2 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 2
    """
    levels = parse_integers(input_data)
    sum = solve(levels)
    return sum[0] + sum[1]