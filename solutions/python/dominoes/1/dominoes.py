from typing import List, Tuple, Optional, Set
def can_chain(input: List[Tuple[int, int]]) -> Optional[List[Tuple[int, int]]]:
    if len(input) == 0:
        return []
    rever = [(end, start) for start, end in input]
    for i in range(len(input)):
        vis: Set[int] = set()
        vis.add(i)
        vec = [input[i]]
        result = f(input[i][0], input[i][1], input, rever, vis, vec)
        if result is not None:
            return result
    return None


def get_candidate(candidate_idx: List[int], input: List[Tuple[int, int]], end: int, vis: Set[int]):
    for idx, (start, _) in enumerate(input):
        if start == end and idx not in vis:
            candidate_idx.append(idx)


def f(first: int, end: int,
      input: List[Tuple[int, int]],
      rever: List[Tuple[int, int]],
      vis: Set[int],
      vec: List[Tuple[int, int]]) -> Optional[List[Tuple[int, int]]]:
    if len(vis) == len(input):
        if first == end and is_right(vec):
            return vec.copy()
        return None
    candidate_idx_input = []
    candidate_idx_rever = []
    get_candidate(candidate_idx_input, input, end, vis)
    get_candidate(candidate_idx_rever, rever, end, vis)

    for idx in candidate_idx_input:
        vec.append(input[idx])
        vis.add(idx)
        result = f(first, input[idx][1], input, rever, vis, vec)
        if result is not None:
            return result
        vis.remove(idx)
        vec.pop()

    for idx in candidate_idx_rever:
        vec.append(rever[idx])
        vis.add(idx)
        result = f(first, rever[idx][1], input, rever, vis, vec)
        if result is not None:
            return result
        vis.remove(idx)
        vec.pop()
    return None


def is_right(input: List[Tuple[int, int]]) -> bool:
    first, equal = input[0]
    for i in range(1, len(input)):
        f, e = input[i]
        if equal != f:
            return False
        equal = e
    return equal == first