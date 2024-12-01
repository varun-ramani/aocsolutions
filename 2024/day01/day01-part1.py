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
    answer = sum([
        abs(val2 - val1)
        for val1, val2 
        in zip(
            sorted(first),
            sorted(second)
        )
    ])

    print(answer)

main()