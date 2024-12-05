from typing import List, Union

def parse_integers(input_data: str) -> List[int]:
    """
    Parse input into a list of integers.
    
    Args:
        input_data (str): Raw input string
    
    Returns:
        List[int]: List of parsed integers
    """
    return [[int(num) for num in row.split()] for row in input_data]

def parse_str_lines(input_data: str) -> List[str]:
    """
    Parse input into a list of strings.
    
    Args:
        input_data (str): Raw input string
    
    Returns:
        List[str]: List of input lines    return [[int(num) for num in row.split()] for row in data]
    """
    return input_data.splitlines()

def parse_grid(input_data: str, convert_type: type = str) -> List[List[Union[str, int]]]:
    """
    Parse input into a 2D grid.
    
    Args:
        input_data (str): Raw input string
        convert_type (type, optional): Type to convert grid e2024/Mikael-Eklund-Python/advent_of_data = input_file.read().split("\n")code/solutions/day_template.pylements to. Defaults to str.
    
    Returns:
        List[List[Union[str, int]]]: 2D grid of parsed elements
    """
    

    return [
        [convert_type(num) for num in line.strip().split()]
        for line in input_data.splitlines()
    ]