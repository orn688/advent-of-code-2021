"""Proof-of-concept dynamic programming version of day 14.

TODO: Translate this into Rust.
"""

import collections
import functools

def main(inp, iterations):
    lines = inp.strip().splitlines()
    pattern = lines[0]
    rules = {}
    for line in lines[2:]:
        start, end = line.split(" -> ")
        rules[tuple(start)] = end

    @functools.cache
    def compute(c1, c2, remaining):
        if remaining == 0 or (c1, c2) not in rules:
            return collections.Counter()
        new = rules[c1, c2]
        return compute(c1, new, remaining-1) + compute(new, c2, remaining-1) + collections.Counter({new: 1})

    result = collections.Counter(pattern)
    prev = pattern[0]
    for curr in pattern[1:]:
        result += compute(prev, curr, iterations)
        prev = curr

    print(result.most_common())
    items = result.most_common()
    _, most_common = items[0]
    _, least_common = items[-1]
    return most_common - least_common


TEST_INPUT = """
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"""

REAL_INPUT = """
OHFNNCKCVOBHSSHONBNF

SV -> O
KP -> H
FP -> B
VP -> V
KN -> S
KS -> O
SB -> K
BS -> K
OF -> O
ON -> S
VS -> F
CK -> C
FB -> K
CH -> K
HS -> H
PO -> F
NP -> N
FH -> C
FO -> O
FF -> C
CO -> K
NB -> V
PP -> S
BB -> N
HH -> B
KK -> H
OP -> K
OS -> V
KV -> F
VH -> F
OB -> S
CN -> H
SF -> K
SN -> P
NF -> H
HB -> V
VC -> S
PS -> P
NK -> B
CV -> P
BC -> S
NH -> K
FN -> P
SH -> F
FK -> P
CS -> O
VV -> H
OC -> F
CC -> N
HK -> N
FS -> P
VF -> B
SS -> V
PV -> V
BF -> V
OV -> C
HO -> F
NC -> F
BN -> F
HC -> N
KO -> P
KH -> F
BV -> S
SK -> F
SC -> F
VN -> V
VB -> V
BH -> O
CP -> K
PK -> K
PB -> K
FV -> S
HN -> K
PH -> B
VK -> B
PC -> H
BO -> H
SP -> V
NS -> B
OH -> N
KC -> H
HV -> F
HF -> B
HP -> S
CB -> P
PN -> S
BK -> K
PF -> N
SO -> P
CF -> B
VO -> C
OO -> K
FC -> F
NV -> F
OK -> K
NN -> O
NO -> O
BP -> O
KB -> O
KF -> O
"""

if __name__ == "__main__":
    print(main(REAL_INPUT, 40))