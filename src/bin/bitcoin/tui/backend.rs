use crate::ui::{error_ui::ErrorUI, account};

use crate::process::transaction;

use cargosos_bitcoin::{
    logs::logger_sender::LoggerSender,
    node_structure::broadcasting::Broadcasting,
    block_structure::utxo_set::UTXOSet,
    wallet_structure::{
        account::Account, address::Address, private_key::PrivateKey, public_key::PublicKey,
        wallet::Wallet,
    }, 
    notifications::notifier::Notifier,
};

use std::io::{
    stdin,
    Write,
    Read,
};

/// Get the private key from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
fn get_private_key(logger: LoggerSender) -> Result<PrivateKey, ErrorUI> {
    let mut private_key: String = String::new();

    println!("Enter the private key: ");
    if stdin().read_line(&mut private_key).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        let _: PrivateKey = match PrivateKey::try_from(private_key.trim()) {
            Ok(result) => {
                let _ = logger.log_wallet("Valid private key entered".to_string());
                return Ok(result);
            }
            Err(error) => {
                let _ = logger.log_wallet(format!(
                    "Invalid private key entered, with error: {:?}",
                    error
                ));

                private_key.clear();
                println!("Error, please enter a valid private key:");
                if stdin().read_line(&mut private_key).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }
                continue;
            }
        };
    }
}

/// Get the public key from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
fn get_public_key(logger: LoggerSender) -> Result<PublicKey, ErrorUI> {
    let mut public_key: String = String::new();

    println!("Enter the public key: ");
    if stdin().read_line(&mut public_key).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        let _: PublicKey = match PublicKey::try_from(public_key.trim().to_string()) {
            Ok(result) => {
                let _ = logger.log_wallet("Valid public key entered".to_string());
                return Ok(result);
            }
            Err(error) => {
                let _ = logger.log_wallet(format!(
                    "Invalid public key entered, with error: {:?}",
                    error
                ));

                public_key.clear();
                println!("Error, please enter a valid public key:");
                if stdin().read_line(&mut public_key).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }

                continue;
            }
        };
    }
}

/// Get the address from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
pub(super) fn get_address(logger: LoggerSender) -> Result<Address, ErrorUI> {
    let mut address: String = String::new();

    println!("Enter the address: ");
    if stdin().read_line(&mut address).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        match Address::new(address.trim()) {
            Ok(result) => {
                let _ = logger.log_wallet("Valid address entered".to_string());
                return Ok(result);
            }
            Err(error) => {
                let _ = logger.log_wallet(format!(
                    "Put an invalid public key, with error: {:?}",
                    error
                ));

                address.clear();
                println!("Error, please enter a valid address:");
                if stdin().read_line(&mut address).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }
                continue;
            }
        };
    }
}

/// Get the account name from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
fn get_account_name() -> Result<String, ErrorUI> {
    let mut name: String = String::new();

    println!("Enter the name: ");
    match stdin().read_line(&mut name) {
        Ok(_) => Ok(name.trim().to_string()),
        Err(_) => Err(ErrorUI::TerminalReadFail),
    }
}

/// Creates a new account with the data entered by the user
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
pub fn create_account<N : Notifier>(
    wallet: &Wallet,
    notifier: N,
    logger: LoggerSender
) -> Result<(), ErrorUI> {
    let _ = logger.log_wallet("Creating a new account".to_string());

    let private_key = get_private_key(logger.clone())?;
    let public_key = get_public_key(logger.clone())?;
    let account_name = get_account_name()?;

    account::create_account(
        wallet,
        &account_name,
        private_key,
        public_key,
        notifier,
    )
}

/// Delete the selected account selected by the user
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
///  * `ErrorUI::CannotUnwrapArc`: It will appear when we try to unwrap an Arc
pub fn remove_account(
    wallet: &Wallet, 
    logger: LoggerSender
) -> Result<(), ErrorUI> {
    let account = select_account(&wallet, logger)?;
    wallet.remove_account(account);

    Ok(())
}

/// Change the selected account to the one selected by the user
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
///  * `ErrorUI::CannotUnwrapArc`: It will appear when we try to unwrap an Arc
pub fn change_account<N : Notifier>(
    wallet: &Wallet, 
    notifier: N,
    logger: LoggerSender
) -> Result<(), ErrorUI> {

    let _ = logger.log_wallet("Selecting an account".to_string());

    println!("Possible accounts: ");
    show_accounts(wallet, logger.clone());

    let mut account_name: String = String::new();

    println!("Enter the address: ");
    if stdin().read_line(&mut account_name).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    while account::change_selected_account(account_name, wallet, notifier).is_err() {
        let _ = logger.log_wallet("Invalid account name entered".to_string());

        account_name.clear();
        println!("Error, please enter a valid account name:");
        if stdin().read_line(&mut account_name).is_err() {
            return Err(ErrorUI::TerminalReadFail);
        }
    }

    Ok(())
}

