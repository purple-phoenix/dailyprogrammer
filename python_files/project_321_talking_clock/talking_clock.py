from typing import Tuple

ClockTime = Tuple[int, int, int, int]


def make_clock_time(clock_str: str) -> ClockTime:
    nums_and_colon = [char for char in clock_str]
    # Leave out colon
    return int(nums_and_colon[0]), int(nums_and_colon[1]), int(nums_and_colon[3]), int(nums_and_colon[4])


def make_clock_statement(clock_time: ClockTime) -> str:
    num_tens_hours = clock_time[0]
    num_ones_hours = clock_time[1]
    num_tens_minutes = clock_time[2]
    num_one_minutes = clock_time[3]

    num_hours = 10 * num_tens_hours + num_ones_hours
    if num_hours > 12:
        num_hours_to_print = num_hours - 12
        hours_str = make_hours_str(num_hours_to_print)
        minutes_str = make_minutes_str(num_tens_minutes, num_one_minutes)
        return make_full_output(hours_str, minutes_str, "pm")
    else:
        num_hours_to_print = num_hours
        hours_str = make_hours_str(num_hours_to_print)
        minutes_str = make_minutes_str(num_tens_minutes, num_one_minutes)
        return make_full_output(hours_str, minutes_str, "am")


def make_full_output(hours_str: str, minutes_str: str, am_pm: str) -> str:
    return ""


def make_hours_str(num_hours: str) -> str:
    return ""


def make_minutes_str(num_ten_minutes: str, num_one_minutes: str) -> str:
    return ""


def clock_num_to_str(clock_num: int, is_hours: bool = False) -> str:
    zero_rep = ""
    if is_hours:
        zero_rep = "twelve"
    else:
        zero_rep = "oh"
    return " " + {
        0: zero_rep,
        1: "one",
        2: "two",
        3: "three",
        5: "five",
        4: "four",
        6: "six",
        7: "seven",
        8: "eight",
        9: "nine",
        10: "ten",
        11: "eleven",
        12: "twelve",
        13: "thirteen",
        14: "fourteen",
        15: "fifteen",
        16: "sixteen",
        17: "seventeen",
        18: "eighteen",
        19: "nineteen",
        20: "twenty",
        30: "thirty",
        40: "fourty",
        50: "fifty"
    }[clock_num] + " "
