# Prices History Update

For simplicity, (at least initially) this will work with the following constraints:

* The same start and end dates will be used for each commodity looked up.
* The pricesdb will be created from scratch each time. If I find fetching the entire history needed is a problem, I will handle just reading missing values in the future.
* The prices in the history file will be stored in date order.
* The pricesdb will output as much as possible; i.e. if the user specifies multiple commodities and one of them is invalid, a prices file would still be generated with the values for the valid commodities. We also will report on the errors.

# Test Run

```
cargo run -- --start-date "2020-01-01" --end-date "2021-01-01" --commodity AMZN --output-file ./pricesdb
```

To get the commodities from Ledger:

```
ledger commodities | grep -v "\$$" | gsed 's/^/---commodity /' | tr '\n' ' '
```

This will give you part of the command line:

```
cargo run -- --start-date "2014-09-30" --end-date "2021-01-12" --output-file ./pricesdb $(ledger commodities | grep -v "\$$" | gsed 's/^/---commodity /' | tr '\n' ' ')
```


# Prices History File Format

Example from the Ledger manual:

```
P 2004/06/21 02:17:58 TWCUX $27.76
P 2004/06/21 02:17:59 AGTHX $25.41
P 2004/06/21 02:18:00 OPTFX $39.31
P 2004/06/21 02:18:01 FEQTX $22.49
P 2004/06/21 02:18:02 AAPL $32.91
```

# References

* Prices DB Format: [Ledger Manual](https://www.ledger-cli.org/3.0/doc/ledger3.html#Commodity-price-histories)

# TODO

* Expand the documentation for commmand line arguments
* Add context to errors from `get_commodity_history`
* Make sure at least one commodity is specified
* Default options for the output path to expand the tilde as well as handling '-' for outputing to stdout
* Use `lazy_static!` for `provider` in `get_commodity_history`.