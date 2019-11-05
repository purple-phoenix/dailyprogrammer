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
        num_hours_with_am_pm = num_tens_hours - 12
        return "It's" + \
               clock_num_to_str(num_hours_with_am_pm) + \
               clock_num_to_str(num_tens_minutes) + \
               clock_num_to_str(num_one_minutes) + "pm"
    else:
        num_hours_with_am_pm = num_tens_hours
        return "It's" + clock_num_to_str(num_hours_with_am_pm) + \
               clock_num_to_str(num_tens_minutes) + \
               clock_num_to_str(num_one_minutes) + "am"


def clock_num_to_str(clock_num: int) -> str:
    pass
