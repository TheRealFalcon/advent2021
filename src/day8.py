from itertools import cycle

# 0: 6
# 1: 2
# 2: 5
# 3: 5
# 4: 4
# 5: 5
# 6: 6
# 7: 3
# 8: 7
# 9: 6

known_segments = {}
input = open('day8_input.txt').readlines()
sample = """\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"""

total_interested = 0
for example in input:
# for example in sample.split('\n'):
    patterns, output = map(lambda x: x.split(), example.split(' | '))

    known_digits = [''] * 10
    for pattern in cycle(patterns):
        if len(pattern) == 2:
            known_digits[1] = pattern
        elif len(pattern) == 3:
            known_digits[7] = pattern
        elif len(pattern) == 4:
            known_digits[4] = pattern
        elif len(pattern) == 7:
            known_digits[8] = pattern
        elif len(pattern) == 5 and known_digits[7] and not known_digits[3]:
            if set(known_digits[7]) < set(pattern):
                known_digits[3] = pattern
        elif len(pattern) == 6 and known_digits[3] and not known_digits[9]:
            if set(known_digits[3]) < set(pattern):
                known_digits[9] = pattern
        elif len(pattern) == 6 and known_digits[9] and known_digits[1] and not known_digits[0] and pattern != known_digits[9]:
            if set(known_digits[1]) < set(pattern):
                known_digits[0] = pattern
        elif len(pattern) == 6 and known_digits[0] and known_digits[9] and pattern not in [known_digits[0], known_digits[9]]:
            known_digits[6] = pattern
        elif len(pattern) == 5 and known_digits[4] and known_digits[3] and pattern != known_digits[3]:
            if len(set(known_digits[4]) & set(pattern)) == 2:
                known_digits[2] = pattern
            else:
                known_digits[5] = pattern

        if all(known_digits):
            break
    known_digits = [''.join(sorted(x)) for x in known_digits]

    out_str = ''
    for out in output:
        out = ''.join(sorted(out))
        out_str += str(known_digits.index(out))
    total_interested += int(out_str)
print(total_interested)
