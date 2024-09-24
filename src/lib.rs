use candid::{CandidType, Deserialize};
use ic_cdk::api::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

/// Represents the token wallet structure
/// It contains a hashmap of balances, where the key is the user's principal ID (as a string)
/// and the value is their token balance
#[derive(CandidType, Default, Deserialize)]
struct TokenWallet {
    balances: HashMap<String, u64>,
}

/// Arguments for the send_tokens function
#[derive(CandidType, Deserialize)]
struct SendArgs {
    to: String,     // Recipient's principal ID
    amount: u64,    // Amount of tokens to send
}

/// Thread-local storage for the wallet
/// This allows the wallet state to persist between function calls
thread_local! {
    static WALLET: RefCell<TokenWallet> = RefCell::new(TokenWallet::default());
}

/// Sends tokens from the caller to another account
/// 
/// # Arguments
/// * `args` - A SendArgs struct containing the recipient and amount
/// 
/// # Returns
/// * `Ok(())` if the transfer is successful
/// * `Err(String)` with an error message if the transfer fails
#[update]
fn send_tokens(args: SendArgs) -> Result<(), String> {
    let caller = caller().to_string();  // Get the caller's principal ID as a string
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        let sender_balance = wallet.balances.get(&caller).cloned().unwrap_or(0);
        
        // Check if the sender has sufficient balance
        if sender_balance < args.amount {
            return Err("Insufficient balance".to_string());
        }
        
        // Deduct the amount from the sender's balance
        *wallet.balances.entry(caller).or_insert(0) -= args.amount;
        // Add the amount to the recipient's balance
        *wallet.balances.entry(args.to).or_insert(0) += args.amount;
        
        Ok(())
    })
}

/// Receives tokens into the caller's account
/// 
/// # Arguments
/// * `amount` - The amount of tokens to receive
/// 
/// # Returns
/// * `Ok(())` if the operation is successful
/// * `Err(String)` with an error message if the operation fails
#[update]
fn receive_tokens(amount: u64) -> Result<(), String> {
    let caller = caller().to_string();  // Get the caller's principal ID as a string
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        // Add the amount to the caller's balance
        *wallet.balances.entry(caller).or_insert(0) += amount;
        Ok(())
    })
}

/// Retrieves the balance of the caller's account
/// 
/// # Returns
/// The balance of the caller's account as a u64
#[query]
fn get_balance() -> u64 {
    let caller = caller().to_string();  // Get the caller's principal ID as a string
    WALLET.with(|wallet| {
        wallet.borrow().balances.get(&caller).cloned().unwrap_or(0)
    })
}

/// Resets all balances in the wallet to zero
/// This function is mainly used for testing purposes
#[update]
fn reset() {
    WALLET.with(|wallet| {
        wallet.borrow_mut().balances.clear();
    })
}

/// Unit tests for the token wallet functions
#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    /// Tests the send and receive token functionality
    #[test]
    fn test_send_receive_tokens() {
        // Reset the wallet
        reset();

        // Set up initial balance for sender
        ic_cdk::setup();
        ic_cdk::setup_with(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
        assert!(receive_tokens(100).is_ok());

        // Send tokens
        let send_args = SendArgs {
            to: "renrk-eyaaa-aaaaa-aaada-cai".to_string(),
            amount: 50,
        };
        assert!(send_tokens(send_args).is_ok());

        // Check balances
        assert_eq!(get_balance(), 50);
        
        ic_cdk::setup_with(Principal::from_text("renrk-eyaaa-aaaaa-aaada-cai").unwrap());
        assert_eq!(get_balance(), 50);
    }

    /// Tests the insufficient balance scenario
    #[test]
    fn test_insufficient_balance() {
        // Reset the wallet
        reset();

        // Set up initial balance for sender
        ic_cdk::setup();
        ic_cdk::setup_with(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
        assert!(receive_tokens(40).is_ok());

        // Try to send more tokens than available
        let send_args = SendArgs {
            to: "renrk-eyaaa-aaaaa-aaada-cai".to_string(),
            amount: 50,
        };
        assert!(send_tokens(send_args).is_err());

        // Check balances (should remain unchanged)
        assert_eq!(get_balance(), 40);
        
        ic_cdk::setup_with(Principal::from_text("renrk-eyaaa-aaaaa-aaada-cai").unwrap());
        assert_eq!(get_balance(), 0);
    }
}