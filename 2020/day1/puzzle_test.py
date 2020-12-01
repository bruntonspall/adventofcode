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
def test_calculate_score(l, target, pair):
    actual = puzzle.find_pair(l, target)
    assert pair == actual

def test_final():
    data = listofnumbers(open("input.txt").readlines())
    actual = puzzle.find_pair(data, 2020)
    print(actual[0]*actual[1])
    assert (1073,947) == actual