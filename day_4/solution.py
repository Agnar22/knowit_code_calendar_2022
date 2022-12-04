available_digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']


def number_to_base(num, base):
    if num == 0:
        return '0'
    digits = []
    while num:
        digits.append(available_digits[int(num%base)])
        num //= base
    return ''.join(digits[::-1])



def is_palindrome(num):
    return str(num) == str(num)[::-1]

multi_palindromes = 0
for i in range(1, 10000000):
    num_palindromes = 0
    for base in range(2, 17):
        if is_palindrome(number_to_base(i, base)):
            num_palindromes+=1
            if num_palindromes == 3:
                break
    if num_palindromes == 3:
        multi_palindromes += i
print(multi_palindromes)
