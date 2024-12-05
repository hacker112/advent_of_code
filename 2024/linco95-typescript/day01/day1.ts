export const day1p1 = (input: string): number => {
    const [firstElements, secondElements] = getColumns(input);
    const zipped = zipBy(
        sorted(firstElements),
        sorted(secondElements),
        distance,
    );

    return sum(zipped);
};
function sorted<T>(input: T[]): T[] {
    return [...input].sort();
}

export const day1p2 = (input: string): number => {
    const [firstElements, secondElements] = getColumns(input);
    const similiarities = firstElements.map((lhs) =>
        secondElements.filter((rhs) => rhs === lhs).length * lhs
    );
    return sum(similiarities);
};
function getColumns(input: string): [number[], number[]] {
    const rows = input.trim().split("\n").map((row) => row.split("   "));
    const firstElements = rows.map(([first]) => parseInt(first));
    const secondElements = rows.map(([, second]) => parseInt(second));
    return [firstElements, secondElements];
}

function zipBy<T, U, K>(first: T[], second: U[], fn: (t: T, u: U) => K): K[] {
    return first.map((value, index) => fn(value, second[index]));
}

function distance(a: number, b: number): number {
    return Math.abs(a - b);
}

function sum(numbers: number[]): number {
    return numbers.reduce((acc, value) => acc + value, 0);
}
