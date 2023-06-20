use super::{account, error_tui::ErrorTUI, menu, menu_option::MenuOption, transaction};

use cargosos_bitcoin::{
    block_structure::{
        block::Block, block_chain::BlockChain, error_block::ErrorBlock, transaction::Transaction,
        utxo_set::{UTXOSet, self},
    },
    logs::logger_sender::LoggerSender,
    node_structure::{
        broadcasting::Broadcasting, error_node::ErrorNode, message_response::MessageResponse,
    },
    wallet_structure::wallet::Wallet,
};

use std::{
    net::TcpStream,
    sync::mpsc::Receiver,
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

type MutArc<T> = Arc<Mutex<T>>;

fn get_reference<'t, T>(reference: &'t MutArc<T>) -> Result<MutexGuard<'t, T>, ErrorTUI> {
    match reference.lock() {
        Ok(reference) => Ok(reference),
        Err(_) => Err(ErrorTUI::CannotUnwrapArc),
    }
}

pub fn user_input(
    broadcasting: &mut Broadcasting<TcpStream>,
    wallet: MutArc<Wallet>,
    utxo_set: MutArc<UTXOSet>,
    block_chain: MutArc<BlockChain>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    loop {
        match menu::select_option(logger.clone())? {
            MenuOption::CreateAccount => creating_accout(&wallet, logger.clone())?,
            MenuOption::ChangeAccount => changing_account(&wallet, logger.clone())?,
            MenuOption::RemoveAccount => removing_account(&wallet, logger.clone())?,
            MenuOption::SendTransaction => {
                sending_transaction(broadcasting, &wallet, &utxo_set, logger.clone())?
            },
            MenuOption::ShowAccounts => {
                let wallet_ref = get_reference(&wallet)?;
                account::show_accounts(&wallet_ref, logger.clone());
            }
            MenuOption::ShowBalance => showing_balance(&wallet, &utxo_set, logger.clone())?,
            MenuOption::LastTransactions => latest_transactions(&block_chain, logger.clone())?,
            MenuOption::Exit => break,
        }
    }

    Ok(())
}

fn creating_accout(wallet: &MutArc<Wallet>, logger: LoggerSender) -> Result<(), ErrorTUI> {
    let mut wallet = get_reference(wallet)?;
    let account = account::create_account(logger.clone())?;
    wallet.add_account(account);

    Ok(())
}

fn changing_account(wallet: &MutArc<Wallet>, logger: LoggerSender) -> Result<(), ErrorTUI> {
    let mut wallet = get_reference(wallet)?;
    let account = account::select_account(&wallet, logger.clone())?;
    wallet.change_account(account);

    Ok(())
}

fn removing_account(wallet: &MutArc<Wallet>, logger: LoggerSender) -> Result<(), ErrorTUI> {
    let mut wallet = get_reference(wallet)?;
    let account = account::select_account(&wallet, logger.clone())?;
    wallet.remove_account(account);

    Ok(())
}

fn sending_transaction(
    broadcasting: &mut Broadcasting<TcpStream>,
    wallet: &MutArc<Wallet>,
    utxo_set: &MutArc<UTXOSet>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    let wallet = get_reference(wallet)?;
    let account = match wallet.get_selected_account() {
        Some(account) => account,
        None => {
            let message = "No account selected can't send transaction";
            println!("{message}");
            let _ = logger.log_wallet(message.to_string());
            return Ok(());
        }
    };
    let utxo_set = get_reference(utxo_set)?;

    let transaction = transaction::create_transaction(&utxo_set, account, logger.clone())?;
    let _ = logger.log_transaction("Sending transaction".to_string());

    match broadcasting.send_transaction(transaction) {
        Ok(()) => Ok(()),
        Err(ErrorNode::WhileSendingMessage(message)) => Err(ErrorTUI::ErrorFromPeer(message)),
        _ => Err(ErrorTUI::ErrorFromPeer(
            "While sending transaction".to_string(),
        )),
    }
}

fn showing_balance(
    wallet: &MutArc<Wallet>,
    utxo_set: &MutArc<UTXOSet>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    let wallet = get_reference(wallet)?;

    let account = wallet.get_selected_account();
    match account {
        Some(account) => {
            let utxo_set = get_reference(utxo_set)?;
            let balance = utxo_set.get_balance_in_satoshis(&account.address);
            let message_output = format!(
                "Account: {:?} has balance of {balance}",
                account.account_name
            );

            println!("{message_output}");
            let _ = logger.log_wallet(message_output);
        }
        None => {
            let _ = logger.log_wallet("No account selected".to_string());
        }
    }

    Ok(())
}

fn latest_transactions(
    block_chain: &MutArc<BlockChain>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    let selected_timestamp = transaction::select_option(logger.clone())?;
    let timestamp = selected_timestamp.get_timestamps_from_now();

    let _ = logger.log_transaction(format!(
        "Selected timestamp: {selected_timestamp}, and it's corresponding timestamp: {timestamp}"
    ));

    let block_chain = get_reference(&block_chain)?;
    let blocks = block_chain.get_blocks_after_timestamp(timestamp as u32);

    for block in blocks {
        for transaction in block.transactions {
            println!("{transaction}");
        }
    }

    Ok(())
}

pub fn handle_peers(
    receiver_broadcasting: Receiver<MessageResponse>,
    wallet: MutArc<Wallet>,
    utxo_set: MutArc<UTXOSet>,
    block_chain: MutArc<BlockChain>,
    logger: LoggerSender,
) -> JoinHandle<Result<(), ErrorTUI>> {
    thread::spawn(move || {
        let mut transactions: Vec<Transaction> = Vec::new();

        for message in receiver_broadcasting {
            match message {
                MessageResponse::Block(block) => {
                    receive_block(
                        &utxo_set,
                        &block_chain,
                        block,
                        &mut transactions,
                        logger.clone(),
                    )?;
                }
                MessageResponse::Transaction(transaction) => {
                    receive_transaction(&wallet, transaction, &mut transactions, logger.clone())?;
                }
            }
        }

        Ok(())
    })
}

fn receive_transaction(
    wallet: &MutArc<Wallet>,
    transaction: Transaction,
    transactions: &mut Vec<Transaction>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    if let Some(account) = get_reference(&wallet)?.get_selected_account() {
        if account.verify_transaction_ownership(&transaction) {
            println!("{transaction} is valid and has not been added to the blockchain yet");
            let _ = logger.log_wallet(format!(
                "Adding transaction {transaction} to list of transaction seen so far"
            ));
            transactions.push(transaction);
        }
    }

    Ok(())
}

fn receive_block(
    utxo_set: &MutArc<UTXOSet>,
    block_chain: &MutArc<BlockChain>,
    block: Block,
    transactions: &mut Vec<Transaction>,
    logger: LoggerSender,
) -> Result<(), ErrorTUI> {
    transactions.retain(|transaction| {
        if block.transactions.contains(transaction) {
            println!("{transaction} has been added to the blockchain");
            let _ = logger.log_wallet(format!(
                "Removing transaction {transaction} from list of transaction seen so far"
            ));
            return false;
        }
        true
    });

    let mut utxo_set = get_reference(&utxo_set)?;
    let mut block_chain = get_reference(&block_chain)?;

    utxo_set.update_utxo_with_block(&block);

    match block_chain.append_block(block) {
        Ok(_) | Err(ErrorBlock::TransactionAlreadyInBlock) => Ok(()),
        _ => Err(ErrorTUI::ErrorWriting(
            "Error appending block to blockchain".to_string(),
        )),
    }
}
