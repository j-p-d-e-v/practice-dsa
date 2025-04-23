<?php 
ini_set("display_errors",1);
error_reporting(-1);
function is_anagram_v1($s, $t) {

  if(count_chars($s) != count_chars($t)) {
  throw new Exception("not an anagram");
  }
  $b =str_split($t);
  foreach(str_split($s) as $a) {
    $already_found = false;
    $tmp = [];
    foreach($b as $v) {
      if($v == $a and $already_found == false) {
        $already_found = true;
        continue;
      }
      $tmp[] = $v;
    }
    $b = $tmp;
  }
  if(count($b) != 0) {
    throw new Exception("not an anagram");
  }
  return true;
}

function is_anagram_v2($s, $t) {
  if(count_chars($s) != count_chars($t)) {
  throw new Exception("not an anagram");
  }
  $a = [];
  $b = [];

 foreach(str_split($s) as $i){
  if(!in_array($i,array_keys($a))){
     $a[$i]=0;
    }
   $a[$i] += 1;
  }
  
  foreach(str_split($t) as $i){
    if(!in_array($i,array_keys($b))){
     $b[$i]=0;
    }
   $b[$i] += 1;
  }

  foreach($a as $k => $v) {
    if(in_array($k,array_keys($b))) {
     $b[$k] -= $a[$k]; 
    }
    $a[$k] = 0;
  }
  if(array_sum(array_values($a)) == 0 && array_sum(array_values($b)) == 0){
    return true;
  }
  throw new Exception("not an anagram");
}

function is_anagram_v3($s, $t) {
  if(count_chars($s) != count_chars($t)) {
  throw new Exception("not an anagram");
  }
  $a = str_split($s);
  $b = str_split($t);
  sort($a);
  sort($b);
  if(count(array_diff($b,$a)) == 0) {
    return true;
  }
  throw new Exception("not an anagram");
}

echo "===============================================================\n";
$s = "racecar";
$t = "carrace";
$start_ts = hrtime(true);
is_anagram_v1($s,$t);
printf("V1 Completed at: %s ns\n",(hrtime(true) - $start_ts));

$start_ts = hrtime(true);
is_anagram_v2($s,$t);
printf("V2 Completed at: %s ns\n",(hrtime(true) - $start_ts));

$start_ts = hrtime(true);
is_anagram_v3($s,$t);
printf("V3 Completed at: %s ns\n",(hrtime(true) - $start_ts));


echo "===============================================================\n";

$s = "jar";
$t = "jam";

$start_ts = hrtime(true);
try {
  is_anagram_v1($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V1 Completed at: %s ns\n",(hrtime(true) - $start_ts));

$start_ts = hrtime(true);
try{
  is_anagram_v2($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V2 Completed at: %s ns\n",(hrtime(true) - $start_ts));


$start_ts = hrtime(true);
try{
  is_anagram_v3($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V3 Completed at: %s ns\n",(hrtime(true) - $start_ts));



echo "===============================================================\n";
$s = str_repeat("asdasdadasdas",10000);
$t = str_repeat("abcdefhghiasd",10000);

$start_ts = hrtime(true);
try{
is_anagram_v1($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V1 Completed at: %s ns\n",(hrtime(true) - $start_ts));

$start_ts = hrtime(true);
try{
  is_anagram_v2($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V2 Completed at: %s ns\n",(hrtime(true) - $start_ts));



$start_ts = hrtime(true);
try{
  is_anagram_v3($s,$t);
}
catch(Exception $e) {
  echo "not an anagram\n";
}
printf("V3 Completed at: %s ns\n",(hrtime(true) - $start_ts));



?>
