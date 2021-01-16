# Financial Importer

This is a rewrite of a Python program I lost (kudos to not having proper backups in place) that I used to 
import various CSV files into my ledger.

I know that other such solutions exist, so this exists largely as a learning tool as well as scratching my own itch.

One major departure will be that this program will operate based on configuration files. My Python script had all of the rules for matching transactions and what to generate from them hard-coded. This time, in order to make it possible to share the code but still keep my financial particulars private, the rules will come from a configuration file that is loaded at runtime.

# Concepts

* **Named Input File Type**: A description of a CSV file type  -- say, a CSV download from your credit card account -- and the rules for how to generate Ledger postings from potential rows in this file. This definition includes column definitions, matching rules, etc.

# Usage

For simplicity, the type of the file format is specified as a command-line argument. Perhaps in the future the input file type will be auto-detected based on the configuration.

# Configuration File Format

I'm undecided on what format to use (TOML<sup id="a1">[1](#f1)</sup>, YAML<sup id="a2">[2](#f2)</sup>, JSON<sup id="a3">[3](#f3)</sup>, or Dhall<sup id="a4">[4](#f4)</sup>), regardless the principles will be the same.

## Named Format Description

A configuration file can define one or more named input file formats. An input file format definition consists of the following:

* A statement row definition: a mapping of named (for files with a header row) or indexed columns (or combination of columns) corresponding to the following:
   * Transaction Description
   * Transaction Date
   * Transaction Amount




---
<b id="f1">1</b> [TOML](https://toml.io/en/) [↩](#a1)

<b id="f2">2</b> [YAML](https://yaml.org/) [↩](#a2)

<b id="f3">3</b> [JSON](https://www.json.org/json-en.html) [↩](#a3)

<b id="f4">4</b> [Dhalll](https://dhall-lang.org/#) [↩](#a4)

