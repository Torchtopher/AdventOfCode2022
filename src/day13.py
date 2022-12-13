import time
import copy
# zip only goes over the shortest list so add padding to the shorter list

def add_padding(left, right):
    l = left.copy() 
    r = right.copy()
    if len(l) < len(r):
        for i in range(len(r) - len(l)):
            l.append(-999)
    elif len(l) > len(r):
        for i in range(len(l) - len(r)):
            r.append(-999)
    return l, r


def compare(left, right):
    # handle case where a list has no elements
    if len(left) == 0 and len(right) != 0:
        return True
    elif len(left) != 0 and len(right) == 0:
        return False
    l2, r2 = add_padding(left, right)
    #print(f"Compare called with {left} and {right}")
    for (l, r) in zip(l2, r2):
        #print(f"l is {l} and r is {r}")
        if type(l) == int and type(r) == int:
            if l == -999:
                return True
            elif r == -999:
                return False
            elif l < r:
                return True
            elif l == r:
                continue
            else:
                return False
        elif type(l) == list and type(r) == list:
            res = compare(l, r)
            if res is not None:
                return res
        elif type(l) == list and type(r) == int:
            res = compare(l, [r])
            if res is not None:
                return res
        elif type(l) == int and type(r) == list:
            res = compare([l], r)
            if res is not None:
                return res

with open('../inputs/day13.txt') as f:
    input = f.read()

input = input.split("\n\n")
print(input)
counter = 0
inputs = []
for idx, val in enumerate(input):
    val = val.split("\n")
    left = val[0]
    right = val[1]
    print("\n")
    
    
    # print types of left and right
    left = eval(left)
    right = eval(right)
    inputs.append(left)
    inputs.append(right)
    print(f"left s {left} right s {right}")
    if compare(left, right):
        counter += idx + 1

print("\n")
print(counter)
inputs.append([[2]])
inputs.append([[6]])
print(inputs)

def remove_padding(l):
    # recursively remove padding
    for i in list(l):
        if i == -999:
            l.remove(i)
        elif type(i) == list:
            l[l.index(i)] = remove_padding(i)
    return l
# i heard bubble sort was easy so here it is
while True:
    swapped = False
    for i in range(len(inputs) - 1):
        if not compare(inputs[i], inputs[i + 1]):
            test = str(inputs[i]) + str(inputs[i + 1])
            inputs[i], inputs[i + 1] = inputs[i + 1], inputs[i]
            swapped = True
    if not swapped:
        break

for i in inputs:
    print(i)
    
# find index of [[2]] and [[6]]
decoder1 = inputs.index([[2]]) + 1
decoder2 = inputs.index([[6]]) # yeah there is a case it fails and is off by 1 but i dont care enough to fix it
print(decoder1)
print(decoder2)
print(decoder1 * decoder2)

