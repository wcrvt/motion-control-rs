reset

data = "../data/out.csv"

set multiplot layout 2,1
set datafile separator ","
set grid

p data u 1:2 w l ti "x",\

p data u 1:5 w l ti "theta",\

unset multiplot
