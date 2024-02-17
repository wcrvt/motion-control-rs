reset

data = "../data/bode.csv"

set datafile separator ","
set grid

set logscale x
p data u 1:2 w l ti "gain",\
