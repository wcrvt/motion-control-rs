reset

data = "../data/bode.csv"

set datafile separator ","
set grid

set logscale x
p data u 1:3 w l ti "gain",\
