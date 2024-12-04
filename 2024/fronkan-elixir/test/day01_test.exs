defmodule Day01Test do
  use ExUnit.Case
  doctest Day01

  test "example 1" do
    assert Day01.run(:example1) == 11
  end

  test "puzzle 1" do
    assert Day01.run(:puzzle1) == 2970687
  end

  test "example 2" do
    assert Day01.run(:example2) == 31
  end

  test "puzzle 2" do
    assert Day01.run(:puzzle2) == 23963899
  end

end
