#!/usr/bin/env elixir

~r/mul\((\d{1,3}),(\d{1,3})\)/
|> Regex.scan(System.argv() |> hd |> File.read!(), capture: :all_but_first)
|> Enum.map(fn [x, y] -> String.to_integer(x) * String.to_integer(y) end)
|> Enum.sum
|> IO.inspect()
