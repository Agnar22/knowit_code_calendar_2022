
actions = {}
with open('slemmehandlinger.txt', 'r') as f:
    for line in f.readlines():
        a, v = line.split(':')
        actions[a] = float(v)

tot_votes = {}
with open('stemmer.txt', 'r') as f:
    for line in f.readlines():
        a, v = line.split(':')
        min_val = 1
        for x in a.split(','):
            min_val = min(min_val, actions.get(x, 1))
        tot_votes[v] = tot_votes.get(v, 0) + min_val
sorted_votes = sorted([i for i in tot_votes.values()])
print(sorted_votes[-1] - sorted_votes[-2])






