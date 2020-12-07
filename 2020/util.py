def parse_groups(lines):
    result = []
    group = []
    for line in lines:
        if line == "":
            result.append(group)
            group = []
        else:
            group.append(line)
    result.append(group)
    return result

assert [['a', 'b']] == parse_groups(["a", "b"])
assert [['a', 'b'], ['c']] == parse_groups(['a', 'b', '', 'c'])

def parse_groups_from_file(filename):
    return parse_groups([l.strip() for l in open(filename)])