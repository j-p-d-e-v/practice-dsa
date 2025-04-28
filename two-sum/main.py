import time


def two_sum_v1(nums, target) -> list[int]:
    for i in range(0, len(nums)):
        i_value = nums[i]
        for j in range(i + 1, len(nums)):
            j_value = nums[j]
            total = j_value + i_value
            if total == target:
                return [i, j]
    return []


def two_sum_v2(nums, target) -> list[int]:
    seen = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in seen:
            j = seen[complement]
            return [j, i] if j < i else [i, j]
        seen[num] = i
    return []


def check_result(result, expected, start_ts):
    elapsed = time.time_ns() - start_ts
    print("Completed at {}, Expected: {}, Got: {}\n".format(elapsed, expected, result))
    assert result == expected


nums = [3, 4, 5, 6]
target = 7
start_ts = time.time_ns()
check_result(two_sum_v1(nums, target), [0, 1], start_ts)
check_result(two_sum_v2(nums, target), [0, 1], start_ts)

print("=" * 20, end="\n")
nums = [4, 5, 6]
target = 10
start_ts = time.time_ns()
check_result(two_sum_v1(nums, target), [0, 2], start_ts)
check_result(two_sum_v2(nums, target), [0, 2], start_ts)

print("=" * 20, end="\n")
nums = [5, 5]
target = 10
start_ts = time.time_ns()
check_result(two_sum_v1(nums, target), [0, 1], start_ts)
check_result(two_sum_v2(nums, target), [0, 1], start_ts)

print("=" * 20, end="\n")
nums = list(range(0, 1000001))
target = 500000
start_ts = time.time_ns()
check_result(two_sum_v1(nums, target), [0, 500000], start_ts)
check_result(two_sum_v2(nums, target), [249999, 250001], start_ts)
