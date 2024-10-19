<?php
    $sum = 0; //define a variable for store sum of multiple

    for ($number = 1; $number<1000; $number++){ 
        if($number%3 == 0 || $number%5 == 0){ 
            $sum += $number;
        }
    }
    echo $sum;
            
?>
