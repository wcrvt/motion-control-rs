reset

data1 = "../data/out_mode_0.csv"
data2 = "../data/out_mode_1.csv"
data3 = "../data/out_mode_2.csv"
data4 = "../data/out_mode_3.csv"

set datafile separator ','
set multiplot layout 4,2

##### 1 #####
data = data1
l 'atomic/x.plt'
l 'atomic/f.plt'

##### 2 #####
data = data2
l 'atomic/x.plt'
l 'atomic/f.plt'

##### 3 #####
data = data3
l 'atomic/x.plt'
l 'atomic/f.plt'

##### 4 #####
data = data4
l 'atomic/x.plt'
l 'atomic/f.plt'

unset multiplot