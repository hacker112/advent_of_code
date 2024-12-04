#!/usr/bin/env node
import { readFileSync } from "node:fs";

const input = readFileSync(process.argv[2], "utf-8");
const regex = /mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)/g;
let sum = 0;
let enabled = true;
for (const [op, x, y] of input.matchAll(regex)) {
  if (op === "do()") enabled = true;
  else if (op === "don't()") enabled = false;
  else if (enabled) sum += +x * +y;
}
console.log(sum);
