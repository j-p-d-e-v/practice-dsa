use std::collections::HashMap;
use std::time::Instant;

fn two_sum_v1(nums: &[isize], target: isize) -> (isize, isize) {
    for i in 0..nums.len() {
        let i_value: isize = nums[i];
        for j in (i + 1)..nums.len() {
            let j_value: isize = nums[j];
            let total: isize = j_value + i_value;
            if total == target {
                return (i as isize, j as isize);
            }
        }
    }
    return (-1, -1);
}

fn two_sum_v2(nums: &[isize], target: isize) -> (isize, isize) {
    let mut seen: HashMap<isize, isize> = HashMap::new();
    for _i in 0..nums.len() {
        let i: isize = _i as isize;
        let complement: isize = target - nums[_i];
        if let Some(_j) = seen.get(&complement) {
            let j: isize = *_j as isize;
            if j < i {
                return (j, i);
            }
            return (i, j);
        }
        seen.insert(nums[_i] as isize, i);
    }
    return (-1, -1);
}

fn check_result(result: (isize, isize), expected: (isize, isize), timer: Instant) {
    println!(
        "Completed at {} nano seconds, Expected: {:?}, got {:?}",
        timer.elapsed().as_nanos(),
        expected,
        result
    );
    assert!(
        result.0 == expected.0 && result.1 == expected.1,
        "invalid two sum"
    );
}

fn main() {
    let nums: &[isize] = &[3, 4, 5, 6];
    let target: isize = 7;

    let timer = Instant::now();
    check_result(two_sum_v1(nums, target), (0, 1), timer);

    let timer = Instant::now();
    check_result(two_sum_v2(nums, target), (0, 1), timer);

    println!("=========================================");
    let nums: &[isize] = &[4, 5, 6];
    let target: isize = 10;
    let timer = Instant::now();
    check_result(two_sum_v1(nums, target), (0, 2), timer);

    let timer = Instant::now();
    check_result(two_sum_v2(nums, target), (0, 2), timer);

    println!("=========================================");
    let nums: &[isize] = &[5, 5];
    let target: isize = 10;
    let timer = Instant::now();
    check_result(two_sum_v1(nums, target), (0, 1), timer);

    let timer = Instant::now();
    check_result(two_sum_v2(nums, target), (0, 1), timer);

    println!("=========================================");
    let mut nums: Vec<isize> = Vec::new();
    for i in 0..1_000_000 {
        nums.push(i);
    }
    let target: isize = 500_000;
    let timer = Instant::now();
    check_result(two_sum_v1(&nums.clone(), target), (0, 500_000), timer);

    let timer = Instant::now();
    check_result(two_sum_v2(&nums, target), (249999, 250001), timer);
}
