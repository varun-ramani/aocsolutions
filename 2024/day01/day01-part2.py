from collections import Counter

def read_input():
    with open('./input.txt') as file:
        data = [
            [int(value) 
             for value in 
             line.split()]
            for line in 
            file
        ]

    first, second = zip(*data)

    return first, second

def main():
    first, second = read_input()
    second_counter = Counter(second)

    answer = sum([
        val1*second_counter.get(val1, 0)
        for val1
        in first
    ])

    print(answer)

main()