use super::error_process::ErrorProcess;

use cargosos_bitcoin::{
    block_structure::block_chain::BlockChain,
    configurations::{save_config::SaveConfig, try_default::TryDefault},
    logs::logger_sender::LoggerSender,
    serialization::deserializable_internal_order::DeserializableInternalOrder,
    wallet_structure::wallet::Wallet,
};

use std::{
    fs::OpenOptions,
    io::BufReader,
    marker::Send,
    mem::replace,
    thread::{self, JoinHandle},
};

type Handle<T> = Option<JoinHandle<T>>;

const BLOCKCHAIN_FILE: &str = "Blockchain";
const WALLET_FILE: &str = "Wallet";

/// Represents the elements to load from files
pub struct LoadSystem {
    block_chain: Handle<Result<BlockChain, ErrorProcess>>,
    wallet: Handle<Result<Wallet, ErrorProcess>>,
}

impl LoadSystem {
    pub fn new(save_config: SaveConfig, logger: LoggerSender) -> LoadSystem {
        LoadSystem {
            block_chain: Some(Self::load_value(
                BLOCKCHAIN_FILE.to_string(),
                save_config.read_block_chain,
                logger.clone(),
            )),
            wallet: Some(Self::load_value(
                WALLET_FILE.to_string(),
                save_config.read_wallet,
                logger,
            )),
        }
    }

    /// Get the block chain from a file, if already loaded it will return the value immediately.
    /// In the case of the file not existing, it will return the default value.
    /// 
    /// ### Error
    ///  * `ErrorProcess:FailThread`: It will appear when a thread panics and fails
    ///  * `ErrorProcess:CannotCreateDefault`: It will appear when can't create the default value
    ///  * `ErrorProcess:AlreadyLoaded`: It will appear when try to get a value that is already loaded
    pub fn get_block_chain(&mut self) -> Result<BlockChain, ErrorProcess> {
        let block_chain_handle = replace(&mut self.block_chain, None);

        if let Some(block_chain_handle) = block_chain_handle {
            return match block_chain_handle.join() {
                Ok(block_chain) => block_chain,
                _ => Err(ErrorProcess::FailThread),
            };
        }

        Err(ErrorProcess::AlreadyLoaded)
    }

    /// Get the wallet from a file, if already loaded it will return the value immediately.
    /// In the case of the file not existing, it will return the default value.
    /// 
    /// ### Error
    ///  * `ErrorProcess:FailThread`: It will appear when a thread panics and fails
    ///  * `ErrorProcess:CannotCreateDefault`: It will appear when can't create the default value
    ///  * `ErrorProcess:AlreadyLoaded`: It will appear when try to get a value that is already loaded
    pub fn get_wallet(&mut self) -> Result<Wallet, ErrorProcess> {
        let wallet_handle = replace(&mut self.wallet, None);

        if let Some(wallet_handle) = wallet_handle {
            return match wallet_handle.join() {
                Ok(wallet) => wallet,
                _ => Err(ErrorProcess::FailThread),
            };
        }

        Err(ErrorProcess::AlreadyLoaded)
    }

    /// Creates a thread to load a deserializable from a file, if the file does not exist or fail to read it will return the default value.
    /// 
    /// ### Error
    ///  * `ErrorProcess:CannotCreateDefault`: It will appear when can't create the default value
    fn load_value<V: TryDefault + DeserializableInternalOrder + Send + 'static>(
        name: String,
        path: Option<String>,
        logger: LoggerSender,
    ) -> JoinHandle<Result<V, ErrorProcess>> {
        thread::spawn(move || {
            if let Some(path) = path {
                if let Ok(file) = OpenOptions::new().read(true).open(path) {
                    let mut file = BufReader::new(file);

                    let _ = logger.log_file(format!("Reading the {name} from file"));

                    let value = V::io_deserialize(&mut file)?;

                    let _ = logger.log_file(format!("{name} loaded from file"));

                    return Ok(value);
                }

                let _ = logger.log_file(format!("Could not open {name} file"));
            }

            match V::try_default() {
                Ok(value) => Ok(value),
                Err(_) => {
                    let _ = logger.log_file(format!("Could create default for {name}"));
                    Err(ErrorProcess::CannotCreateDefault)
                }
            }
        })
    }
}
