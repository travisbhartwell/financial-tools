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


def transaction_fields(line: str, year: str, is_jan: bool):
    month = line[0:2]

    if is_jan:
        year = str(int(year) - 1) if month == "12" else year

    day = line[3:5]
    end = line.index('"', 7) + 1 if line[6] == '"' else line.index(",", 7)
    desc_start = 6 if line[5] in [',', ' ', '"'] else 5

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


def main(input_filename, output_filename, year, is_jan):
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
        transaction_fields(line, year, is_jan) for line in transaction_lines
    ]

    # Write CSV
    write_output_file(transactions_by_fields, output_filename)

    return 0


if __name__ == "__main__":
    args = sys.argv[1:]

    if len(args) < 3:
        print("Usage: \n")
        print(f"\t{os.path.basename(__file__)} <input_file> <output_file> <year> [--january-statement]")
        sys.exit(1)

    # We handle January statements different as they straddle the year
    is_jan = len(args) > 3 and args[3] == "--january-statement"

    print(f"Is January: {is_jan}")

    try:
        result = main(args[0], args[1], args[2], is_jan)
    except Exception as e:
        result = 1
        print(f"{str(e)}", file=sys.stderr)

    sys.exit(result)
