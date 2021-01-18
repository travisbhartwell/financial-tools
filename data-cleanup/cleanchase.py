#!/usr/bin/env python3

import csv
import os.path
import re
import sys

COLUMN_NAMES = [
    "Date",
    "Description",
    "Amount"
]

def load_input(input_filename):
    with open(input_filename) as f:
        content = [line.strip() for line in f]

    return content

TRANSACTION_FIELDS_RE = re.compile(r"^(?P<month>\d{2})\/(?P<day>\d{2})[ ,](?P<description>.*),(?P<amount>[^,]+)$")

def transaction_fields(line: str, year: str):
    month = line[0:2]; day = line[3:5]
    
    if line[6] == '"':
        end = line.index('"', 7) + 1
    else:
        end = line.index(',', 7)

    description = f"{line[6:end]}"
    amount = line[end + 1:].replace('"', '').replace(",", "")

    return {
        "Date": f"{year}-{month}-{day}", 
        "Description": description, 
        "Amount": amount
    }

def cleanup_commas(line: str):
    return line.replace(",,", ",")

def fix_quoting(line: str):
    # "10/27 I V Y LABS, INC. HTTPSWWW.TALK CAA"
    if line[0] == '"' and ("I V Y" in line):
        return line[1:6] + ',"' + line[7:]
    else:
        return line

TRANSACTION_LINE_RE = re.compile(r"^\d{2}")

def is_transaction_line(line: str):
    return TRANSACTION_LINE_RE.match(line) is not None

def write_output_file(transactions_by_fields, output_filename):
    with open(output_filename, "w") as f:
        writer = csv.DictWriter(f, fieldnames=COLUMN_NAMES)
        writer.writeheader()
        writer.writerows(transactions_by_fields)

def main(input_filename, output_filename, year):
    # Load full input
    input_content = load_input(input_filename)

    # First clean up lines so they match:
    input_content = [fix_quoting(line) for line in input_content]

    # Filter down to just lines containing transactions
    transaction_lines = [line for line in input_content if is_transaction_line(line)]

    # First, change double comma to single
    transaction_lines = [cleanup_commas(line) for line in transaction_lines]

    # Grab fields
    transactions_by_fields = [transaction_fields(line, year) for line in transaction_lines]

    # Write CSV
    write_output_file(transactions_by_fields, output_filename)

    return 0

if __name__ == "__main__":
    if len(sys.argv[1:]) < 3:
        print("Usage: \n")
        print(f"\t{os.path.basename(__file__)} <input_file> <output_file> <year>")
        sys.exit(1)

    try:
        result = main(sys.argv[1], sys.argv[2], sys.argv[3])
    except Exception as e:
        result = 1
        print(f"{str(e)}", file=sys.stderr)

    sys.exit(result)
    