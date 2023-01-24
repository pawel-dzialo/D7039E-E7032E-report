import mne
import numpy

#given a value, and a max voltage reference, scales the value to where
#max positive voltage reference = maxint16 , max negative voltage reference = minint16
#if val exceeds the range, the corresponding max min int is returned.
def convert_to_adc(val,vref):
    if(val<vref and val>-vref):
        return numpy.int16((val*(numpy.iinfo(numpy.int16).max))/vref)
    elif val>0:
        return numpy.iinfo(numpy.int16).max
    else:
        return numpy.iinfo(numpy.int16).min

raw_data = mne.io.read_raw_edf('/home/pawel/Downloads/chb01_04.edf').get_data()
#raw_data = [[],[],[],[],[],[],[],[],[]]
#we work with 8 channels, so throw away any channels that exceed that limit
if(len(raw_data)>8):
    raw_data_pruned = []
    i=0
    for channel in raw_data:
        if i<8:
            raw_data_pruned.append(channel)
            i+=1
    raw_data = raw_data_pruned
print(len(raw_data))

#dump the edf data into file, file is formatted as such:
# [ch1_s1,ch2_s1,...,ch16_s1]...[ch1_sn,ch2_sn,...,ch16_sn]
# where chx signifies xth channel, and sy indicates yth sample
f = open("output_seizure", "w")
maxvref = 0.0010379487179487177
#for i,sample in enumerate(raw_data[0]):
#    j=0
#    while j<8:
#        if raw_data[j][i]>maxvref:
#            maxvref = raw_data[j][i]
#        j+=1

print(maxvref)

for i,sample in enumerate(raw_data[0]):
    f.write('[')
    j=0
    while j<8:
        value = convert_to_adc(raw_data[j][i],maxvref)
        f.write(str(value)) #convert numpy.float64 to string
        f.write(',') #formatting
        j+=1 #go through all the channels
    f.write(']') # formatting
