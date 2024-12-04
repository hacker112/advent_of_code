from aoc_prepare import PrepareAoc

import re


def parse_instr(inp):
    p = re.compile(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))")
    for m in p.finditer(inp):
        mul, f1, f2, do, dont = m.groups()
        if dont:
            yield "don't"
        elif do:
            yield "do"
        elif mul:
            yield ("mul", int(f1), int(f2))


def match_mul(g):
    for m in g:
        match m:
            case ("mul", f1, f2):
                yield f1 * f2
            case _:
                yield 0


def match_all(g):
    active = True
    for m in g:
        match m:
            case "do":
                active = True
                yield 0
            case "don't":
                active = False
                yield 0
            case ("mul", f1, f2):
                yield f1 * f2 if active else 0


def part1(inp):
    return sum(prod for prod in match_mul(parse_instr(inp)))


def part2(inp):
    return sum(prod for prod in match_all(parse_instr(inp)))


def test_1_1():
    assert (
        part1(
            """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
        )
        == 161
    )


def test_1_2():
    assert (
        part2(
            """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""
        )
        == 48
    )


def main(inp):
    print("Part1:", part1(inp.strip()))
    print("Part2:", part2(inp.strip()))


if __name__ == "__main__":
    prep = PrepareAoc(2024, 3)
    main(prep.get_content())
