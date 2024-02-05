reset

data = "../data/out.csv"

set datafile separator ","
set grid

p data u 1:5 w l ti "theta",\
