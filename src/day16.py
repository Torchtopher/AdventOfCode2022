from collections import namedtuple
from dijkstar import Graph, find_path
from collections import deque
from itertools import permutations
from itertools import combinations
import time
#define namedtuple
value = namedtuple("cost", "name value")
highest_extra_moves = 0


class Node:
    def __init__(self, name: str, rate: int, tunnels) -> None:
        self.name = name
        self.open = False
        self.rate = rate
        self.tunnels = tunnels

    def __repr__(self) -> str:
        return f"Node({self.name}, rate={self.rate}, leads_to={self.tunnels})"
    
    def is_open(self) -> bool:
        return self.open
    
    def open(self) -> None:
        self.open = True
    
    def get_children(self) -> list:
        return self.tunnels
    
    def get_name(self) -> str:
        return self.name
    
    def get_rate(self) -> int:
        return self.rate

    def get_children_cost(self) -> list:
        return [value(child.name, child.rate) for child in self.tunnels]

class Volcano:
    def __init__(self) -> None:
        self.starting_node = "AA"
        self.current_node = "AA"
        self.children_dict = {}
        self.nodes = {}
        self.graph = None

    def find_best_path(self) -> int:
        # find best path
        best_pressure = 0
        best_path = []
        best_states = {} # key is (location, move#) and value is pressure
        non_zero_nodes = [node for node in self.nodes.values() if node.get_rate() != 0]
        # trying something like depth first search and then keeping track of best states at each position
    
        while True:
            
            done = False
            move = 0
            pressure = 0
             
            current_path = []
            current_path.append(self.starting_node)
            possible_paths = []
            # 30 moves per path
            while not done:
                try:
                    current_path = possible_paths[-1][-1]
                except IndexError:
                    print("No paths in possible paths")

                move = 0
                branch_paths = []
                for node in non_zero_nodes:
                    move = 0
                    path = find_path(self.graph, self.current_node, node.get_name())
                    tmp_moves = len(path.nodes)
                    print(path.nodes)
                    pressure = self.nodes[node.get_name()].get_rate() * (30 - tmp_moves - 1)
                    print(f"Path is {len(path.nodes)} long and leads to {node.get_name()} with pressure {self.nodes[node.get_name()].get_rate()}")
                    # be greedy and take path with highest expected reward, will need to eventually find all paths
                    # however if we have a good baseline than it will help with pruning
                    print(f"Expected reward is {pressure}")
                    print()
                    branch_paths.append((path, pressure))
                    if best_states.get((node.get_name(), move)) is None:
                        best_states[(node.get_name(), move)] = pressure
                    else:
                        if pressure > best_states[(node.get_name(), move)]:
                            best_states[(node.get_name(), move)] = pressure

                # sort by expected reward
                branch_paths.sort(key=lambda x: x[1], reverse=True)
                possible_paths.append(branch_paths)
                break

                # pressure += self.nodes[self.current_node].get_rate()
           

with open("../inputs/day16.txt") as f:
    lines = f.read().splitlines()

nodes = []
for l in lines:
    l = l.replace("Valve ", "").replace(" has flow rate", "").replace(" tunnels lead to valves ", "").replace(" tunnel leads to valve ", "")
    #print(l)
    split = l.split("=")
    name = split[0]
    split = split[1].split(";")
    rate = int(split[0])
    split = split[1].split(",")
    tunnels = []
    for tunnel in split:
        tunnels.append(tunnel.strip())
    print(name, rate, tunnels)
    nodes.append(Node(name, rate, tunnels))


volcano = Volcano()
# initalize graph
for node in nodes:
    children = node.get_children()
    volcano.children_dict[node.name] = children
    volcano.nodes[node.name] = node

# construct graph
#graph = Graph(undirected=True)
graph = Graph()
for node in nodes:
    children = node.get_children()
    for child in children:
        graph.add_edge(node.name, child, 0)

volcano.graph = graph
# find best path
print(graph)

#volcano.find_best_path()

non_zero_nodes = [node for node in volcano.nodes.values() if node.get_rate() != 0]

 
def find_legal_modes(path, move):
    legal_moves = []
    for node in non_zero_nodes:
        cost = precomputed_paths[(path[-1], node.get_name())]
        if node.get_name() not in path and cost + move < 30:
            legal_moves.append(node.get_name())
