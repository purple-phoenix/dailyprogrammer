from typing import List
from math import ceil


def crossword_generator(dim_len, across_word_indexes, down_word_indexes) -> List[List[bool]]:
    pass


def validate_crossword(crossword: List[List[bool]], dim_len: int) -> bool:
    if not validate_cross_len(crossword, dim_len):
        return False

    for index in range(0, ceil(dim_len/2)):
        row = crossword[index]
        one_eighty_dim_comparison_row = crossword[dim_len - index - 1]
        one_eighty_dim_comparison_row.reverse()
        if row != one_eighty_dim_comparison_row:
            return False

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
