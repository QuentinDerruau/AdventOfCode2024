#!/bin/bash

# Parser

# valuepart1=0
# valuepart1=0
# enable="do"
# while read -r line
# do  
#     if [[ $line =~ mul\([0-9]+,[0-9]+\) ]]; then
#         num1=$(echo $line | awk -F'[(),]' '{print $2}')
#         num2=$(echo $line | awk -F'[(),]' '{print $3}')
#         valuepart1=$((valuepart1 + num1 * num2))
#         if [ $enable == "dont" ]; then
#             num1=$(echo $line | awk -F'[(),]' '{print $2}')
#             num2=$(echo $line | awk -F'[(),]' '{print $3}')
#             valuepart2=$((valuepart2 + num1 * num2))
#         fi
#     elif [[ $line =~ do\(\) ]]; then
#         enable="do"
#     elif [[ $line =~ don.t\(\) ]]; then
#         enable="dont"
#     fi
# done < <(grep -Po 'do\(\)|don.t\(\)|mul\(\d{1,3},\d{1,3}\)' input.txt)
# echo "valeur part 1" : $valuepart1
# echo "valeur part 2" : $(($valuepart1 - $valuepart2))

# Regex

valuepart1=$(cat input.txt | grep -Po 'mul\(\d{1,3},\d{1,3}\)' | awk -F'[(),]' '{print $2 * $3}' | awk '{s+=$1} END {print s}')

valuepart2=$(cat input.txt | tr -d '\n' | sed "s/don't()/\ndon't()/g" | sed "s/do()/\ndo()/g" | grep -w "don't" | grep -Po 'mul\(\d{1,3},\d{1,3}\)' | awk -F'[(),]' '{print $2 * $3}' | awk '{s+=$1} END {print s}')

echo "valeur part 1" : $valuepart1
echo "valeur part 2" : $(($valuepart1 - $valuepart2))

