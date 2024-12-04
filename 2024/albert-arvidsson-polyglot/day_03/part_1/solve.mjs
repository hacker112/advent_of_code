#!/usr/bin/env node
import { readFileSync } from "node:fs";

const input = readFileSync(process.argv[2], "utf-8");
const regex = /mul\((\d{1,3}),(\d{1,3})\)/g;
let sum = 0;
for (const [, x, y] of input.matchAll(regex)) sum += +x * +y;
console.log(sum);
