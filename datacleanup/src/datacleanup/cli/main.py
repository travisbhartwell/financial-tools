from pathlib import Path
from typing import Annotated

import typer

from datacleanup.clean import capitalone, chasevisa

app = typer.Typer()


@app.command()
def chase_visa(
    input_file_path: Annotated[
        Path, typer.Option(exists=True, file_okay=True, dir_okay=False, readable=True, resolve_path=True)
    ],
    output_file_path: Annotated[Path, typer.Option(exists=False, writable=True, resolve_path=True)],
    year: Annotated[str, typer.Option("--year", "-y")],
    *,
    is_jan: Annotated[bool, typer.Option("--january-statement/ ")] = False,
):
    chasevisa.do_clean(input_file_path, output_file_path, year, is_jan=is_jan)


@app.command()
def capital_one(
    input_file_path: Annotated[
        Path, typer.Option(exists=True, file_okay=True, dir_okay=False, readable=True, resolve_path=True)
    ],
    output_file_path: Annotated[Path, typer.Option(exists=False, writable=True, resolve_path=True)],
):
    capitalone.do_clean(input_file_path, output_file_path)
