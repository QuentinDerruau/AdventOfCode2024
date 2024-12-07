<?php

function evaluate_operations($target, $numbers) {
    $operations = generate_operations(count($numbers) - 1);
    $found = false;
    foreach ($operations as $ops) {
        if (evaluate_expression($numbers, $ops) === $target) {
            echo "Solution trouvée : " . format_expression($numbers, $ops) . " = $target\n";
            $found = true;
        }
    }
    if (!$found) {
        echo "Aucune solution trouvée pour cible $target.\n";
    }
    return $found ? $target : 0;
}

function evaluate_expression($numbers, $ops) {
    $result = $numbers[0];
    for ($i = 0; $i < count($ops); $i++) {
        if ($ops[$i] === '+') {
            $result += $numbers[$i + 1];
        } elseif ($ops[$i] === '*') {
            $result *= $numbers[$i + 1];
        } elseif ($ops[$i] === '||') {
            $result = (int)($result . $numbers[$i + 1]);
        }
    }
    return $result;
}
function format_expression($numbers, $ops) {
    $expression = (string)$numbers[0];
    for ($i = 0; $i < count($ops); $i++) {
        $expression .= " " . $ops[$i] . " " . $numbers[$i + 1];
    }
    return $expression;
}

function generate_operations($n) {
    $operations = [];
    $base = ['+', '*', '||'];
    $count = pow(count($base), $n);
    for ($i = 0; $i < $count; $i++) {
        $ops = [];
        $temp = $i;
        for ($j = 0; $j < $n; $j++) {
            $ops[] = $base[$temp % 3];
            $temp = intdiv($temp, 3);
        }
        $operations[] = $ops;
    }
    return $operations;
}
$result_part1 = 0;
$file = fopen("input.txt", "r");
if ($file) {
    while (($line = fgets($file)) !== false) {
        list($target, $numbers) = explode(':', trim($line));
        $target = (int)$target;
        $numbers = array_map('intval', explode(' ', trim($numbers)));
        $total += evaluate_operations($target, $numbers);
    }
    fclose($file);
    echo "Total des résultats valides : $total\n";
} else {
    echo "Erreur d'ouverture du fichier.\n";
}
?>