# Uncomment lines at the end of this file
import json, glob

debug = False

def load_frame_valid_data_from_file(path: str):
    valid_data = []

    for frame in json.load(open(path, 'r')):
        # Frame data
        data = frame['_source']['layers']['Setup Data']['usb.data_fragment'].lower().split(':')
        data = [
            '0x' + bytes_
            for bytes_ in data
        ]

        # If it is valid
        if data[13:13 + 6] == ['0xff'] * 6:
            continue # drop data filter
        else:
            valid_data.append(data)

    return valid_data

def show_bytes_sample_from_data_from_file(file_prefix, show_bits = False):
    print(f'--- Start of {file_prefix} ---')
    file = file = glob.glob(f'{file_prefix}*')[0]
    data = load_frame_valid_data_from_file(file)

    freq = {}
    for entry in data:
        for e, i in enumerate(entry):
            tupla = (i , e)
            if tupla in freq:
                freq[tupla] += 1
            else:
                freq[tupla] = 1

    lista = sorted([(e, freq[e]) for e in freq if freq[e] < 22])

    if debug:
        [print(e) for e in lista]

    freq = {}

    for e in lista:
        pos = e[0][1]
        if pos in freq:
            freq[pos] += 1
        else:
            freq[pos] = 1

    if debug:
        print('Bytes that change between slighty different calls:')
        print(list(freq.keys()))
        print('Amount of change:')
        print(list(freq.values()))

    should_check = sorted(freq.keys())
    check_range = (should_check[0], should_check[-1] + 1)

    if debug:
        print(len(data))

    # Remove all contiguous repetition
    to_pop = []
    for i in range(1, len(data)):
        if data[i] == data[i - 1]:
            to_pop.append(i)
    for i in reversed(to_pop):
        data.pop(i)

    for i in range(len(data)):
        for j in range(5 - 1, 5 + 9):
            print(data[i][j], end=' ')
        print(f' | footer: {data[i][-3:]}')
        if show_bits:
            for j in range(5, 5 + 8):
                print('{:08b}'.format(int(data[i][j], 16)), end=' ')
            print()

    print(f'--- End of {file_prefix} ---')


# Uncomment this lines
# show_bytes_sample_from_data_from_file('frequency', False)
# show_bytes_sample_from_data_from_file('dpi', False)
# show_bytes_sample_from_data_from_file('brightness', False)
