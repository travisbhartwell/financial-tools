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


def date_clean(input_value, _row):
    month = input_value[0:2]
    day = input_value[3:5]
    year = input_value[6:8]

    return f"20{year}-{month}-{day}"


def description_clean(input_value, _row):
    return f'"{input_value}"'


TRANSACTION_TYPE_FIELD = "Transaction Type"
DEBIT = "Debit"
CREDIT = "Credit"


def amount_clean(input_value, row):
    if row[TRANSACTION_TYPE_FIELD] == CREDIT:
        return f"-{input_value}"
    else:
        return input_value


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
    return {column: COLUMN_CLEAN_FUNCTIONS[column](row[COLUMN_MAP[column]], row) for column in COLUMN_NAMES}


def transform_rows(input_content):
    return [transform_row(row) for row in input_content]


def do_clean(input_file_path: Path, output_file_path: Path):
    input_content = load_input(input_file_path)

    transformed_content = transform_rows(input_content)

    write_output_file(transformed_content, output_file_path)
