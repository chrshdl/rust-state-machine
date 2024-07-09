mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
/* TODO: Add the derive macro to implement the `Debug` trait for `Pallet`. */
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
    }
}

fn main() {
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(&String::from("alice"), 100);

    // start emulating a block
    /* TODO: Increment the block number in system. */
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);
    /* TODO: Assert the block number is what we expect. */

    // first transaction
    /* TODO: Increment the nonce of `alice`. */
    runtime.system.inc_nonce(&String::from("alice"));
    /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
          the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */
    let _res = runtime
        .balances
        .transfer(&String::from("alice"), &String::from("bob"), 30)
        .map_err(|e| print!("{}", e));

    // second transaction
    /* TODO: Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(&String::from("alice"));

    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
        .balances
        .transfer(&String::from("alice"), &String::from("charlie"), 20)
        .map_err(|e| print!("{}", e));

    /* TODO: Print the final runtime state after all transactions. */
    println!("{:#?}", runtime);
    /*
    cwasilei@babydoll:~/projects/rust-state-machine$ cargo run
    Compiling rust-state-machine v0.1.0 (/home/cwasilei/projects/rust-state-machine)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
    Running `target/debug/rust-state-machine`

    Runtime {
        system: Pallet {
            block_number: 1,
            nonce: {
                "alice": 2,
            },
        },
        balances: Pallet {
            balances: {
                "alice": 50,
                "bob": 30,
                "charlie": 20,
            },
        },
    }
    */
}
