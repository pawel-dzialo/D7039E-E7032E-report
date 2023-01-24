#string = "hello lol xd 0x45 333 0x38 hihi 0x4 jdj 0x88 jhsh 0x5"
f = open("data.txt", "r")
result = ""
bytearr = []
counter = 0
for string in f:
    for word in string.split(" "):
        if "0x" in word:
            word = word.strip()
            if(len(word)<4):
                result+="0"+word[2:]
                print(result)
                counter+=1
            else:
                result+=word[2:]
                print(result)
                counter+=1
            if(counter == 4):
                bytearr.append(result)
                result = ""
                counter = 0
bytearr.append(result)

print(bytearr)
bytearrtwo = []
for byte in bytearr:
    byte = byte[:6]
    print(byte)
    bytearrtwo.append(byte)

def twos_complement(hexstr, bits):
    value = int(hexstr, 16)
    if value & (1<<bits-1):
        value -=1 << bits
    return value

bytearr = []
for byte in bytearrtwo:
    if len(byte)>2:
        bytearr.append(twos_complement(byte,24))

import copy

adcone = copy.deepcopy(bytearr)

f = open("data2.txt", "r")
result = ""
bytearr = []
counter = 0
for string in f:
    for word in string.split(" "):
        if "0x" in word:
            word = word.strip()
            if(len(word)<4):
                result+="0"+word[2:]
                print(result)
                counter+=1
            else:
                result+=word[2:]
                print(result)
                counter+=1
            if(counter == 4):
                bytearr.append(result)
                result = ""
                counter = 0
bytearr.append(result)

print(bytearr)
bytearrtwo = []
for byte in bytearr:
    byte = byte[:6]
    print(byte)
    bytearrtwo.append(byte)

def twos_complement(hexstr, bits):
    value = int(hexstr, 16)
    if value & (1<<bits-1):
        value -=1 << bits
    return value

bytearr = []
for byte in bytearrtwo:
    if len(byte)>2:
        bytearr.append(twos_complement(byte,24))

print(bytearr)



import matplotlib.pyplot as plt

xvals = range(0,len(bytearr), 1)

plt.plot(xvals,adcone,'r',xvals,bytearr,'g')
plt.show()


