import { assertEquals } from "@std/assert";
import { day1p1, day1p2 } from "./day1.ts";

Deno.test("day1 examples", () => {
    const testInput = `3   4
4   3
2   5
1   3
3   9
3   3`;

    assertEquals(day1p1(testInput), 11);
    assertEquals(day1p2(testInput), 31);
});

const input = Deno.readTextFileSync("day1.input");
Deno.test("day1 puzzle", () => {
    assertEquals(day1p1(input), 1197984);
    assertEquals(day1p2(input), 1816);
});

const day1Group = "day01";
Deno.bench({
    name: "day1 p1",
    group: day1Group,
    fn: () => {
        day1p1(input);
    },
});

Deno.bench({
    name: "day1 p2",
    group: day1Group,
    fn: () => {
        day1p2(input);
    },
});

Deno.bench({
    name: "full day 1",
    group: day1Group,
    fn: () => {
        day1p1(input);
        day1p2(input);
    },
});
