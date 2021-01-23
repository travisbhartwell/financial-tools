# Data Cleanup

As my accounts don't offer data exports (that I have found available), I needed some way to easily import the data to minimize the number of transactions I need to enter by hand.

I have found [Tabula](https://tabula.technology/) to be a great tool for extracting tabular data from PDFs. It's a fairly simple process to load PDFs of statements from my banks into Tabula, have it auto-detect the tabular data and then export that as a CSV. However, it's not 100% perfect.

To simplify the [Financial Importer tool](../financial-importer),  I plan on writing some simple scripts here to clean up the output from Tabula into a more regular CSV file suitable for parsing in Financial Importer.

# TODO

* Chase Statement Cleaner
    * TODO Handle incrementing year when dates from both December and January are included