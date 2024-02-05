reset

data = "../data/out.csv"

set datafile separator ","

p data u 1:2 w l ti "x1 cmd",\
  data u 1:3 w l ti "x2 cmd",\
  data u 1:4 w l ti "x1 res",\
  data u 1:5 w l ti "x2 res"