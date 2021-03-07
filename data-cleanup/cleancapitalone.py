#!/usr/bin/env python3

"""
Transform transactions downloaded from Capital One 360 Account in CSV format.

CSV headers:
1   Account Number
2   Transaction Date
3   Transaction Amount
4   Transaction Type
5   Transaction Description
6   Balance
"""

import csv
import os.path
import re
import sys
from pprint import pprint

from clean import COLUMN_NAMES, write_output_file


def date_clean(input_value):
    month = input_value[0:2]
    day = input_value[3:5]
    year = input_value[6:8]

    return f"20{year}-{month}-{day}"


def description_clean(input_value):
    return f'"{input_value}"'


def amount_clean(input_value):
    # The rules for the financial importer are designed first around
    # a credit card statement, where the amount of a transaction is
    # subtracted to show what we owe. This is the opposite.
    if input_value[0] == "-":
        return input_value[1:]
    else:
        return f"-{input_value}"


COLUMN_MAP = {
    "date": "Transaction Date",
    "description": "Transaction Description",
    "amount": "Transaction Amount",
}

COLUMN_CLEAN_FUNCTIONS = {
    column: globals()[f"{column}_clean"] for column in COLUMN_NAMES
}


def load_input(input_filename):
    with open(input_filename) as f:
        reader = csv.DictReader(f)

        return [row for row in reader]


def transform_row(row):
    return {
        column: COLUMN_CLEAN_FUNCTIONS[column](row[COLUMN_MAP[column]])
        for column in COLUMN_NAMES
    }


def transform_rows(input_content):
    return [transform_row(row) for row in input_content]


def main(input_filename, output_filename):
    input_content = load_input(input_filename)

    transformed_content = transform_rows(input_content)

    write_output_file(transformed_content, output_filename)

    return 0


if __name__ == "__main__":
    if len(sys.argv[1:]) < 2:
        print("Usage: \n")
        print(f"\t{os.path.basename(__file__)} <input_file> <output_file>")
        sys.exit(1)

    try:
        result = main(sys.argv[1], sys.argv[2])
    except Exception as e:
        result = 1
        print(f"{str(e)}", file=sys.stderr)

    sys.exit(result)
