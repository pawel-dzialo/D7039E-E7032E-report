f = open("dataexport.txt", "r")
bytearr = []
for line in f:
    val = ""
    counter = 0
    for byte in line.split(" "):
        byte = byte.strip()
        val += byte
        counter += 1
        if counter>3:
            bytearr.append(val)
            val = ""
            counter = 0

def twos_complement(hexstr, bits):
    value = int(hexstr, 16)
    if value & (1<<bits-1):
        value -=1<<bits
    return value

result = []
for word in bytearr:
    result.append(twos_complement(word,32))

xvals = range(0,len(result), 1)

import matplotlib.pyplot as plt

plt.plot(xvals, result)
plt.show()
