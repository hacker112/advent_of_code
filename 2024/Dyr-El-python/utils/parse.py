import re


def parse_lines(inp, functions, split_groups=None):
    if split_groups:
        groups = list(inp.split(split_groups))
        group_result = list()
        for function, group in zip(functions, group):
            group_result.append(list())
            for line in group:
                group_result[-1].append(function(line))
        return group_result
    result = []
    for line in inp.splitlines():
        result.append(functions(line))
    return result
