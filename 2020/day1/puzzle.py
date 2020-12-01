def find_pair(l, target):
    for a in l:
        for b in l:
            if a != b and a+b == target:
                return (a,b)
    return (0,0)