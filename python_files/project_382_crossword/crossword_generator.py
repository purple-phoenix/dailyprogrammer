from typing import List


def crossword_generator(dim_len, across_word_indexes, down_word_indexes) -> List[List[bool]]:
    pass


def validate_crossword(crossword: List[List[bool]], dim_len: int) -> bool:
    num_rows_is_dim_len = len(crossword) == dim_len
    if not num_rows_is_dim_len:
        print("Number of rows is not " + str(dim_len))
        return False

    for index, row in enumerate(crossword):
        if len(row) != dim_len:
            print("Row " + str(index) + " is not " + str(dim_len))
            return False

    return True


