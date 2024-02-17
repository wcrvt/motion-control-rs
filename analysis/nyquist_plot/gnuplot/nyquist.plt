reset

data = "../data/nyquist.csv"

set datafile separator ","
set grid

p data u 2:3 w l

