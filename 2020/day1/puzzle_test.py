import pytest
import puzzle
from utils import listofnumbers

@pytest.mark.parametrize(
    "l,target,pair",
    [
        ([1,2,3,4,5,6], 7, (1,6)),
       ([1721,979,366,299,675,1456], 2020, (1721,299))
    ],
)
def test_find_pair(l, target, pair):
    actual = puzzle.find_pair(l, target)
    assert pair == actual

@pytest.mark.parametrize(
    "l,target,triple",
    [
        ([1,2,3,4,5,6], 9, (1,2,6)),
       ([1721,979,366,299,675,1456], 2020, (979, 366, 675))
    ],
)
def test_find_triple(l, target, triple):
    actual = puzzle.find_triple(l, target)
    assert triple == actual

def test_part1():
    data = listofnumbers(open("input.txt").readlines())
    actual = puzzle.find_pair(data, 2020)
    print(actual[0]*actual[1])
    assert (1073,947) == actual

def test_part2():
    data = listofnumbers(open("input.txt").readlines())
    actual = puzzle.find_triple(data, 2020)
    print(actual[0]*actual[1]*actual[2])
    assert (911, 618, 491) == actual