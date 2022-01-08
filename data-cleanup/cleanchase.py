#!/usr/bin/env python3

import csv
import os.path
import re
import sys

from clean import COLUMN_NAMES, write_output_file


def load_input(input_filename):
    with open(input_filename) as f:
        content = [line.strip() for line in f]

    return content


TRANSACTION_FIELDS_RE = re.compile(
    r"^(?P<month>\d{2})\/(?P<day>\d{2})[ ,](?P<description>.*),(?P<amount>[^,]+)$"
)


def transaction_fields(line: str, year: str):
    month = line[0:2]
    day = line[3:5]

    if line[6] == '"':
        end = line.index('"', 7) + 1
    else:
        end = line.index(",", 7)

    if line[5] in [',', ' ', '"']:
        desc_start = 6
    else:
        desc_start = 5

    description = f"{line[desc_start:end]}"
    amount = line[end + 1 :].replace('"', "").replace(",", "")

    return {
        "date": f"{year}-{month}-{day}",
        "description": description,
        "amount": amount,
    }

def cleanup_commas(line: str):
    return line.replace(",,", ",")


START_QUOTE_RE = re.compile(r'^"(?P<date>\d{2}\/\d{2}) (?P<rest>[^"]+".*)')


def fix_quoting(line: str):
    if match := START_QUOTE_RE.match(line):
        date_part = match.group("date")
        rest_part = match.group("rest")
        return f'{date_part},"{rest_part}'
    else:
        return line


TRANSACTION_LINE_RE = re.compile(r"^\d{2}")


def is_transaction_line(line: str):
    return TRANSACTION_LINE_RE.match(line) is not None


def check_lines_contains_string(search, log, lines):
    if any([search in line for line in lines]):
        print(f"Search '{search}' found at step: '{log}'")

        for line in lines:
            if search in line:
                print(f"Line: '{line}'")


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
    transactions_by_fields = [
        transaction_fields(line, year) for line in transaction_lines
    ]

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
