import importlib
import pytest
from pathlib import Path

DIRECTORY = Path(__file__).parent

solution = importlib.import_module(f"{DIRECTORY.name}.solution")


@pytest.mark.parametrize(
    "puzzle, result",
    [
        (1, 161),
        (2, 48)
    ],
)
def test_puzzle_example_input(puzzle, result):
    if puzzle == 1:
        input_file = Path(__file__).parent / "example_input.txt"
    else:
        input_file = Path(__file__).parent / "example_input2.txt"
    puzzle_solver = getattr(solution, f"puzzle{puzzle}")
    assert puzzle_solver(input_file) == result


@pytest.mark.parametrize(
    "puzzle, result",
    [
        (1, 183380722),
        (2, 82733683)
    ],
)
def test_puzzle_real_input(puzzle, result):
    input_file = Path(__file__).parent / "input.txt"
    puzzle_solver = getattr(solution, f"puzzle{puzzle}")
    assert puzzle_solver(input_file) == result