#!/usr/bin/env elixir

product = fn x, y -> String.to_integer(x) * String.to_integer(y) end

~r/mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)/
|> Regex.scan(System.argv() |> hd |> File.read!())
|> Enum.reduce({0, true}, fn
  ["do()"], {sum, _} -> {sum, true}
  ["don't()"], {sum, _} -> {sum, false}
  ["mul(" <> _, _, _], {sum, false} -> {sum, false}
  [_, x, y], {sum, _} -> {sum + product.(x, y), true}
end)
|> elem(0)
|> IO.puts()
