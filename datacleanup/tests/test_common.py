import csv
from pathlib import Path

import pytest

from datacleanup.clean.common import write_output_file


@pytest.fixture
def transactions():
    return [
        {"date": "1/1/2020", "description": "Expense", "amount": 100},
        {"date": "2/1/2020", "description": "Income", "amount": 200},
    ]


@pytest.fixture
def output_path(tmp_path):
    return Path(tmp_path / "output.csv")


def test_writes_header(transactions, output_path):
    # Act
    write_output_file(transactions, output_path)

    # Assert
    with open(output_path) as f:
        header = next(csv.reader(f))
        assert header == ["date", "description", "amount"]


def test_writes_rows(transactions, output_path):
    # Act
    write_output_file(transactions, output_path)

    # Assert
    with open(output_path) as f:
        rows = list(csv.reader(f))
        assert rows[1] == ["1/1/2020", "Expense", "100"]
        assert rows[2] == ["2/1/2020", "Income", "200"]


def test_empty_input(output_path):
    # Arrange
    transactions = []

    # Act
    write_output_file(transactions, output_path)

    # Assert
    with open(output_path) as f:
        rows = list(csv.reader(f))
        assert len(rows) == 1  # Only header


def test_invalid_path(transactions):
    output_path = Path("invalid/path")

    with pytest.raises(IOError):
        write_output_file(transactions, output_path)


# Test that clean.capitalon.do_clean writes an output file
def test_capitalone_do_clean():
    """
    Test that output file is generated.
    """
