from typing import List

Responses = List[int]


def warmup1(responses: Responses) -> Responses:
    return list(filter(lambda response: response != 0, responses))

def warmup2(responses: Responses) -> Responses:
    return sorted(responses, reverse=True)
