'''
DAY 24: Arithmetic Logic Unit
The ALU is a four-dimentional processing unit with 4 registers w, x, y, z and
six instruction:
 - inp a - Reads an input value to the register a
 - add a b - Add the value of a to the value of b, then store the result in
     variable a. b can be a numeric literal.
 - mul a b - Multiply the value of a to the value of b, then store the result
     in variable a. b can be a numeric literal.
 - div a b - Divide the value of a by the value of b, then store the floor in
     variable a. b can be a numeric literal.
 - eql a b - Compare a and b if they are equal store 1 in a store 0 otherwise

The input is a program that reads 14 inputs. The input is considered valid if
at the end the z register contains 0.
'''
def read_instruction_blocks(filepath: str) -> list:
    instruction_blocks = []
    with open(filepath) as fp:
        for line in fp:
            cleared = line.strip()
            if cleared.startswith('inp'):
                instruction_blocks.append([])
            instruction_blocks[-1].append(cleared)
    return instruction_blocks


def get_block_type(block):
    return 'mult' if block[4] == 'div z 1' else 'div'


def get_block_info(block):
    block_type = get_block_type(block)
    if block_type == 'mult':
        z_constant = int(block[15].split(' ')[-1])
        return block_type, z_constant
    eq_constant = int(block[5].split(' ')[-1])
    return block_type, eq_constant


def main():
    blocks = read_instruction_blocks('input24.txt')
    nums = [0] * 14
    mult_stack = []
    for index, block in enumerate(blocks):
        block_type, constant = get_block_info(block)
        if block_type == 'mult':
            mult_stack.append((index, constant))
        else:
            mult_index, mult_constant = mult_stack.pop()
            offset = mult_constant + constant # mult + offset = div
            if offset <= 0:
                nums[mult_index] = 9
                nums[index] = 9 + offset
            else:
                nums[mult_index] = 9 - offset
                nums[index] = 9
        print(f'{nums=}{mult_stack=}')
    print(''.join(map(str, nums)))
    nums = [0] * 14
    mult_stack = []
    for index, block in enumerate(blocks):
        block_type, constant = get_block_info(block)
        if block_type == 'mult':
            mult_stack.append((index, constant))
        else:
            mult_index, mult_constant = mult_stack.pop()
            offset = mult_constant + constant # mult + offset = div
            if offset <= 0:
                nums[mult_index] = 1 - offset
                nums[index] = 1
            else:
                nums[mult_index] = 1
                nums[index] = 1 + offset
        print(f'{nums=}{mult_stack=}')
    print(''.join(map(str, nums)))


if __name__ == '__main__':
    main()
