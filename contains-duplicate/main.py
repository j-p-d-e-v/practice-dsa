def has_duplicates(nums, expected):
    num_counters = {}
    for n in nums:
        if not n in num_counters.keys():
            num_counters[n] = 0
        num_counters[n] += 1
    found_duplicates = next(
        (True for value in num_counters.values() if value >= 2), False
    )
    print("Has Duplicate, Expected: {}, Got: {} \n".format(expected, found_duplicates))
    return found_duplicates


nums = [1, 2, 3, 3]
has_duplicates(nums, True)
nums = [1, 2, 3, 4]
has_duplicates(nums, False)
