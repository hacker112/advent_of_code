#!/usr/bin/env ruby

File.read(ARGV[0])
    .scan(/mul\((\d{1,3}),(\d{1,3})\)/)
    .sum { |matches| matches.map(&:to_i).inject(:*) }
    .then(&method(:pp))
