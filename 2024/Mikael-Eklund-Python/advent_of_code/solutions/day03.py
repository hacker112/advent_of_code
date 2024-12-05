from advent_of_code.utils.input_parser import parse_integers, parse_str_lines, parse_grid

def part1(input_data: str) -> int:
    """
    Solve Part 1 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 1
    """
    pos = -1
    sum = 0
    line1 = ""
    line = ""
    for line1 in input_data:
        line += line1    
    while True:
        pos = line.lower().find("mul(", pos + 1)
        if pos == -1:
            break
        start_pos = pos + 4
        end_pos = line.find(")", start_pos)
        if end_pos != -1:
            content = line[start_pos:end_pos]
            values = content.split(",")
            if len(values) == 2:
                if not ' ' in values[0] and not ' ' in values[1]:
                    if values[0].strip().isdigit() and values[1].strip().isdigit():
                        if int(values[0]) < 1000 and int(values[1]) < 1000:
                            sum = sum + int(values[0]) * int(values[1])
    return sum

def part2(input_data: str) -> int:
    """
    Solve Part 2 of the day's challenge.
    
    Args:
        input_data (str): Puzzle input
    
    Returns:
        int: Solution for Part 2
    """
    pos = -1
    sum = 0
    dos = -1
    dos_list = []
    donts_list = []
    donts = 0
    line1 = ""
    line = ""
    for line1 in input_data:
        line += line1
    while True:
        dos = line.lower().find("do()", dos + 1)
        if dos != -1:
            dos_list.append(dos)
        if dos == -1:
            break
        start_pos = dos

    while True:
        donts = line.lower().find(r"don't()", donts + 1)
        if donts != -1:
            donts_list.append(donts)
        if donts == -1:
            break
        start_pos = donts
    dontpos = -1
    dopos = -1

    while True:
        pos = line.lower().find("mul(", pos + 1)
        if pos == -1:
            break
        start_pos = pos + 4
        end_pos = line.find(")", start_pos)
        for dos in dos_list:
            if dos < pos:
                dopos = dos
        for donts in donts_list:
            if donts < pos:
                dontpos = donts
        if dopos > dontpos or dontpos == -1:
            if end_pos != -1:
                content = line[start_pos:end_pos]
                values = content.split(",")
                if len(values) == 2:
                    if not ' ' in values[0] and not ' ' in values[1]:
                        if values[0].strip().isdigit() and values[1].strip().isdigit():
                            if int(values[0]) < 1000 and int(values[1]) < 1000:
                                sum = sum + int(values[0]) * int(values[1])
    return sum