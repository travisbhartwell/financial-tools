# Financial Importer

This is a rewrite of a Python program I lost (kudos to not having proper backups in place) that I used to 
import various CSV files into my ledger.

I know that other such solutions exist, so this exists largely as a learning tool as well as scratching my own itch.

One major departure will be that this program will operate based on configuration files. My Python script had all of the rules for matching transactions and what to generate from them hard-coded. This time, in order to make it possible to share the code but still keep my financial particulars private, the rules will come from a configuration file that is loaded at runtime.

# Concepts

* **Named Input File Type**: A description of a CSV file type  -- say, a CSV download from your credit card account -- and the rules for how to generate Ledger postings from potential rows in this file. This definition includes column definitions, matching rules, etc.

# Values

The following values are in interest of focus and simplicity of this tool:

* My use of Ledger CLI is a mix of manually entered postings and using automated tools (such as this one) to generate postings from various sources.

* As such, it is not expected that all input to this tool will produce postings. My intended usage is to have my most common postings to be easily generated from source data and then the un-matched input also output such that I can manually write these postings in my Ledger journal.

* Because my journal is a mix of manually entered postings and those generated from tools such as this one, I rely on my editor (specifically Emacs and [ledger-mode](https://github.com/ledger/ledger-mode)) to keep my journal formatted neatly and the postings sorted appropriately by date. Therefor, the Ledger CLI positings produced by this tool are only minimally formatted as to be correctly parsed by Ledger and related tools. My usual workflow is to take the output of this tool and paste it into my Ledger journal and let Emacs reformat and sort the file.

* Originally, I was going to make this tool more flexible. Instead, I will rely on clean up scripts that transform input data into a format that this tool understands. This tool will be solely focused on processing input and matching those against rules.

* At least initially, the supported postings that are output are of the simplest type that Ledger supports, with a date, a Payee, and then two accounts. For example, borrowing from the Ledger manual:

```
2004/09/29 Pacific Bell
    Expenses:Pacific Bell              $23.00
    Assets:Checking
```

However, per my own personal preference, even though the corresponding `$-23.00` on the final line can be elided for convenience, postings from this tool will be explicit on the amount for each line.

* Also, for simplicity (I think, for now), a line from the input should only match a single rule from the configuration. An error will be shown if multiple match, as this will be ambiguous.

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


Simplified:
* A definition of accounts (in the Ledger posting sense) that will be used in the rules, including potential nicknames for these accounts.

* A rule consists of the following:
   * A regular expression (using [the syntax](https://docs.rs/regex/1.4.3/regex/#syntax) supported by the Rust regex crate) to match against the description field in the input data.
   * The payee to use.
   * The account used in the first line of the posting.
   * The account used in the second line of the posting.
   * Negative account? 

Possible additions:
* Commodity/currency definitions and/or defaults.

Possible extensions:
* Instead of having a hard coded Payee field, optionally include named capture groups (need to see what is supported in the regex crate) from the description and then included in the Payee. For example, DoorDash transactions.

# TODO

* DONE Write a simple tool to take a file with a regex per line and then will print out matching lines from stdin.

* Write types for configuration to figure out loading.

* DONE Construct minimal configuration in code, and experiment with Serde and TOML bindings of writing out the data structure and see if it is possible to load it again.

* Change the transaction rule to store a string. Then have a `build` method that validates the rules loaded and then creates a `RegexSet` from the strings.

* Hook this up to regex tester.

* Add a `TODO` field, so this will output a comment so I can update the transaction manually.

---
<b id="f1">1</b> [TOML](https://toml.io/en/) [↩](#a1)

<b id="f2">2</b> [YAML](https://yaml.org/) [↩](#a2)

<b id="f3">3</b> [JSON](https://www.json.org/json-en.html) [↩](#a3)

<b id="f4">4</b> [Dhall](https://dhall-lang.org/#) [↩](#a4)

