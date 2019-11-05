from typing import Tuple

ClockTime = Tuple[int, int, int, int]


def make_clock_time(clock_str: str) -> ClockTime:
    nums_and_colon = [char for char in clock_str]
    # Leave out colon
    return int(nums_and_colon[0]), int(nums_and_colon[1]), int(nums_and_colon[3]), int(nums_and_colon[4])


def make_clock_statement(clock_time: ClockTime) -> str:
    pass
