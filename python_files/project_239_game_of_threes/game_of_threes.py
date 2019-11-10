##
# Do not name variables "input" as input is an existing variable in python:
#  https://stackoverflow.com/questions/20670732/is-input-a-keyword-in-python
#
# By convention, internal functions should start with an underscore.
# https://stackoverflow.com/questions/11483366/protected-method-in-python
##

##
#  @param  operand:  number to operate on.
#  @return boolean:  If inputs are valid.
##
def game_of_threes(operand: int) -> bool:
    # Check for invalid inputs first
    if operand < 1:
        print("Invalid input. Starting number must be greater than zero")
        return False
    if operand == 1:
        print(operand)
        return True
    # As prior functional blocks all return, there is no need for "elif" control blocks
    if _divisible_by_three(operand):
        print(str(operand) + " 0")
        return game_of_threes(int(operand / 3))
    if _add_one_divisible_by_three(operand):
        print(str(operand) + " 1")
        return game_of_threes(operand + 1)
    if _sub_one_divisible_by_three(operand):
        print(str(operand) + " -1")
        return game_of_threes(operand - 1)


def _divisible_by_three(num: int) -> bool:
    return num % 3 == 0


def _add_one_divisible_by_three(num: int) -> bool:
    return (num + 1) % 3 == 0


def _sub_one_divisible_by_three(num: int) -> bool:
    return (num - 1) % 3 == 0
