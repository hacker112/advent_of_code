from argparse import ArgumentParser
from collections import Counter
from pathlib import Path


def puzzle1(input_file: Path):
    values = [int(value) for value in input_file.read_text().split()]
    return sum(abs(v1 - v2) for v1, v2 in zip(sorted(values[::2]), sorted(values[1::2])))

def puzzle2(input_file: Path):
    all_values = [int(value) for value in input_file.read_text().split()]
    values = all_values[::2]
    counts = Counter(all_values[1::2])
    return sum(v * counts[v] for v in values)


if __name__ == "__main__":
    print("Day 1")
    parser = ArgumentParser()
    parser.add_argument("--example", action="store_true", default=False)
    args = parser.parse_args()
    if args.example:
        print("running example file")
        input_file = Path(__file__).parent / "example_input.txt"
    else:
        input_file = Path(__file__).parent / "input.txt"
    print("Puzzle 1:", puzzle1(input_file))
    try:
        print("Puzzle 2:", puzzle2(input_file))
    except NotImplementedError:
        print("puzzle 2 not implemented yet")
