from aoc_prepare import PrepareAoc


def parseLevels(inp):
    return [[int(x) for x in line.split()] for line in inp.splitlines()]


def checkLevel(level):
    # fmt: off
    return (all(((x - y) in (1, 2, 3) for x, y in zip(level, level[1:]))) or 
            all(((y - x) in (1, 2, 3) for x, y in zip(level, level[1:]))))
    # fmt: on


def part1(inp):
    return sum((checkLevel(level) for level in parseLevels(inp)))


def part2(inp):
    return sum(
        (
            any(
                (
                    checkLevel(level[:idx] + level[idx + 1 :])
                    for idx in range(len(level))
                )
            )
            for level in parseLevels(inp)
        )
    )


def test_1_1():
    assert (
        part1(
            """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
        )
        == 2
    )


def test_1_2():
    assert (
        part2(
            """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
        )
        == 4
    )


def main(inp):
    print("Part1:", part1(inp.strip()))
    print("Part2:", part2(inp.strip()))


if __name__ == "__main__":
    prep = PrepareAoc(2024, 2)
    main(prep.get_content())
