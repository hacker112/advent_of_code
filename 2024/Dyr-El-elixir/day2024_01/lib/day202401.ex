defmodule Day202401 do
  @moduledoc """
  Documentation for `Day202401`.
  """

  @doc """
  Hello world.

  """
  def readFile(fileName) do
    File.read!(fileName)
  end

  def parseFile(fileName) do
    body = readFile(fileName)
    line_list = String.split(body, "\n")

    string_pair_list =
      for line <- line_list,
          do: String.split(line, " ", trim: true)

    int_pair_list =
      for [s1, s2] <- string_pair_list,
          do: [elem(Integer.parse(s1), 0), elem(Integer.parse(s2), 0)]

    int_tuples = List.zip(int_pair_list)
    Enum.map(int_tuples, fn x -> Tuple.to_list(x) end)
  end

  def calculatePart1(fileName) do
    int_lists = parseFile(fileName)
    sorted_int_lists = for l <- int_lists, do: Enum.sort(l)
    Enum.sum(Enum.map(Enum.zip(sorted_int_lists), fn {x, y} -> abs(x - y) end))
  end

  def counter([]) do
    %{}
  end

  def counter([head | tail]) do
    Map.update(counter(tail), head, 1, fn x -> x + 1 end)
  end

  def calculatePart2(fileName) do
    [first, second] = parseFile(fileName)
    Enum.sum(Enum.map(first, fn x -> Map.get(counter(second), x, 0) * x end))
  end
end