/// Get an account from the wallet with the corresponding name
fn get_account_from_name(account_name: &str, wallet: &Wallet) -> Option<Account> {
    for account in wallet.get_accounts() {
        if account.account_name == account_name {
            return Some(account.clone());
        }
    }

    None
}

/// Select an account from the wallet
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
pub fn select_account(
    wallet: &Wallet,
    logger: LoggerSender,
) -> Result<Account, ErrorUI> {
    let _ = logger.log_wallet("Selecting an account".to_string());

    println!("Possible accounts: ");
    show_accounts(wallet, logger.clone());

    let mut account_name: String = String::new();

    println!("Enter the address: ");
    if stdin().read_line(&mut account_name).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        match get_account_from_name(account_name.trim(), wallet) {
            Some(account) => {
                let _ = logger.log_wallet("Valid account name entered".to_string());
                return Ok(account);
            }
            None => {
                let _ = logger.log_wallet("Invalid account name entered".to_string());

                account_name.clear();
                println!("Error, please enter a valid account name:");
                if stdin().read_line(&mut account_name).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }
                continue;
            }
        };
    }
}

/// Show all accounts from the wallet
pub fn show_accounts(wallet: &Wallet, logger: LoggerSender) {
    let _ = logger.log_wallet("Showing accounts".to_string());

    let possible_selected_account = wallet.get_selected_account();

    wallet.get_accounts().iter().for_each(|account| {
        let mut selected = "";
        if let Some(selected_account) = possible_selected_account {
            if selected_account == account {
                selected = "[ ★ ]";
            }
        }

        println!("{selected} {account}\n");
    });
}

/// Get the amount for the transaction from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
fn get_amount(logger: LoggerSender) -> Result<f64, ErrorUI> {
    let mut amount: String = String::new();

    println!("Enter an amount: ");
    if stdin().read_line(&mut amount).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        match amount.trim().parse::<f64>() {
            Ok(result) => {
                let _ = logger.log_wallet("Valid amount entered".to_string());
                return Ok(result);
            }
            Err(error) => {
                let _ =
                    logger.log_wallet(format!("Invalid amount entered, with error: {:?}", error));

                amount.clear();
                println!("Error, please enter a valid amount:");
                if stdin().read_line(&mut amount).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }

                continue;
            }
        };
    }
}

/// Get the fee for the transaction from the terminal
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
fn get_fee(logger: LoggerSender) -> Result<f64, ErrorUI> {
    let mut fee: String = String::new();

    println!("Enter a fee: ");
    if stdin().read_line(&mut fee).is_err() {
        return Err(ErrorUI::TerminalReadFail);
    }

    loop {
        match fee.trim().parse::<f64>() {
            Ok(result) => {
                let _ = logger.log_wallet("Valid fee entered".to_string());
                return Ok(result);
            }
            Err(error) => {
                let _ = logger.log_wallet(format!("Invalid fee entered, with error: {:?}", error));

                fee.clear();
                println!("Error, please enter a valid fee:");
                if stdin().read_line(&mut fee).is_err() {
                    return Err(ErrorUI::TerminalReadFail);
                }

                continue;
            }
        };
    }
}

/// Broadcast the transaction created by the user to the peers from the selected account in the wallet
///
/// ### Error
///  * `ErrorUI::TerminalReadFail`: It will appear when the terminal read fails
///  * `ErrorUI::CannotUnwrapArc`: It will appear when we try to unwrap an Arc
///  * `ErrorUI::ErrorFromPeer`: It will appear when a conextion with a peer fails
pub fn sending_transaction<N : Notifier, RW: Read + Write + Send + 'static>(
    broadcasting: &mut Broadcasting<RW>,
    wallet: &Wallet,
    utxo_set: &UTXOSet,
    notifier: N,
    logger: LoggerSender,
) -> Result<(), ErrorUI> {
    let address = get_address(logger.clone())?;
    let amount = get_amount(logger.clone())?;
    let fee = get_fee(logger.clone())?;

    transaction::sending_transaction(
        broadcasting,
        wallet,
        utxo_set,
        address,
        (amount, fee),
        notifier,
        logger,
    )
}