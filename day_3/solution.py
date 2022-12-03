gifts = []
with open('pakker.csv', 'r') as f:
    f.readline()
    for gift in f.readlines():
        gifts.append(sorted(list(map(int, gift.split(',')))))

length = 0
for gift in gifts:
    alternatives = []
    if 2*(gift[2]+gift[0]) <= 110:
        alternatives.append(gift[1]+gift[0])
    if 2*(gift[1]+gift[0]) <= 110:
        alternatives.append(gift[2]+gift[0])
    if gift[2] + gift[0] <= 110:
        alternatives.append(2*(gift[1] + gift[0]))
    if gift[1] + gift[0] <= 110:
        alternatives.append( 2*(gift[2] + gift[0]))
    if len(alternatives) == 0:
        print('Impossible', gift)
    length += min(alternatives)

print(length)
