from dijkstar import Graph, find_path
import numpy as np
import time
import dijkstar
with open('../inputs/day12.txt') as f:
    input = f.read()

arr = []

for line in input.split("\n"):
    temp = []
    for char in line:
        if char == "S":
            temp.append(-999)
        elif char == "E":
            temp.append(-1000)
        else:
            temp.append(ord(char))
    arr.append(temp)

arr.pop() # extra newline
print(arr)
nparr = np.array(arr)
print(nparr)
graph = Graph()

for (idx, val) in np.ndenumerate(nparr):
    if val == -999:
        print(val)
        print("Found start")
        val = ord('a')
        start = idx
        nparr[idx] = val
    elif val == -1000:
        end = idx
        print("Found end")
        nparr[idx] = ord('z')
# enumerate over nparr
for (idx, val) in np.ndenumerate(nparr):
    # check out of bounds
    # ultra lazy, should have used a namedtuple with .x and .y bc numpy indexing messed me up
    if not (idx[1] == 0):
        if nparr[idx[0]][idx[1] - 1] <= val + 1:
            graph.add_edge(idx, (idx[0], idx[1] - 1), nparr[idx[0]][idx[1] - 1])

    # right
    if not (idx[1] == len(nparr[0]) - 1):
        if nparr[idx[0]][idx[1] + 1] <= val + 1:
            graph.add_edge(idx, (idx[0], idx[1] + 1), nparr[idx[0]][idx[1] + 1])

    if not (idx[0] == 0):
        if nparr[idx[0] - 1][idx[1]] <= val + 1:
            graph.add_edge(idx, (idx[0] - 1, idx[1]), nparr[idx[0] - 1][idx[1]])

    if not (idx[0] == len(nparr) - 1):
        if nparr[idx[0] + 1][idx[1]] <= val + 1:
            graph.add_edge(idx, (idx[0] + 1, idx[1]), nparr[idx[0] + 1][idx[1]])

   


res = find_path(graph, start, end)
print(len(res.nodes) - 1)

shortest = 999999

# yeah its a lot of wasted work but it only takes a few seconds
for (idx, val) in np.ndenumerate(nparr):
    if val == ord('a'):
        #print(idx)
        try:
            res = find_path(graph, idx, end)
        except dijkstar.algorithm.NoPathError:
            continue
        if len(res.nodes) - 1 < shortest:
            shortest = len(res.nodes) - 1

print(shortest)