import sys
from math import gcd

# pollards_rho algorithm used to factorize a number n = p * q
def pollards_rho(n):
    def f(x):
        return (x*x + 1) % n

    x = 2
    y = 2
    d = 1

    while d == 1:
        x = f(x)
        y = f(f(y))
        d = gcd(abs(x - y), n)

    if d == n:
        return None
    else:
        return d

# takeing a number as input and returns a list of its factors.
def factorize(number):
    factors = []
    while number > 1:
        factor = pollards_rho(number)
        if factor is None:
            factors.append(number)
            break
        else:
            factors.append(factor)
            number //= factor
    return factors


# taking a number and its factors as input and returns a 
# formatted string representing the factorization in the desired format
def format_factorization(number, factors):
    factors_str = '*'.join(str(factor) for factor in factors)
    return f"{number}={factors_str}"

def main():
    if len(sys.argv) != 2:
        print("Usage: factors <file>")
        return
    
    filename = sys.argv[1]
    
    try:
        with open(filename, 'r') as file:
            numbers = [int(line.strip()) for line in file]
    except FileNotFoundError:
        print(f"File '{filename}' not found.")
        return
    
# this dictionary are created to store the factorizations of the numbers
# The dictionary will map each number to its factorization

    factorizations = {}
    
    for number in numbers:
        if number not in factorizations:
            factorizations[number] = factorize(number)
    
    for number, factors in factorizations.items():
        factorization = format_factorization(number, factors)
        print(factorization)

if __name__ == '__main__':
    main()

