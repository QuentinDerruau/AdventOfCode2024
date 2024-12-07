XMAS_horrizontaly=$(<input.txt)
XMAS_horrizontally_revert=$(echo "$XMAS_horrizontaly" | awk '{for(i=length($0);i>0;i--) printf "%s", substr($0, i, 1); printf "\n"}')
XMAS_verticaly=$(echo "$XMAS_horrizontaly" | awk '{for (i = 1; i <= length($0); i++) { a[i, NR] = substr($0, i, 1)}} END {for (i = 1; i <= length($0); i++) {for (j = 1; j <= NR; j++) {printf "%s", a[i, j]} printf "\n"}}')
XMAS_verticaly_revert=$(echo "$XMAS_verticaly" | awk '{for(i=length($0);i>0;i--) printf "%s", substr($0, i, 1); printf "\n"}')
XMAS_diags=$(echo "$XMAS_horrizontaly" | awk '
BEGIN { FS = "" }{for (i = 1; i <= NF; i++) d[NR, i] = $i}
END {# Diagonales top-left to bottom-right
    for (k = 1 - NR; k < NF; k++) {
        diag = ""
        for (i = 1; i <= NR; i++) {
            j = i + k
            if (j >= 1 && j <= NF) {
                diag = diag d[i, j]
            }
        }
        if (length(diag) >= 4) {
            print diag
        }
    }
    # Diagonales de haut-droite à bas-gauche
    for (k = 1; k <= NR + NF - 1; k++) {
        diag = ""
        for (i = 1; i <= NR; i++) {
            j = k - i
            if (j >= 1 && j <= NF) {
                diag = diag d[i, j]
            }
        }
        if (length(diag) >= 4) {
            print diag
        }
    }
}')

XMAS_diags_revert=$(echo "$XMAS_diags" | awk '{for(i=length($0);i>0;i--) printf "%s", substr($0, i, 1); printf "\n"}')

o_h=$(echo "$XMAS_horrizontaly" | grep -o XMAS | wc -l)
o_h_r=$(echo "$XMAS_horrizontally_revert" | grep -o XMAS | wc -l)
o_v=$(echo "$XMAS_verticaly" | grep -o XMAS | wc -l)
o_v_r=$(echo "$XMAS_verticaly_revert" | grep -o XMAS | wc -l)
o_d=$(echo "$XMAS_diags" | grep -o XMAS | wc -l)
o_d_r=$(echo "$XMAS_diags_revert" | grep -o XMAS | wc -l)

echo "total" : $(($o_h + $o_h_r + $o_v + $o_v_r + $o_d + $o_d_r))