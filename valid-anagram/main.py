def is_anagram(s: str, t: str) -> bool:
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


s = "racecar"
t = "carrace"

assert is_anagram(s, t) == True

s = "jar"
t = "jam"

assert is_anagram(s, t) == False
