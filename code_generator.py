import csv

def to_bytes_table(header, data):
    bytes_table = [0] * 256
    for row in data:
        idx_hex = row[1]
        bytes = row[3]
        bytes_table[int(idx_hex, 16)] = int(bytes)
    print(bytes_table)
    

def read_csv(file_path):
    with open(file_path, mode='r', newline='') as file:
        csv_reader = csv.reader(file)
        header = next(csv_reader)  # Skip the header row
        data = [row for row in csv_reader]
    return header, data

file_path = '6052-instructions.csv'
header, data = read_csv(file_path)

to_bytes_table(header, data)

