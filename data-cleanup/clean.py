import csv

COLUMN_NAMES = ["date", "description", "amount"]


def write_output_file(transactions_by_fields, output_filename):
    with open(output_filename, "w") as f:
        writer = csv.DictWriter(f, fieldnames=COLUMN_NAMES)
        writer.writeheader()
        writer.writerows(transactions_by_fields)
