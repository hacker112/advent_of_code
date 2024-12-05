defmodule Day202401Test do
  use ExUnit.Case
  doctest Day202401

  test "test part 1 with example data" do
    answer = Day202401.calculatePart1("day202401_test.txt")
    expected = 11
    assert answer == expected
  end

  test "test part 1 with real data" do
    answer = Day202401.calculatePart1("day202401.txt")
    IO.puts("Problem 1: #{answer}")
    expected = 2_430_334
    assert answer == expected
  end

  test "test part 2 with example data" do
    answer = Day202401.calculatePart2("day202401_test.txt")
    expected = 31
    assert answer == expected
  end

  test "test part 2 with real data" do
    answer = Day202401.calculatePart2("day202401.txt")
    IO.puts("Problem 2: #{answer}")
    expected = 28_786_472
    assert answer == expected
  end
end
