reset

data = "../data/omega.csv"

set datafile separator ","
set grid

p data u 1:2 w l ti "omega res 1",\
  data u 1:3 w l ti "omega res 2",\
  data u 1:4 w l ti "omega res 3",\
  data u 1:5 w l ti "omega est 1",\
  data u 1:6 w l ti "omega est 2",\
  data u 1:7 w l ti "omega est 3",\
