const fs = require('fs');
const { get } = require('http');
var map = [];
var original_map = [];
var direction = "";
var is_ended = false;

function getPos(map) {
    for (let i = 0; i < map.length; i++) {
        for (let j = 0; j < map[i].length; j++) {
            if (map[i][j] != '.' && map[i][j] != '#') {
                switch (map[i][j]) {
                    case '^':
                        direction = "top";
                        return [i, j];
                    case 'V':
                        direction = "bottom";
                        return [i, j];
                    case '<':
                        direction = "left";
                        return [i, j];
                    case '>':
                        direction = "right";
                        return [i, j];
                }
            }
        }
    }
    return null; // Return null if no position is found
}

function turn(map, x, y) {
        switch (direction) {
            case "top":
                map[x][y] = '>';
                direction = "right";
                break
            case "bottom":
                map[x][y] = '<';
                direction = "left";
                break;
            case "left":
                map[x][y] = '^';
                direction = "top";
                break;
            case "right":
                map[x][y] = 'V';
                direction = "bottom";
                break;
        }
}

function move(map, x, y) {
    switch (direction) {
        case "top":
            if (x - 1 < 0 || !map[x - 1][y]) {
                is_ended = true;
                map[x][y] = 'X';
                break;
            } else if (map[x - 1][y] == '#') {
                turn(map, x, y);
                break;
            } else {
                map[x - 1][y] = map[x][y];
                map[x][y] = 'X';
                break;
            }
        case "bottom":
            if (x + 1 >= map.length || !map[x + 1][y]) {
                is_ended = true;
                map[x][y] = 'X';
                break;
            } else if (map[x + 1][y] == '#') {
                turn(map, x, y);
                break;
            } else {
                map[x + 1][y] = map[x][y];
                map[x][y] = 'X';
                break;
            }
        case "left":
            if (y - 1 < 0 || !map[x][y - 1]) {
                is_ended = true;
                map[x][y] = 'X';
                break;
            } else if (map[x][y - 1] == '#') {
                turn(map, x, y);
                break;
            } else {
                map[x][y - 1] = map[x][y];
                map[x][y] = 'X';
                break;
            }
        case "right":
            if (y + 1 >= map[x].length || !map[x][y + 1]) {
                is_ended = true;
                map[x][y] = 'X';
                break;
            } else if (map[x][y + 1] == '#') {
                turn(map, x, y);
                
                break;
            } else {
                map[x][y + 1] = map[x][y];
                map[x][y] = 'X';
                break;
            }
    }
}

function countOccurrences(map, char) {
    let count = 0;
    map.forEach(row => {
        row.forEach(cell => {
            if (cell === char) {
                count++;
            }
        });
    });
    return count;
}
function findPositions(map, char) {
    let positions = [];
    map.forEach((row, i) => {
        row.forEach((cell, j) => {
            if (cell === char && (i !== getPos(original_map)[0] || j !== getPos(original_map)[1])) {
                positions.push([i, j]);
            }
        });
    });
    return positions;
}

function allPositions(xGuard,yGuard){
    var t_ended = false;
    var is_loop = false
    map[xGuard][yGuard] = '#';
    while (t_ended == false){
            for (let i = 0; i < 10000; i++){
                const pos = getPos(map);
                if (pos) {
                    const [x, y] = pos;
                    move(map, x, y);
                    is_loop = true;
                }
                else {
                    t_ended = true;
                    is_loop = false;
                    break;
                }  
            }
            t_ended = true;
    }
    return is_loop;
}
function printMap(map) {
    map.forEach(row => {
        console.log(row.join(' '));
    });
    console.log('\n');
}

fs.readFile('input.txt', 'utf8', function(err, data) {
    if (err) {
        console.error(err);
        return;
    }
    original_map = data.trim().split(/\r?\n/).map(row => row.split(''));
    map = data.trim().split(/\r?\n/).map(row => row.split(''));
    while (!is_ended) {
        const pos = getPos(map);
        const [x, y] = pos;
        move(map, x, y);
    }

    const xCount = countOccurrences(map, 'X');

    console.log('Number of X Part1:', xCount);


    pathX=findPositions(map, 'X');
    map = data.trim().split(/\r?\n/).map(row => row.split(''));
    var loop = 0;
    for(let i = 0; i < pathX.length; i++){
    // for(let i = 0; i < 1; i++){
  
        map = data.trim().split(/\r?\n/).map(row => row.split(''));
        
            if (allPositions(pathX[i][0],pathX[i][1]) == true){
                loop += 1;
            }

        
    };
    console.log('Number of X Part2:', loop);
});