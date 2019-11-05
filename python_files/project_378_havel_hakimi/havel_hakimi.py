from typing import List

Responses = List[int]


def remove_all_zeros(responses: Responses) -> Responses:
    return list(filter(lambda response: response != 0, responses))


def sort_desc(responses: Responses) -> Responses:
    return sorted(responses, reverse=True)


def is_responses_shorter_than(n: int, responses: Responses) -> bool:
    return n > len(responses)


def reduce_first_n_by_one(first_n: int, sorted_responses: Responses) -> Responses:
    return _reduce_first_n_by_one_helper(first_n, sorted_responses, [], 0)


def _reduce_first_n_by_one_helper(first_n: int,
                                  sorted_responses: Responses,
                                  processed_responses,
                                  accum: int) -> Responses:
    if first_n == accum:
        return processed_responses + sorted_responses
    elif not sorted_responses:
        return processed_responses
    else:
        processed_response = sorted_responses[0] - 1
        rest_of_responses = sorted_responses[1:]
        return _reduce_first_n_by_one_helper(first_n,
                                             rest_of_responses,
                                             processed_responses + [processed_response],
                                             accum + 1
                                             )


def havel_hakimi(responses: Responses) -> bool:
    zeroes_removed = remove_all_zeros(responses)
    sorted_responses = sort_desc(zeroes_removed)
    if not sorted_responses:
        return True
    else:
        largest_response = sorted_responses[0]
        remaining_responses = sorted_responses[1:]
        if is_responses_shorter_than(largest_response, remaining_responses):
            return False
        else:
            reduced_responses = reduce_first_n_by_one(largest_response, remaining_responses)
            return havel_hakimi(reduced_responses)
