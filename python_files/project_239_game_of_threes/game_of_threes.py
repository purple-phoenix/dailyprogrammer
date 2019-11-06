

def game_of_threes(input: int) -> ():
    if input == 1:
        print(input)
        return
    elif divisible_by_three(input):
        print(str(input) + " 0")
        return game_of_threes(int(input/3))
    elif add_one_divisible_by_three(input):
        print(str(input) + " 1")
        return game_of_threes(input + 1)
    elif sub_one_divisible_by_three(input):
        print(str(input) + " -1")
        return game_of_threes(input - 1)
    else:
        print("Invalid input. Starting number must be greater than zero")
        return


def divisible_by_three(num: int) -> bool:
    return num % 3 == 0


def add_one_divisible_by_three(num: int) -> bool:
    return (num + 1) % 3 == 0


def sub_one_divisible_by_three(num: int) -> bool:
    return (num - 1) % 3 == 0
