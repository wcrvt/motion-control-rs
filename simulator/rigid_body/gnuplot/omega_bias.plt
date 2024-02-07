reset

data = "../data/omega.csv"

set datafile separator ","
set grid

p data u 1:8 w l ti "omega bias 1",\
  data u 1:9 w l ti "omega bias 2",\
  data u 1:10 w l ti "omega bias 3",\