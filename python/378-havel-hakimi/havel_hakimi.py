from typing import List

Responses = List[int]


def warmup1(responses: Responses) -> Responses:
    return list(filter(lambda response: response != 0, responses))

def warmup2(responses: Responses) -> Responses:
    return sorted(responses, reverse=True)

def warmup3(n: int, responses: Responses) -> bool:
    return n > len(responses)

def warmup4(first_n: int, sorted_responses: Responses) -> Responses:
    return warmup4_helper(first_n, sorted_responses, [], 0)

def warmup4_helper(first_n: int,
                   sorted_responses: Responses,
                   processed_responses,
                   accum: int) -> Responses:

    if first_n == accum:
        return processed_responses + sorted_responses
    else:
        processed_response = sorted_responses[0] - 1
        rest_of_responses = sorted_responses[1:]
        return warmup4_helper(first_n,
                              rest_of_responses,
                              processed_responses + [processed_response],
                              accum + 1
                              )

def havel_hakimi(responses: Responses) -> bool:
    pass
