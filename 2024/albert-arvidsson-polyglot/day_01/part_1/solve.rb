#!/usr/bin/env ruby

File.readlines(ARGV[0]).map(&:strip)
    .map { |line| line.scan(/\d+/).map(&:to_i) }
    .transpose.map(&:sort)
    .transpose
    .sum { |(a, b)| (a.abs - b.abs).abs }
    .then(&method(:puts))
