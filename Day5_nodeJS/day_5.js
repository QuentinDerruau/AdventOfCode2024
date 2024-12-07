const fs = require('fs');

var rules = [];
var updates = [];

function check_rules(update, rules) {
    const filteredRules = rules.filter(rule => update.includes(rule[0]) && update.includes(rule[1]));
    let isValid = false;
    for (let rule of filteredRules) {
        if (update.includes(rule[0]) && update.includes(rule[1])) {
            if (update.indexOf(rule[0]) < update.indexOf(rule[1])) {
                isValid = true;
            }
            else {
                return false;
            }
        }
    }
    return isValid;
}
function re_order_check(update) {
    const filteredRules = rules.filter(rule => update.includes(rule[0]) && update.includes(rule[1]));
    var isValid = false;
    while (isValid == false) {
        var isValidRound = true;
        for (let rule of filteredRules) {
            const index1 = update.indexOf(rule[0]);
            const index2 = update.indexOf(rule[1]);
            
            if (index1 > index2) {
                // Swap elements to respect the rule
                [update[index1], update[index2]] = [update[index2], update[index1]];
                isValidRound = false;
                }
            }
        if (isValidRound == true) {
            isValid = true;
        }
    }
    return update;
}

function get_mid(update) {
    const length = update.length;
    if (length % 2 === 0) {
        return length / 2;
    }
    else {
        return [Math.floor(length / 2)];
    }
}

fs.readFile('input.txt', 'utf8', function(err, data) {
    if (err) {
        console.error(err);
        return;
    }
    updates = data.match(/^.*,+.*$/gm).map(update => update.split(',').map(Number));
    rules = data.match(/[0-9]{2}\|[0-9]{2}/g).map(rule => rule.split('|').map(Number));;

    const filteredUpdates = updates.filter(update => check_rules(update, rules));
    var totalpart1 = 0
    filteredUpdates.forEach(update => {
        totalpart1 += update[get_mid(update)];
    });
    console.log( "total part 1 :",totalpart1);

    //part 2

    var totalpart2 = 0;
    const filteredNotPassedUpdates= updates.filter(update => !check_rules(update, rules));
    const filteredNewNotPassed = filteredNotPassedUpdates.filter(update => re_order_check(update, rules));

    filteredNewNotPassed.forEach(update => {
        totalpart2 += update[get_mid(update)];
    });
    console.log("total part 2 :" ,totalpart2);
});