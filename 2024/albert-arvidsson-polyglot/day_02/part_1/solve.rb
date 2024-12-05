#!/usr/bin/env ruby

def parse_line(line) = line.scan(/\d+/).map(&:to_i)

def sorted_levels?(lev) = lev == lev.sort || lev == lev.sort.reverse

def safe_steps?(line)
  line.each_cons(2).all? { |a, b| (1..3).include?((a - b).abs) }
end

ARGF.readlines
    .map(&method(:parse_line))
    .filter(&method(:sorted_levels?))
    .count(&method(:safe_steps?))
    .then(&method(:pp))
