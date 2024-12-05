from argparse import ArgumentParser
from pathlib import Path
import re

MUL_INSTRUCTION = re.compile(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")

def puzzle1(input_file: Path):
    # mul_instruction = re.compile(r"mul\([1-9]{1,3},[1-9]{1,3}\)")   

    # chars = [
    #     char
    #     for char in input_file.read_text()
    #     if char in {"m","u","l","(", ")", ",", *(str(v) for v in range(10))}
    # ]
    # return sum(
    #     int(x) * int(y)
    #     for x,y in mul_instruction.findall("".join(chars))
    # )
    return sum(
        int(x) * int(y)
        for x,y in MUL_INSTRUCTION.findall(input_file.read_text())
    )

def puzzle2(input_file: Path):
    instructions= re.compile(r"(?:do\(\))|(?:don't\(\))|(?:mul\([0-9]{1,3},[0-9]{1,3}\))")
    execute = True
    sum = 0
    for instruction in instructions.findall(input_file.read_text()):
        if instruction == "don't()":
            execute = False
        elif instruction == "do()":
            execute = True
        else:
            if execute:
                match = MUL_INSTRUCTION.match(instruction)
                sum += int(match[1]) * int(match[2])
    return sum

if __name__ == "__main__":
    print("Day 3")

    parser = ArgumentParser()
    parser.add_argument("--example", action="store_true", default=False)
    args = parser.parse_args()


    if not args.example:
        input_file = Path(__file__).parent / "input.txt"
    else:
        print("running example file")
        input_file = Path(__file__).parent / "example_input.txt"
    try:
        print("Puzzle 1:", puzzle1(input_file))
    except NotImplementedError:
        print("puzzle 1 not implemented yet")

    if args.example:
        input_file = Path(__file__).parent / "example_input2.txt"
    try:        
        print("Puzzle 2:", puzzle2(input_file))
    except NotImplementedError:
        print("puzzle 2 not implemented yet")