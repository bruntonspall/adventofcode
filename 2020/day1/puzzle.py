import itertools

def find_pair(l, target):
    for (a,b) in itertools.combinations(l,2):
        if a+b == target:
            return (a,b)
    return (0,0)

def find_triple(l, target):
    for (a,b,c) in itertools.combinations(l,3):
        if a+b+c == target:
            return (a,b,c)
    return (0,0,0)
