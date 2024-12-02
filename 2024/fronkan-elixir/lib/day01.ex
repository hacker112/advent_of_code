defmodule Day01 do

def part1(input) do
  # pairs = input
  # |> String.split(["\n"])
  # |> Enum.map(fn pair -> String.split(pair, [" "], trim: true) end)


  # first_values = Enum.map(pairs, fn pair -> pair[0] end)
  # second_values = Enum.map(pairs, fn pair -> pair[1] end)

  # IO.puts(first_values)
  # IO.puts(second_values)

  input
  |> parse()
  |> then(
    fn {l1, l2}->
      Enum.zip_reduce(
        Enum.sort(l1),
        Enum.sort(l2),
        0,
        fn x,y, acc -> acc + abs((x-y)) end
      )
    end)
end

def parse(input) do
  input
  |> String.split("\n")
  |> Enum.reduce({[],[]},
    fn line, {acc1, acc2} ->
      {v1, rest} = Integer.parse(line)
      {v2, _} = String.trim(rest) |> Integer.parse
      # this reverses the order in the accumulator,
      # which doesn't matter in this case.
      # otherwise use acc1 ++ [v1]. However, this is slower
      {[v1 | acc1], [v2 | acc2]}
    end)
end

def part2(input) do
  input
  |> parse()
  |> then(
    fn {l1,l2}->
      counts = Enum.frequencies(l2)
      Enum.reduce(l1, 0, fn v, acc -> acc + (v*Map.get(counts, v, 0)) end)
    end
)
end

def run(:example1) do
  {:ok, content} = File.read("./inputs/day01/example_input.txt")
  content
  |> part1()
end

def run(:example2) do
  {:ok, content} = File.read("./inputs/day01/example_input.txt")
  content
  |> part2()
end

def run(:puzzle1) do
  {:ok, content} = File.read("./inputs/day01/input.txt")
  content
  |> part1()
end

def run(:puzzle2) do
  {:ok, content} = File.read("./inputs/day01/input.txt")
  content
  |> part2()
end


end
