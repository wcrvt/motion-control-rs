reset

data = "../data/quaternion.csv"

set datafile separator ","
set grid

p data u 1:14 w l ti "omega bias 1",\
  data u 1:15 w l ti "omega bias 2",\
  data u 1:16 w l ti "omega bias 3"
