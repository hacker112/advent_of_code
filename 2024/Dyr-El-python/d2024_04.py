from aoc_prepare import PrepareAoc
from utils import Vec2D, Grid2D


def parse(inp):
    d = dict()
    for yidx, line in enumerate(inp.splitlines()):
        for xidx, ch in enumerate(line):
            if ch in "XMAS":
                d[(xidx, yidx)] = ch
    return d


def part1(inp):
    grid = Grid2D(inp, lambda x: x in "XMAS")
    directions = [Vec2D(1, 0) << i for i in range(4)] + [
        Vec2D(1, 1) << i for i in range(4)
    ]
    return sum(
        (
            "".join((grid.get(pos + direction * i, " ") for i in range(4))) == "XMAS"
            for direction in directions
            for pos, ch in grid.items()
            if ch == "X"
        )
    )


def part2(inp):
    grid = Grid2D(inp, lambda x: x in "MAS")
    paths = [
        [
            pos << i
            for pos in [
                Vec2D(-1, -1),
                Vec2D(-1, 1),
                Vec2D(0, 0),
                Vec2D(1, -1),
                Vec2D(1, 1),
            ]
        ]
        for i in range(4)
    ]
    return sum(
        ("".join(grid.get(pos + offset, " ") for offset in offsets)) == "MMASS"
        for offsets in paths
        for pos, ch in grid.items()
        if ch == "A"
    )


def test_1_1():
    assert (
        part1(
            """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""
        )
        == 18
    )


def test_1_2():
    assert (
        part2(
            """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""
        )
        == 9
    )


def main(inp):
    print("Part1:", part1(inp.strip()))
    print("Part2:", part2(inp.strip()))


if __name__ == "__main__":
    prep = PrepareAoc(2024, 4)
    main(prep.get_content())
