#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let acc = BankAccount::new(100.0);
        assert_eq!(acc.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        let mut acc = BankAccount::new(100.0);
        acc.deposit(50.0);
        assert_eq!(acc.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(40.0);
        assert_eq!(acc.balance(), 60.0);
    }

    #[test]
    fn test_negative_deposit() {
        let mut acc = BankAccount::new(100.0);
        acc.deposit(-20.0);
        assert_eq!(acc.balance(), 100.0);
    }

    #[test]
    fn test_withdraw_too_much() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(200.0);
        assert_eq!(acc.balance(), 100.0);
    }
}