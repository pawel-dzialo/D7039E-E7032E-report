import numpy
def convert_to_adc(val,vref):
    if(val<vref and val>-vref):
        return numpy.int16((val*(numpy.iinfo(numpy.int16).max))/vref)
    elif val>0:
        return numpy.iinfo(numpy.int16).max
    else:
        return numpy.iinfo(numpy.int16).min

maxvref = 0.0010379487179487177
threshold = convert_to_adc(0.0001,maxvref)
print(threshold)

interictal_file = open("output","r")

def parsefile(file):
    data = [[],[],[],[],[],[],[],[]]
    i = 0
    tempstr = ''
    while True:
        c=file.read(1)
        if not c:
            print("EOF")
            break
        if c == '[':
            i = 0
        elif c == ']':
            i = 0
        elif c == ',':
            data[i].append(int(tempstr))
            tempstr = ''
            i+=1
        else:
            tempstr+=c
    return data

interictal_data = parsefile(interictal_file)
for channel in interictal_data:
    print(channel[0])

spikes = [[],[],[],[],[],[],[],[]]
min_spike_len = 10
max_spike_len = 19
for j,channel in enumerate(interictal_data):
    past_threshold_samples = 0
    for i,sample in enumerate(channel):
        if sample>threshold:
            past_threshold_samples += 1
        elif past_threshold_samples < max_spike_len and past_threshold_samples > min_spike_len:
            spikes[j].append(i)
            past_threshold_samples = 0
        else:
            past_threshold_samples = 0

print(spikes)
window_amounts = []
for channel in spikes:
    i = 0
    currspike = 0
    for spike in channel:
        if i==0:
            i+=1
            currspike = spike
        elif spike<(currspike+256*5):
            i+=1
        else:
            window_amounts.append(i)
            i=0
            currspike = spike
print(window_amounts)
print(max(window_amounts))


ictal_file = open("output_seizure","r")
ictal_data = parsefile(ictal_file)
print(len(ictal_data))
for channel in ictal_data:
    print(channel[0])

spikes = [[],[],[],[],[],[],[],[]]
min_spike_len = 10
max_spike_len = 19
window_len = 256*5
windows = [[],[],[],[],[],[],[],[]]
for j,channel in enumerate(ictal_data):
    window = []
    for i, sample in enumerate(channel):
        if i%window_len == window_len-1:
            windows[j].append(window)
            window = []
        else:
            window.append(sample)

for channel in windows:
    print(channel[0])
print("1st windows")

spikes_per_window = [[],[],[],[],[],[],[],[]]
for j,channel in enumerate(windows):
    print("Channel length:",len(channel))
    for window in channel:
        spike_amount = 0
        past_threshold_samples = 0
        for sample in window:
            if sample>threshold:
                past_threshold_samples += 1
            elif past_threshold_samples<max_spike_len and past_threshold_samples > min_spike_len:
                spike_amount +=1
                past_threshold_samples = 0
            else:
                past_threshold_samples = 0
        spikes_per_window[j].append(spike_amount)

print("------------- SPIKES PER WINDOW START----------------")
print(spikes_per_window)
for i,channel in enumerate(spikes_per_window):
    print("Channel",i,"max",max(channel))
print("------------- SPIKES PER WINDOW END -----------------")

for i,channel in enumerate(spikes_per_window):
    for j,window in enumerate(channel):
        if window>=2:
            print("Seizure warning on channel ",i,", ",j*5,"sec into recording,",window,"spikes detected over window of 5 sec...")



for j,channel in enumerate(ictal_data):
    past_threshold_samples = 0
    for i,sample in enumerate(channel):
        if sample>threshold:
            past_threshold_samples += 1
        elif past_threshold_samples < max_spike_len and past_threshold_samples > min_spike_len:
            spikes[j].append(i)
            past_threshold_samples = 0
        else:
            past_threshold_samples = 0

#print(spikes)
window_amounts = []
for channel in spikes:
    i = 0
    currspike = 0
    for spike in channel:
        if i==0:
            i+=1
            currspike = spike
        elif spike<(currspike+256*5):
            i+=1
        else:
            window_amounts.append(i)
            i=0
            currspike = spike
#print(window_amounts)
#print(max(window_amounts))




