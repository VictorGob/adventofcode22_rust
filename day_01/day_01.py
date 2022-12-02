# Python version to compare time of execution

filename = "input_d1.txt"


lines = []
with open(filename, "r") as fhand:
    lines = fhand.readlines()

datalist = []
count = 0
for l in lines:
    value = l.strip()
    if value:
        count += int(value)
    else:
        datalist.append(count)
        count = 0
# Add last count
datalist.append(count)
datalist.sort(reverse=True)
print(f"Most: {datalist[0]}")
top3 = sum(datalist[0:3])
print(f"top3: {top3}")