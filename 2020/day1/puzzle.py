def find_pair(l, target):
    for a in l:
        for b in l:
            if a != b and a+b == target:
                return (a,b)
    return (0,0)

def find_triple(l, target):
    for a in l:
        for b in l:
            for c in l:
                if a != b and a != c and b != c and a+b+c == target:
                    return (a,b,c)
    return (0,0,0)
