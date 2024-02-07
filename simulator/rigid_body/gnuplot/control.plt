reset

data = "../data/quaternion.csv"

set datafile separator ","
set grid

p data u 1:2 w l ti "q0 cmd",\
  data u 1:3 w l ti "q1 cmd",\
  data u 1:4 w l ti "q2 cmd",\
  data u 1:5 w l ti "q3 cmd",\
  data u 1:6 w l ti "q0",\
  data u 1:7 w l ti "q1",\
  data u 1:8 w l ti "q2",\
  data u 1:9 w l ti "q3"
