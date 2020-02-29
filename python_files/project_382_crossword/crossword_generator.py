from typing import List
from math import ceil
from networkx import Graph, is_connected


def crossword_generator(dim_len, across_word_indexes, down_word_indexes) -> List[List[bool]]:
    pass


def validate_crossword(crossword: List[List[bool]], dim_len: int) -> bool:
    if not validate_cross_len(crossword, dim_len):
        return False
    one_eighty_validity = validate_cross_one_eighty_symmetry(crossword, dim_len)
    if not one_eighty_validity:
        return False

    if not validate_white_connection(crossword):
        return False

    return True


def validate_cross_one_eighty_symmetry(crossword: List[List[bool]], dim_len: int) -> bool:
    for index in range(0, ceil(dim_len/2)):
        row = crossword[index]
        one_eighty_dim_comparison_row = crossword[dim_len - index - 1]
        one_eighty_dim_comparison_row.reverse()
        if row != one_eighty_dim_comparison_row:
            print("Crossword is not one eighty symmetrical at " + str(index))
            return False
        else:
            one_eighty_dim_comparison_row.reverse()

    return True


def validate_cross_len(crossword: List[List[bool]], dim_len: int) -> bool:
    num_rows_is_dim_len = len(crossword) == dim_len
    if not num_rows_is_dim_len:
        print("Number of rows is not " + str(dim_len))
        return False

    for index, row in enumerate(crossword):
        if len(row) != dim_len:
            print("Row " + str(index) + " is not " + str(dim_len))
            return False

    return True


def validate_white_connection(crossword: List[List[bool]]) -> bool:
    graph = make_graph(crossword)
    return is_connected(graph)


def make_graph(crossword: List[List[bool]]) -> Graph:
    graph = Graph()

    dim_len = len(crossword)
    for row in range(dim_len):
        for col in range(dim_len):
            this_entry = crossword[row][col]
            if not this_entry:
                continue
            if col != dim_len - 1:
                maybe_entry_to_right = crossword[row][col + 1]
                if maybe_entry_to_right:
                    graph.add_edge((row, col), (row, col + 1))
            if row != dim_len - 1:
                maybe_entry_below = crossword[row + 1][col]
                if maybe_entry_below:
                    graph.add_edge((row, col), (row + 1, col))

    return graph
