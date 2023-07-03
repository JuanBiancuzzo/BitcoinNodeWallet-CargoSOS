use cargosos_bitcoin::{
    logs::logger_sender::LoggerSender,
    notifications::{notification::Notification, notifier::Notifier},
};

use std::cmp::max;

/// Struct that handles the representation of the notifications for the TUI.
#[derive(Clone)]
pub struct NotifierTUI {
    logger: LoggerSender,
}

impl NotifierTUI {
    pub fn new(logger: LoggerSender) -> Self {
        Self { logger }
    }
}

impl Notifier for NotifierTUI {
    fn notify(&self, notification: Notification) {
        match notification {
            Notification::AttemptingHandshakeWithPeer(socket_address) => {
                println!("Attempting handshake with {socket_address}");
            }
            Notification::SuccessfulHandshakeWithPeer(socket_address) => {
                println!("Successful handshake with {socket_address}");
            }
            Notification::FailedHandshakeWithPeer(socket_address) => {
                println!("Failed handshake with {socket_address}");
            }
            Notification::ConnectionUpdated(connection_id) => {
                println!("Connection updated: {connection_id}");
            }
            Notification::TransactionOfAccountReceived(accounts, transaction) => {
                for account in accounts {
                    show_notification(
                        "Transaction received",
                        &format!(
                            "The transaction: {transaction} was received \n    in the account: {account}",
                            transaction = transaction.clone(),
                        ),
                        &self.logger,
                    );
                }
            }
            Notification::TransactionOfAccountInNewBlock(block, transaction) => show_notification(
                "Transaction in block",
                &format!("The transaction {transaction} was added to a block with hash {block}"),
                &self.logger,
            ),
            Notification::NewBlockAddedToTheBlockchain(block) => {
                show_notification(
                    "New block added",
                    &format!("The block {block}\n    was added to the blockchain"),
                    &self.logger,
                );
            }
            Notification::UpdatedSelectedAccount(account) => {
                let message = format!("Account selected: {account}");
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::RegisterWalletAccount(account) => {
                let message = format!("New account {account} was added to the wallet");
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::NotifyBlockchainIsReady => {
                let message = "Blockchain is up to date".to_string();
                println!("{message}");
                let _ = self.logger.log_node(message);
            }
            Notification::LoadAvailableBalance(account, balance, pending) => {
                let total = format!("{:.8}", balance + pending);
                let balance = format!("{:.8}", balance);
                let pending = format!("{:.8}", pending);

                println!("Account: {account}\n    Balance: {balance}\n    Pending: {pending}\n    Total: {total}", account = account.account_name);
                let _ = self.logger.log_wallet(format!(
                    "Account: {account} with balance: {balance} and pending: {pending}",
                    account = account.account_name
                ));
            }
            Notification::AccountNotSelected => {
                let message = "Account not selected".to_string();
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::AccountTransactions(account, transactions) => {
                let mut message_transaction = "".to_string();
                for transaction in transactions {
                    message_transaction.push_str(&format!("{transaction}\n"));
                }
                show_notification(
                    &format!("In the account: {account}", account = account.account_name),
                    &message_transaction,
                    &self.logger,
                )
            }
            Notification::InvalidAddressEnter => {
                let message = "Invalid address enter".to_string();
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::InvalidPublicKeyEnter => {
                let message = "Invalid public key enter".to_string();
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::InvalidPrivateKeyEnter => {
                let message = "Invalid private key enter".to_string();
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::AccountCreationFail => {
                let message = "Creation of the account fail".to_string();
                println!("{message}");
                let _ = self.logger.log_wallet(message);
            }
            Notification::NotEnoughFunds => {
                let message = "Not enough founds to create transaction".to_string();
                println!("{message}");
                let _ = self.logger.log_transaction(message);
            }
            Notification::SuccessfullySentTransaction(transaction) => {
                show_notification(
                    "Transaction sent",
                    &format!("The transaction {transaction} was sent"),
                    &self.logger,
                );
            }
            Notification::ProgressDownloadingBlocks(blocks_downloaded, total_blocks) => {
                let percentage_downloaded =
                    (blocks_downloaded as f32 / total_blocks as f32) * 100.0;
                let message = format!(
                    "Finished downloading {percentage}% of blocks",
                    percentage = percentage_downloaded
                );
                println!("{message}");
            }
            Notification::ProgressUpdatingBlockchain(blocks_updated, total_blocks) => {
                let percentage_updated = (blocks_updated as f32 / total_blocks as f32) * 100.0;
                let message = format!(
                    "Finished updating {percentage}% of the blockchain",
                    percentage = percentage_updated
                );
                println!("{message}");
            }
            Notification::HeadersReceived(headers) => {
                println!("Received {headers} headers");
            }
            Notification::ClosingPeers => println!("Closing peers"),
            Notification::ClosingPeer => println!("Closing this peer"),
            Notification::ReceivedMessage(message) => {
                println!("Received message of type {:?}", message)
            }
            Notification::ProblemVerifyingTransactionMerkleProofOfInclusion(error_message) => {
                show_notification(
                    "Error while verifying transaction merkle proof of inclusion",
                    &format!("There was an error in the process of verifying the merkle proof\n the error was: {error_message}"),
                    &self.logger,
                );
            },
            Notification::SuccessfulMerkleProof(path, root) => {
                let mut message_path = "".to_string();

                for hash in path.clone() {
                    message_path.push_str(&format!("{}\n", from_hashtype_to_string(&hash)));
                }

                show_notification(
                    "Merkle proof successfully validated",
                    &format!(
                        "Merkle root: \n{root}\n Merkle path:\n{message_path}",
                        root = from_hashtype_to_string(&root),
                    ),
                    &self.logger,
                );
            },
        }
    }
}

/// Notify the user in a clean way
fn show_notification(title: &str, body: &str, logger: &LoggerSender) {
    let len_message = max(calculate_body_len(title), calculate_body_len(body));
    let border = "#".repeat(len_message + 4);

    let mut message = format!("{border}\n");
    for title_line in title.split('\n') {
        let spaces = len_message - title_line.len();
        message.push_str(&format!("# {}{} #\n", title_line, " ".repeat(spaces)));
    }

    for body_line in body.split('\n') {
        let spaces = len_message - body_line.len();
        message.push_str(&format!("# {}{} #\n", body_line, " ".repeat(spaces)));
    }

    message.push_str(border.as_str());

    println!("{message}");
    let _ = logger.log_notification(body.to_string());
}

/// Given a body of text, returns the length of the longest line
fn calculate_body_len(body: &str) -> usize {
    let mut len = 0;
    for line in body.split('\n') {
        len = max(len, line.len());
    }
    len
}

/// Turns a hashtype to a string
fn from_hashtype_to_string(hash: &[u8]) -> String {
    let mut hash_string = "".to_string();
    for byte in hash.iter() {
        hash_string.push_str(&format!("{:02x}", byte));
    }
    hash_string
}