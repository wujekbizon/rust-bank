use std::fmt;

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}
#[derive(Debug)]
struct InsufficientFundsError;

impl fmt::Display for InsufficientFundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Insufficient funds")
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> Result<i32, String> {
        if amount < 0 {
            return Err("Cannot deposit a negative amount".to_string());
        }

        self.balance += amount;
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: i32) -> Result<i32, InsufficientFundsError> {
        if amount < 0 || amount > self.balance {
            return Err(InsufficientFundsError);
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    fn summary(&self) -> String {
        format!("{} has a balance ${}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank: Bank = Bank::new();

    let mut account1: Account = Account::new(1, String::from("Account holder 1"));
    let account2: Account = Account::new(2, String::from("Account holder 2"));

    let balance_after_deposit: i32 = match account1.deposit(1000) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let balance_after_withdraw: i32 = match account1.withdraw(10) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("Your current balance : {:#?}", balance_after_deposit);
    println!("Your current balance : {:#?}", balance_after_withdraw);

    let summary = account1.summary();

    bank.add_account(account1);
    bank.add_account(account2);

    println!("{:#?}", summary);
    println!("{:#?}", bank.summary());

    let total_balance = bank.total_balance();
    println!(
        "Total balance for all accounts in our bank is ${:#?}",
        total_balance
    );
}
