import time


def is_anagram_v1(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    b = t
    for a in s:
        already_found = False
        tmp = []
        for v in b:
            if v == a and not already_found:
                already_found = True
                continue
            tmp.append(v)
        b = tmp
    return len(b) == 0


def is_anagram_v2(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    a = {}
    b = {}
    for i in s:
        if not i in a.keys():
            a[i] = 0
        a[i] += 1
    for i in t:
        if not i in b.keys():
            b[i] = 0
        b[i] += 1
    for i in a:
        if i in b.keys():
            b[i] = abs(b[i] - a[i])
        if i in a.keys():
            a[i] = 0
    return abs(sum(a.values())) == 0 and sum(b.values()) == 0


def is_anagram_v3(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    return sorted(s) == sorted(t)


print("=============================================")
s = "racecar"
t = "carrace"

start_ts = time.time_ns()
assert is_anagram_v1(s, t) == True
print("V1 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v2(s, t) == True
print("V2 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v3(s, t) == True
print("V3 Completed at: {}".format((time.time_ns() - start_ts)))


print("=============================================")
s = "jar"
t = "jam"

start_ts = time.time_ns()
assert is_anagram_v1(s, t) == False
print("V1 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v2(s, t) == False
print("V2 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v3(s, t) == False
print("V3 Completed at: {}".format((time.time_ns() - start_ts)))

print("=============================================")
s = "asdasdadasdas" * 1000
t = "abcdefhghiasd" * 1000

start_ts = time.time_ns()
assert is_anagram_v1(s, t) == False
print("V1 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v2(s, t) == False
print("V2 Completed at: {}".format((time.time_ns() - start_ts)))

start_ts = time.time_ns()
assert is_anagram_v3(s, t) == False
print("V3 Completed at: {}".format((time.time_ns() - start_ts)))
