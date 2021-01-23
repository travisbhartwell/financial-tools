use financial_importer::FinancialImporter;
use std::io;
use std::io::prelude::*;

// fn write() {
//     let account_alias1: AccountAlias = String::from("chase_visa");
//     let full_account1: FullAccountName = String::from("Liabilities:Visa");
//     let account_alias2: AccountAlias = String::from("checking");
//     let full_account2: FullAccountName = String::from("Assets:Checking");
//     let account_alias3: AccountAlias = String::from("uber");
//     let full_account3: FullAccountName = String::from("Expenses:Transportation:Ride Share");

//     let mut importer_config = ImporterConfiguration::new();

//     importer_config.add_account_alias(account_alias1.clone(), full_account1.clone());
//     importer_config.add_account_alias(account_alias2.clone(), full_account2.clone());
//     importer_config.add_account_alias(account_alias3.clone(), full_account3.clone());

//     let rule1 = TransactionRule::new(
//         ".*Payment.*",
//         account_alias1,
//         account_alias2.clone(),
//         String::from("Amazon Rewards Visa Payment"),
//     );
//     importer_config.add_transaction_rule(rule1);

//     let rule2 = TransactionRule::new(
//         r".*(UBER TECHNOLOGIES|UBERTRIP).*",
//         account_alias3,
//         account_alias2.clone(),
//         String::from("Uber"),
//     );
//     importer_config.add_transaction_rule(rule2);

//     let toml = toml::to_string(&importer_config).unwrap();
//     println!("{}", toml);
// }

fn read() -> FinancialImporter {
    let contents = std::fs::read_to_string("test.toml")
        .unwrap_or_else(|_| panic!("Problems reading from file"));

    let importer: FinancialImporter = toml::from_str(&contents).unwrap();

    importer
}

fn main() {
    // println!("Testing writing configuration:");
    // write();

    println!("Testing reading configuration:");
    let importer = read();

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();

        importer.match_rule(line.as_str());
    }
}
