#!/usr/bin/env ruby

def parse_line(line) = line.scan(/\d+/).map(&:to_i)

def sorted?(lev) = lev == lev.sort || lev == lev.sort.reverse

def safe_step?((one, two)) = (1..3).include?((one - two).abs)

def safe_steps?(lev) = lev.each_cons(2).all?(&method(:safe_step?))

def safe?(lev) = sorted?(lev) && safe_steps?(lev)

def safe_with_dampening?(lev)
  safe?(lev) || (0...lev.size)
    .map { |i| lev[...i] + lev[i + 1..] }
    .any?(&method(:safe?))
end

ARGF.readlines
    .map(&method(:parse_line))
    .count(&method(:safe_with_dampening?))
    .then(&method(:pp))
