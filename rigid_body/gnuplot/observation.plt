reset

data = "../data/quaternion.csv"

set datafile separator ","
set grid

p data u 1:6 w l ti "q0 res",\
  data u 1:7 w l ti "q1 res",\
  data u 1:8 w l ti "q2 res",\
  data u 1:9 w l ti "q3 res",\
  data u 1:10 w l ti "q0 est",\
  data u 1:11 w l ti "q1 est",\
  data u 1:12 w l ti "q2 est",\
  data u 1:13 w l ti "q3 est"