'''
print(non_zero_nodes)
#print(len(list(permutations(non_zero_nodes))))
# find all possible paths
inital_path = ["AA"]
stopped_nodes = []
legal_moves = find_legal_modes(inital_path, stopped_nodes)
print(legal_moves)
print(stopped_nodes)
'''

# find the four lowest pressure nodes from non_zero_nodes and remove them from the list
# then find all permutations of the remaining nodes
# sort by lowest pressure
non_zero_nodes.sort(key=lambda x: x.get_rate())
# remove the four lowest pressure nodes
#non_zero_nodes = non_zero_nodes[4:]   
count = 0
best = 0

precomputed_paths = {} # key (start, end) as strings and value is how long the path is
for node in non_zero_nodes:
    # find all the paths from the start node to all other nodes
    if node.get_name() != "AA":
        path = find_path(graph, "AA", node.get_name())
        precomputed_paths[("AA", node.get_name())] = (len(path.nodes), path)
    for node2 in non_zero_nodes:
        if node.get_name() != node2.get_name():
            path = find_path(graph, node.get_name(), node2.get_name())
            precomputed_paths[(node.get_name(), node2.get_name())] = (len(path.nodes), path)
# find average path length
total = 0
for key, value in precomputed_paths.items():
    total += value[0]
print(f"Avg path len {total / len(precomputed_paths)}")
#exit(0)

#print(precomputed_paths[("TR", "FF")])

def find_legal_modes(path, move):
    legal_moves = []
    for node in non_zero_nodes:
        if node.get_name() == path[-1]:
            continue
        cost = precomputed_paths[(path[-1], node.get_name())]
        if node.get_name() not in path and cost[0] + move < 30:
            legal_moves.append(node.get_name())
    return legal_moves

# correctly evalutes test case path
def evaluate_path(path):
    global highest_extra_moves
    move = 0
    pressure = 0
    # path is a list of visited non zero nodes like ["BB", "CC", "HH"]
    for idx, node in enumerate(path):
        if idx == 0:
            p = precomputed_paths[("AA", node)]
            move += p[0]
            pressure += volcano.nodes[node].get_rate() * (30 - move)
        else:
            p = precomputed_paths[(path[idx - 1], node)]
            # -1 here to avoid double counting the end node 
            # +1 because we have to stop to activate pressure
            # yes i know -1 + 1 = 0 (probably get optimized away anyway)
            move += p[0] - 1 + 1
            if move >= 30:
                return 0
            pressure += volcano.nodes[node].get_rate() * (30 - move)
    extra_moves = find_legal_modes(path, move)
    if extra_moves and pressure > highest_extra_moves:
        print(f"extra moves {extra_moves}")
        print(f"Found legal moves for path with pressure {pressure} on move {move}")
        best_extra_move = 0
        for extra_move in extra_moves:
            move_cost = precomputed_paths[(path[-1], extra_moves[0])][0]
            if move_cost + move < 30:
                reward = volcano.nodes[extra_move].get_rate() * (30 - (move_cost + move))
                if reward > best_extra_move:
                    best_extra_move = reward
        print(f"Now we have {pressure + best_extra_move} pressure")
        if pressure + best_extra_move > pressure:
            pressure += best_extra_move    
        #move += extra_moves
        highest_extra_moves = pressure
    return pressure

#test2_path = ["DD", "BB", "JJ", "HH", "EE", "CC"]
#print(find_legal_modes(test2_path, 0))
#exit(1)

#print(evaluate_path(test_path))
print(precomputed_paths)
best = 0
# 1601 too low
# 1632 too low
# just assume that we stop at all non zero nodes on the path
# then try for 6, 5, 4, 3 and probbaly get the best result
for attempt in range(3, 8):
    print(f"Attempt {attempt}")
    for i in permutations(non_zero_nodes, attempt):
        cost = 0
        path = deque()
        for node in i:
            path.append(node.get_name())
        
        reward = evaluate_path(path)
        if reward > best:
            best = reward
            print(f"New best {best}")
        # print every 10 millionth permutation
        if count % 10_000_000 == 0:
            print(f"Permutation {count}")
        count += 1

print("DONE")
print(best)
print("DONE")
time.sleep(22)
