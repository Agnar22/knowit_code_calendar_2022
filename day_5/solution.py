import numpy as np


def check_cycle(num):
    c = np.arange(num)
    for i in range(13):
        l = c[:num//2]
        r = c[num//2:]
        c = np.zeros(c.shape, dtype=int)
        c[np.arange(0, num-1, 2)] = l
        c[np.arange(1, num, 2)] = r
        if all(c == np.arange(num)):
            return i + 1
    return -1


i = 2
while True:
    c = check_cycle(i)
    if c == 13:
        print(i)
        break
    i+= 2
