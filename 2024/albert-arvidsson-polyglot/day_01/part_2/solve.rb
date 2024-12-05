#!/usr/bin/env ruby

File.readlines(ARGV[0]).map(&:strip)
    .map { |line| line.scan(/\d+/).map(&:to_i) }
    .transpose
    .then { |(one, two)| one.zip(one.map { |n| two.count { |m| m == n } }) }
    .map { |(o, t)| o * t }
    .sum
    .then(&method(:pp))
