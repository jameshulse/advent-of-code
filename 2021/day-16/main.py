def parse_input(input) -> str:
    return ''.join([bin(int(ch, 16))[2:].zfill(4) for ch in input])

def part_one(packet: str, versions: list):
    if packet == len(packet) * '0':
        return

    version = int(packet[:3], 2)
    type_id = int(packet[3:6], 2)

    versions.append(version)

    if type_id == 4: # handle literal
        i = 0
        literal_parts = []

        while True:
            literal_part = packet[6 + i * 5:6 + i * 5 + 5]

            literal_parts.append(literal_part)

            if literal_part[0] == "0":
                break

            i += 1

        part_one(packet[6 + i * 5 + 5:], versions)
    elif packet[6] == "0": # handle sub packets of length 15
        length = int(packet[7:22], 2)

        # part_one(packet[22:22+length], versions)
        part_one(packet[22:], versions)
    else: # handle count of sub packets
        sub_count = int(packet[7:18], 2)

        part_one(packet[18:], versions)



# 011 000 1 00000000010
#                      000 000 0 000000000010110 0001000101010110001011001000100000000010000100011000111000110100

# Run part 
input = open('src/input').readline()
versions = []

part_one(parse_input(input), versions)

print("Result", sum(versions))

# Run part one tests
# test_cases = [
#     ("620080001611562C8802118E34", 12),
#     ("D2FE28", 6),
#     ("8A004A801A8002F478", 16),
#     ("C0015000016115A2E0802F182340", 23),
#     ("A0016C880162017C3686B18A3D4780", 31),
# ]

# for (packet, expected_sum) in test_cases:
#     print("Testing", packet)

#     versions = []

#     part_one(parse_input(packet), versions)

#     if sum(versions) != expected_sum:
#         print("Failed on", packet, "Expected", expected_sum, "Got", sum(versions))
#         break