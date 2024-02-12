reset

data = "../data/out.csv"

set datafile separator ","
set grid

p   data u 1:2 w l ti "ref",\
    data u 1:3 w l ti "ref x",\
    data u 1:4 w l ti "res y",\
