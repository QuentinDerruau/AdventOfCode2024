input=$(<input.txt)
XMAS_cross=$(echo "$input" | awk 'BEGIN { FS = "" }
{ for (i = 1; i <= NF; i++) array[NR, i] = $i }
END {
     for (i = 1; i <= NR; i++) {
        for (j = 1; j <= NF; j++) {
            if (array[i, j] == "A") {
                a = array[i - 1, j - 1];
                b = array[i + 1, j + 1];
                c = array[i - 1, j + 1];
                d = array[i + 1, j - 1];
                if (((a == "S" && b == "M") || ((a == "M" && b == "S"))) &&
                    ((c == "S" && d == "M") || ((c == "M" && d == "S"))))
                        t++;
            }
        }
    }
    print t;
}')
echo "valeur part 1" : $XMAS_cross