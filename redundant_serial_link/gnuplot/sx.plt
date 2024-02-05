reset

data = "../data/out.csv"

set datafile separator ","

sp  data u 2:3:1 w l,\
    data u 4:5:1 w l