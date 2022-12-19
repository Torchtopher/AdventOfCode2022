import time
# defaultdict
from collections import defaultdict
'''
For each beacon, calculate the Manhattan distance to the sensor
      Start at 

'''



from collections import namedtuple
with open("../inputs/day15.txt") as f:
    input = f.read()

# make point that is named tuple
Point = namedtuple("Point", "x y")
BeaconSensor = namedtuple("BeaconSensor", "sensor beacon")

class BeaconZone:
    def __init__(self):
        self.beacons_and_sensor = []
        # make a dict of excluded ranges with a default value of []
        self.excluded_ranges = defaultdict(list)

        # make a set of excluded points 
        self.excluded_points = set()
        self.on_2000000 = 0

    def __repr__(self):
        return f"BeaconZone(x={self.x}, y={self.y}, distance={self.distance})"

    def find_distress_signal(self):
        # find the point that has x <= 0 and y <= 0
        # and the point that has x >= 20 and y >= 20
        pass

    def calc_exclusions(self):
        for idx, beacon_sensor in enumerate(self.beacons_and_sensor):
            print(f"Beacon {idx}")
            beac = beacon_sensor.beacon
            sens = beacon_sensor.sensor
            man_dist = abs(beac.x - sens.x) + abs(beac.y - sens.y)
            #print(f"Manhattan distance between {beac} and {sens} is {man_dist}")
            s_man_dist = man_dist
            for y in range(sens.y - s_man_dist - 1, sens.y + s_man_dist + 2):
                if True:
                    for x in range(sens.x - s_man_dist - 1, sens.x + s_man_dist + 2):
                        #print(f"X {x}, Y {y}")
                        #print(f"Sens x {sens.x}, Sens y {sens.y}")
                        # check manhatten dist for points that are in the row we care about
                        temp_man_dist = abs(x - sens.x) + abs(y - sens.y)
                        if temp_man_dist <= s_man_dist:
                            if beac.y == y and beac.x == x:
                                #print("Beacon")
                                #time.sleep(1)
                                continue
                            #print(f"Temp man dist {temp_man_dist}")
                            #print(f"Adding {x}, {y} to excluded ranges")
                            #time.sleep(1)
                            # add point to the set of excluded points
                            self.excluded_points.add((x, y))

    def calc_exclusions_pt2_attempt_2(self):
        try:
            for idx, beacon_sensor in enumerate(self.beacons_and_sensor):
                print(f"Beacon {idx}")
                beac = beacon_sensor.beacon
                sens = beacon_sensor.sensor
                man_dist = abs(beac.x - sens.x) + abs(beac.y - sens.y)
                #print(f"Manhattan distance between {beac} and {sens} is {man_dist}")
                s_man_dist = man_dist
                for y in range(0, 4000000):
                    print(f"Y {y}")
                    if True:
                        for x in range(0, 4000000):
                            #print(f"X {x}, Y {y}")
                            #print(f"Sens x {sens.x}, Sens y {sens.y}")
                            # check manhatten dist for points that are in the row we care about
                            temp_man_dist = abs(x - sens.x) + abs(y - sens.y)
                            if temp_man_dist <= s_man_dist:
                                if beac.y == y and beac.x == x:
                                    #print("Beacon")
                                    #time.sleep(1)
                                    continue
                                #print(f"Temp man dist {temp_man_dist}")
                                #print(f"Adding {x}, {y} to excluded ranges")
                                #time.sleep(1)
                                # add point to the set of excluded points
                                self.excluded_points.add((x, y))
        except KeyboardInterrupt:
            print(f"Total points {len(self.excluded_points)}")

    def calc_exclusions_pt2(self):
        debug = False
        for idx, beacon_sensor in enumerate(self.beacons_and_sensor):
            beac = beacon_sensor.beacon
            sens = beacon_sensor.sensor
            print()
            print(f"Beacon {idx} at {beac}")
            print(f"Sensor at {sens}")
            
            man_dist = abs(beac.x - sens.x) + abs(beac.y - sens.y)
            print(f"Manhattan distance is {man_dist}")
            s_man_dist = man_dist
            # we are going to make a range of points that are excluded to make it easier to find the real point
            for y in range(0, 21):
                if y == 11:
                    debug = True
                else:
                    debug = False
                s_range = None
                e_range = None
                ranges = []
                for x in range(0, 21):
                    if debug:
                        #print(f"X {x}, Y {y}")
                        #print(f"Sens x {sens.x}, Sens y {sens.y}")
                        pass
                    # check manhatten dist for points that are in the row we care about
                    temp_man_dist = abs(x - sens.x) + abs(y - sens.y)
                    if temp_man_dist <= s_man_dist:
                        if beac.y == y and beac.x == x:
                            #print("Beacon")
                            #time.sleep(1)
                            continue
                        #print(f"Temp man dist {temp_man_dist}")
                        #print(f"Adding {x}, {y} to excluded ranges")
                        #time.sleep(1)
                        if s_range is None:
                            s_range = x
                            e_range = x
                        if x - 1 == e_range:
                            e_range = x
                        else:
                            print(f"s range {s_range}, e {e_range}")
                            ranges.append(range(s_range, e_range + 1))
                            s_range == None
                            e_range = None
                if s_range and e_range:
                    ranges.append(range(s_range, e_range + 1))
                for r in ranges:
                    if s_range != e_range:
                        self.excluded_ranges[y].append(r)
                        if debug:
                            print(f"Added range {r}")
                            print(self.excluded_ranges)
                            print()
                            #time.sleep(2)

    def find_continous_ranges(self):
        # find ranges that are continuous from 0 to 20
        # loop over keys and values of excluded ranges
        for y, ranges in self.excluded_ranges.items():
            if y > 20 or y < 0:
                continue
            print(f"Y {y}")
            #print(f"Ranges {ranges}")
            farthest = None
            current_range = None
            done = False
            # loop over ranges
            while not done:
                #print(f"Far {farthest}")
                for r in ranges:
                    #print(r)
                    if r.start <= 0 and current_range is None:
                        #print("Found starting range")
                        current_range = r
                        farthest = r.stop
                        continue
                    if farthest is not None:
                        if farthest + 1 in r:
                            current_range = r
                            farthest = r.stop
                        
                            if farthest >= 20:
                                done = True
                #print("One loop")
                #time.sleep(2)
            
                
    def visualize(self):
        # visualize the excluded ranges as # and the rest as .
        # also add the beacons and sensors and B and S
        for i in range(-10, 40):
            for j in range(-10, 50):
                # check if this is a beacon or sensor
                for beacon_sensor in self.beacons_and_sensor:
                    if beacon_sensor.beacon.x == j and beacon_sensor.beacon.y == i:
                        print("B", end="")
                        break
                    elif beacon_sensor.sensor.x == j and beacon_sensor.sensor.y == i:
                        print("S", end="")
                        break
                if self.excluded_ranges.get(i) is None:
                    print(".", end="")
                else:
                    for r in self.excluded_ranges[i]:
                        if j in r:
                            print("#", end="")
                            break
                    else:
                        print(".", end="")
            print("")

