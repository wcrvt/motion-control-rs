reset

data = "../data/out.csv"

set datafile separator ","
set grid

p data u 1:2 w l ti "input",\
  data u 1:3 w l ti "output 1",\
  data u 1:4 w l ti "output 2",\
  data u 1:5 w l ti "output 3",\


