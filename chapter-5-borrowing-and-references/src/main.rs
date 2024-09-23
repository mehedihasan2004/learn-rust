/*
References and Borrowing
Safety and Performance
Borrowing and references are powerfull concepts

Understanding References:-
References: Enable you to borrow values without taking ownership.
I. Immutable reference.
II. Mutabel reference.
Create reference by add "&".
*/

// I. Immutable reference
// fn main() {
//     let x: i8 = 5;
//     let y: &i8 = &x;

//     println!("Value of x is: {}", x);
//     println!("Value of y is: {}", y);
// }

// II. Mutable reference
fn main() {
    // let mut x: i8 = 5;
    // let y: &mut i8 = &mut x;

    // *y += 1;

    // println!("Value of x is: {}", x);
    // println!("Value of y is: {}", y); // Error-> A mutable reference is allowed only.

    let mut account: BankAccount = BankAccount {
        owner: "John".to_string(),
        balance: 149.44,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdrao money
    account.withdraw(45.5);

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account, owned by {}",
            amount, self.owner
        );

        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {}, has a balance of {}",
            self.owner, self.balance
        );
    }
}
