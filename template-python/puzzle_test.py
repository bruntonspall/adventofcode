import pytest
import puzzle
from utils import listofnumbers

def test_pass():
    data = listofnumbers(open("input.txt").readlines())
    answer = puzzle.solve(data)
    assert 0 == answer
