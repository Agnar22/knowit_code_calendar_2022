import copy

def solve(letter, dictionary):
    letter = copy.copy(letter)
    if len(letter) == 0:
        return ''
    for idx in range(1, len(letter)+1):
        if letter[:idx] in dictionary.keys():
            res = solve(letter[idx:], dictionary)
            if res is not None:
                return (dictionary[letter[:idx]] + ' ' + res).strip()
    return None


dictionary = {}
with open('dictionary.txt', 'r') as f:
    for line in f.readlines():
        dictionary[line.split(',')[0]] = line.split(',')[1].strip()
with open('letter.txt', 'r') as f:
    letter = f.read()
print(len(solve(letter, dictionary)))
