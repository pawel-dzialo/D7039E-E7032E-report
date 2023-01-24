import matplotlib.pyplot as plt

seizure_file = open("output", "r")

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
        elif c ==',':
            data[i].append(int(tempstr))
            tempstr = ''
            if len(data[i])==20000:
                break
            i+=1
        else:
            tempstr+=c
    return data
    
data = parsefile(seizure_file)
fig, ((ax1,ax2,ax3),(ax4,ax5,ax6),(ax7,ax8,ax9)) = plt.subplots(3,3)
for i, ax in enumerate(fig.get_axes()):
    if i>7:
        continue
    else:
        ax.plot([element * 1/256 for element in range(0,len(data[i]))],data[i],'b',[element*1/256 for element in range(0,len(data[i]))],[3156]*len(data[i]),'r')
    ax.label_outer()
plt.show()



