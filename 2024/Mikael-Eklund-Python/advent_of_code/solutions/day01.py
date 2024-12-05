from advent_of_code.utils.input_parser import parse_integers, parse_str_lines, parse_grid

def part1(input_data: str) -> int:
    """
    Solve Part 1 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 1
    """
    sum = 0
    dist = 0
    lstr = []
    rstr = []
    strings = input_data
    #strings = parse_str_lines(input_data)
    for string in strings:
        string = string.strip().split(' ')
        lstr.append(int(string[0]))
        rstr.append(int(string[-1]))
        lstr.sort()
        rstr.sort()
    for item, ix in enumerate(lstr):
        dist = rstr[item] - lstr[item]
        sum += abs(dist)
    return sum

def part2(input_data: str) -> int:
    """ 
    Solve Part 2 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 2
    """
    sum = 0
    dist = 0
    lstr = []
    rstr = []
    strings = input_data
    #strings = parse_str_lines(input_data)
    for string in strings:
        string = string.strip().split(' ')
        lstr.append(int(string[0]))
        rstr.append(int(string[-1]))
    for ix, item in enumerate(lstr):
        dist = (rstr.count(item)) * item
        sum += abs(dist)
    return sum