<?php

function two_sum_v1($nums, $target)
{
    foreach ($nums as $i_key => $i_value) {
        $j_key = $i_key + 1;
        $inner_nums = array_slice($nums, $j_key);
        foreach ($inner_nums as $j_value) {
            if (($j_value + $i_value) == $target) {
                return [$i_key, $j_key];
            }
            $j_key++;
        }
    }
    return [];
}

function two_sum_v2($nums, $target)
{
    $seen = [];
    $i_key = 0;
    while ($i_key < count($nums)) {
        $i_value = $nums[$i_key];
        $complement = $target - $i_value;
        $j_key = $seen[$complement] ?? -1;
        if ($j_key !== -1) {
            if ($j_key < $i_key) {
                return [$j_key, $i_key];
            }
            return [$i_key, $j_key];
        }
        $seen[$i_value] = $i_key;
        $i_key++;
    }
    return [];
}

function check_result($start_ts, $expected, $result)
{
    $execution_time = hrtime(true) - $start_ts;
    try {
        if (count(array_diff($expected, $result)) > 0) {
            throw new Exception('Failed');
        }
    } catch (Exception $e) {
        echo sprintf("%s\n", $e->getMessage());
    }
    echo sprintf("Execution Time is %s, Expected %s, Got %s\n", $execution_time, print_r($expected, true), print_r($result, true));
}

$nums = [3, 4, 5, 6];
$target = 7;
$start_ts = hrtime(true);
$result = two_sum_v1($nums, $target);
check_result($start_ts, [0, 1], $result);

$start_ts = hrtime(true);
$result = two_sum_v2($nums, $target);
check_result($start_ts, [0, 1], $result);
echo sprintf("%s\n", str_repeat('=', 20));

$nums = [4, 5, 6];
$target = 10;
$start_ts = hrtime(true);
$result = two_sum_v1($nums, $target);
check_result($start_ts, [0, 2], $result);

$start_ts = hrtime(true);
$result = two_sum_v2($nums, $target);
check_result($start_ts, [0, 2], $result);
echo sprintf("%s\n", str_repeat('=', 20));

$nums = [5, 5];
$target = 10;
$start_ts = hrtime(true);
$result = two_sum_v1($nums, $target);
check_result($start_ts, [0, 1], $result);

$start_ts = hrtime(true);
$result = two_sum_v2($nums, $target);
check_result($start_ts, [0, 1], $result);
echo sprintf("%s\n", str_repeat('=', 20));

$nums = [];
for ($i = 0; $i <= 1000000; $i++) {
    $nums[] = $i;
}
$target = 500000;
$start_ts = hrtime(true);
$result = two_sum_v1($nums, $target);
check_result($start_ts, [0, 500000], $result);

$start_ts = hrtime(true);
$result = two_sum_v2($nums, $target);
check_result($start_ts, [249999, 250001], $result);
echo sprintf("%s\n", str_repeat('=', 20));

?>
