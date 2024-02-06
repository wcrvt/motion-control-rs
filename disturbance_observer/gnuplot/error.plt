reset

data = "../data/estimated.csv"

set datafile separator ","
set grid

p data u 1:2 w l ti "tau dis",\
  data u 1:($2-$3) w l ti "1",\
  data u 1:($2-$4) w l ti "2",\
  data u 1:($2-$5) w l ti "3",\
  data u 1:($2-$6) w l ti "4",\
