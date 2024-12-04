#!/usr/bin/env ruby

regex = /(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))/
enabled = true
sum = 0
File.read(ARGV[0]).scan(regex).each do |(match, one, two)|
  enabled = match.start_with? 'do(' if match.start_with? 'do'
  sum += one.to_i * two.to_i if enabled && match.start_with?('mul(')
end
pp sum
