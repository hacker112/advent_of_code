from argparse import ArgumentParser
from pathlib import Path

def puzzle1(input_file: Path):
    cnt = 0
    for report in input_file.read_text().splitlines():
        levels = [int(level) for level in report.split()]
        is_safe_rising = all(
            _is_safe_rise(x,y)
            for x, y in zip(levels, levels[1:])
        )
        is_safe_falling = all(
            _is_safe_rise(y,x)
            for x, y in zip(levels, levels[1:])
        )
        if is_safe_falling or is_safe_rising:
            cnt += 1


    return cnt

def _is_safe_rise(x,y):
    return x>y and x-y <= 3


def puzzle2(input_file: Path):
    cnt = 0
    for report in input_file.read_text().splitlines():
        levels = [int(level) for level in report.split()]
        if _is_safe_rising_puzzle_2(levels) or _is_safe_rising_puzzle_2(levels[::-1]):
            print(report, True)
            cnt +=1
        else:
            print(report, False)
        
    return cnt

def _is_safe_rising_puzzle_2(levels):
    is_safe, idx = _safe_until(levels)
    if is_safe:
        return True

    if idx == 1 and _safe_until(levels[1:])[0]:
        return True

    if _safe_until(levels[:idx] + levels[idx+1:])[0]:
        return True
    
    return _safe_until(levels[:idx-1] + levels[idx:])[0]
    
    
def _safe_until(levels):
    for idx, (x,y) in enumerate(zip(levels, levels[1:]), start=1):
        if not _is_safe_rise(x,y):
            return False, idx
    return True, idx


if __name__ == "__main__":
    print("Day 2")

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