reset

data1 = "../out.csv"

set datafile separator ","

set grid

set xlabel "x"
set ylabel "y"

p   data1 u 1:2 w l,\
    data1 u 1:3 w l,\
    data1 u 1:4 w l,\
    data1 u 1:5 w l,\
