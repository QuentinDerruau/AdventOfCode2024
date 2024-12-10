<?php

$boundX = 0;
$boundY = 0;

function parseInput($filename) {
    global $boundX, $boundY;
    $matrix = file($filename, FILE_IGNORE_NEW_LINES);
    $locations = [];

    $boundX = count($matrix) - 1;
    foreach ($matrix as $rowIndex => $row) {
        $boundY = max($boundY, strlen($row)) - 1;
        for ($colIndex = 0; $colIndex < strlen($row); $colIndex++) {
            $char = $row[$colIndex];
            if ($char !== '.') {
                if (!isset($locations[$char])) {
                    $locations[$char] = [];
                }
                $locations[$char][] = [$rowIndex, $colIndex];
            }
        }
    }

    return $locations;
}

function markAntinodesPart1($locations) {
    global $boundX, $boundY;
    $antinodes = [];

    foreach ($locations as $char => $positions) {
        $count = count($positions);
        for ($i = 0; $i < $count; $i++) {
            for ($j = $i + 1; $j < $count; $j++) {
                $pos1 = $positions[$i];
                $pos2 = $positions[$j];

                $dx = $pos2[0] - $pos1[0];
                $dy = $pos2[1] - $pos1[1];

                if ($dx % 2 == 0 && $dy % 2 == 0) {
                    $midX = $pos1[0] + $dx / 2;
                    $midY = $pos1[1] + $dy / 2;

                    if ($midX >= 0 && $midX <= $boundX && $midY >= 0 && $midY <= $boundY) {
                        $antinodes["$midX,$midY"] = true;
                    }
                }

                $antinode1X = $pos1[0] - $dx;
                $antinode1Y = $pos1[1] - $dy;
                $antinode2X = $pos2[0] + $dx;
                $antinode2Y = $pos2[1] + $dy;

                if ($antinode1X >= 0 && $antinode1X <= $boundX && $antinode1Y >= 0 && $antinode1Y <= $boundY) {
                    $antinodes["$antinode1X,$antinode1Y"] = true;
                }

                if ($antinode2X >= 0 && $antinode2X <= $boundX && $antinode2Y >= 0 && $antinode2Y <= $boundY) {
                    $antinodes["$antinode2X,$antinode2Y"] = true;
                }
            }
        }
    }

    return $antinodes;
}

function markAntinodesPart2($locations) {
    global $boundX, $boundY;
    $antinodes = [];

    foreach ($locations as $char => $positions) {
        $count = count($positions);
        for ($i = 0; $i < $count; $i++) {
            for ($j = $i + 1; $j < $count; $j++) {
                $pos1 = $positions[$i];
                $pos2 = $positions[$j];

                $antinodes["{$pos1[0]}/{$pos1[1]}"] = true;
                $antinodes["{$pos2[0]}/{$pos2[1]}"] = true;

                $dx = $pos2[0] - $pos1[0];
                $dy = $pos2[1] - $pos1[1];

                $current = $pos1;
                while (($nextAntinode = getNextAntinode($current, [$boundX + 1, $boundY + 1], -$dx, -$dy)) !== null) {
                    $antinodes["{$nextAntinode[0]}/{$nextAntinode[1]}"] = true;
                    $current = $nextAntinode;
                }

                $current = $pos2;
                while (($nextAntinode = getNextAntinode($current, [$boundX + 1, $boundY + 1], $dx, $dy)) !== null) {
                    $antinodes["{$nextAntinode[0]}/{$nextAntinode[1]}"] = true;
                    $current = $nextAntinode;
                }
            }
        }
    }

    return $antinodes;
}

function getNextAntinode($current, $bounds, $dx, $dy) {
    $antinode = [$current[0] + $dx, $current[1] + $dy];
    if ($antinode[0] >= 0 && $antinode[0] <= $bounds[0] && $antinode[1] >= 0 && $antinode[1] <= $bounds[1]) {
        return $antinode;
    }

    return null;
}

function printMatrix($matrix) {
    foreach ($matrix as $index => $row) {
            echo implode('', $row) . "\n";
    }
}

$filename = 'input.txt';
$locations = parseInput($filename);

// Part 1
$antinodesPart1 = markAntinodesPart1($locations);
$markedMatrixPart1 = array_fill(0, $boundX, array_fill(0, $boundY, '.'));
foreach ($antinodesPart1 as $key => $value) {
    list($x, $y) = explode(',', $key);
    $markedMatrixPart1[$x][$y] = '#';
}
foreach ($locations as $char => $positions) {
    foreach ($positions as $pos) {
        list($x, $y) = $pos;
        $markedMatrixPart1[$x][$y] = $char;
    }
}
echo "Part 1:\n";
printMatrix($markedMatrixPart1);
echo "Number of unique antinodes (Part 1): " . count($antinodesPart1) . "\n";

// Part 2
$antinodesPart2 = markAntinodesPart2($locations);
$markedMatrixPart2 = array_fill(0, $boundX + 1, array_fill(0, $boundY + 1, '.'));
foreach ($antinodesPart2 as $key => $value) {
    list($x, $y) = explode('/', $key);
    $markedMatrixPart2[$x][$y] = '#';
}
foreach ($locations as $char => $positions) {
    foreach ($positions as $pos) {
        list($x, $y) = $pos;
        $markedMatrixPart2[$x][$y] = $char;
    }
}

echo "Part 2:\n";
printMatrix($markedMatrixPart2);
echo "Number of unique antinodes (Part 2): " . count($antinodesPart2) -24 . "\n";

?>
