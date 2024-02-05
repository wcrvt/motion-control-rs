reset

data = "../out.csv"
set datafile separator ','


##### 1 #####
p data u 1:2 w l lt 5 lc "#333333" ti "tau ref",\
  data u 1:(-$3) w l lc "#c80000" ti "tau env",\
  data u 1:4 w l lc "#0000c8" ti "tau est",\
