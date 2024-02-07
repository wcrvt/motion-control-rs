reset

data = "../data/out.csv"

set datafile separator ","

p data u 1:6 w l ti "theta 1",\
  data u 1:7 w l ti "theta 2",\
  data u 1:8 w l ti "theta 3"