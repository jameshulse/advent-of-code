import string

def is_reactive(polymer: string):
    if polymer[0].lower() != polymer[1].lower():
        return False
    
    return polymer[0].swapcase() == polymer[1]

def reduce_polymers(input: str):
    result = input
    i = 0

    while i < len(result) - 1:
        polymer = result[i:i+2]

        if is_reactive(polymer):
            result = result[:i] + result[i + 2:]
            i = max(i - 1, 0)
        else:
            i += 1

    return result

if __name__ == "__main__":
    input = open('input.txt').read()

    reduced = reduce_polymers(input)

    print('Remaining polymer length:', len(reduced))