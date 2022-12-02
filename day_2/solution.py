
with open('gaver.txt', 'r') as f:
    gifts = list(map(lambda x: x.strip(), f.readlines()))

given = []
lines = 0
for gift in gifts:
    lines += 2 + max(len(given)-2, 0)
    if 'alv' not in gift:
        given.append(gift)
print(lines)
