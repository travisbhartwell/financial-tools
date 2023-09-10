import csv
from pathlib import Path
from typing import Any

COLUMN_NAMES = ["date", "description", "amount"]


def write_output_file(transactions_by_fields: list[dict[str, Any]], output_path: Path) -> None:
    with output_path.open("w") as f:
        writer = csv.DictWriter(f, fieldnames=COLUMN_NAMES)
        writer.writeheader()
        writer.writerows(transactions_by_fields)
