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
from pathlib import Path

from datacleanup.clean.common import COLUMN_NAMES, write_output_file


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

COLUMN_CLEAN_FUNCTIONS = {column: globals()[f"{column}_clean"] for column in COLUMN_NAMES}


def load_input(input_file_path: Path) -> list[dict[str, str]]:
    with input_file_path.open() as f:
        reader = csv.DictReader(f)
        return list(reader)


def transform_row(row):
    return {column: COLUMN_CLEAN_FUNCTIONS[column](row[COLUMN_MAP[column]]) for column in COLUMN_NAMES}


def transform_rows(input_content):
    return [transform_row(row) for row in input_content]


def do_clean(input_file_path: Path, output_file_path: Path):
    input_content = load_input(input_file_path)

    transformed_content = transform_rows(input_content)

    write_output_file(transformed_content, output_file_path)
