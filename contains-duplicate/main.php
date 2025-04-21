<?php

function has_duplicates($nums, $expected) {
  $num_counters = [];
  foreach($nums as $n) {
    if(!array_key_exists($n,$num_counters)) {
      $num_counters[$n] = 0;
    }
    $num_counters[$n] += 1;
  }
  $found_duplicates = max(array_values($num_counters)) >= 2;

  printf("Has Duplicates, Expected %s, Got %s\n",$expected ? "True" : "False",$found_duplicates  ? "True" : "False");
  if($found_duplicates !== $expected ) {
    throw new Exception("Failed");
  }
}

$nums = [1,2,3,3];
has_duplicates($nums,true);

$nums = [1,2,3,4];
has_duplicates($nums,false);
?>
