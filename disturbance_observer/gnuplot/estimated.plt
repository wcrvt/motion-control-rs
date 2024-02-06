reset

data = "../data/estimated.csv"

set datafile separator ","
set grid

p data u 1:2 w l ti "tau dis",\
  data u 1:3 w l ti "zeroth",\
  data u 1:4 w l ti "first",\
  data u 1:5 w l ti "second",\
  data u 1:6 w l ti "third",\
