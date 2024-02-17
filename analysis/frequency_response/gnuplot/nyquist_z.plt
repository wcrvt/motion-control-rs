reset

data = "../data/frequency_response_z.csv"

set datafile separator ","
set grid

p data u 4:5 w l,\
