reset

data = "../data/bode.csv"

set datafile separator ","
set grid
set logscale x

set multiplot layout 2,1

p data u 1:2 w l ti "gain",\

p data u 1:3 w l ti "phase",\

unset multiplot