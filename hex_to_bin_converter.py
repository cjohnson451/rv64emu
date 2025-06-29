hex_string = """
13 05 50 00
13 06 c0 00
3b 84 c5 40
23 2e 81 fe
83 24 c1 ff
13 05 10 00
63 14 94 00
6f 00 40 00
13 05 00 00
67 00 00 00
"""

output_filename = "comprehensive_test.bin"

print(f"Preparing to create binary file: {output_filename}")

hex_values = hex_string.strip().split()

try:
    byte_values = [int(h, 16) for h in hex_values]
    print(f"Successfully parsed {len(byte_values)} bytes.")
except ValueError as e:
    print(f"Error: Invalid hexadecimal value found in string. {e}")
    exit(1)

byte_array = bytearray(byte_values)

try:
    with open(output_filename, "wb") as binary_file:
        binary_file.write(byte_array)
    print(f"Success! Binary file '{output_filename}' created.")
    print(f"Total size: {len(byte_array)} bytes.")
except IOError as e:
    print(f"Error writing to file: {e}")