beacons_sensors = []
for line in input.splitlines():
    #print("New beacon")
    l = []
    for part in line.split(":"):
        part = part.replace("Sensor at x=", "")
        part = part.replace("closest beacon is at x=", "")
        part = part.replace("y=", "")
        part = part.replace(" ", "")
        num = part.split(",")
        pt = Point(int(num[0]), int(num[1]))
        
        #print(pt)
        l.append(pt)
    beacons_sensors.append(BeaconSensor(l[0], l[1]))

#print(beacons_sensors)
zone = BeaconZone()
zone.beacons_and_sensor = beacons_sensors
zone.calc_exclusions_pt2_attempt_2()    
# sort the set by x
zone.excluded_points = sorted(zone.excluded_points, key=lambda x: x[0])
print(zone.excluded_points)
print(len(zone.excluded_points))
# hold all points from (0, 0) to (20, 20) lazily with a set
all_points = set()
for x in range(0, 4000000):
    for y in range(0, 4000000):
        all_points.add((x, y))
# subtract the excluded points from the all points
all_points = all_points - set(zone.excluded_points)
# remove all beacons from set
for beacon_sensor in zone.beacons_and_sensor:
    try:
        all_points.remove((beacon_sensor.beacon.x, beacon_sensor.beacon.y))
    except KeyError:
        pass
print(all_points)

#zone.calc_exclusions_pt2()
#zone.find_continous_ranges()
#print(zone.excluded_ranges[11])
