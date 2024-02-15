reset

data1 = "../data/map.csv"
data2 = "../data/sample.csv"

set datafile separator ","
set multiplot layout 2,1

set lmargin 10
set bmargin 0
unset xtics

set ylabel "Score"
p   data1 u 1:($3+$4):($3-$4) w filledcu lc "#222222" fs transparent solid 0.1 ti "variances",\
    data1 u 1:3 w l ti "est",\
    data2 u 1:2 w p pt 6 lc "#c80000",\
    data1 u 1:2 w l ti "y" lc "#000000",\
    
set tmargin 0
set bmargin 3
set xtics
set ylabel "St. dev."
p   data1 u 1:4 w l

unset multiplot