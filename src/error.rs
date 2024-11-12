use cosmwasm_std::{StdError, VerificationError};

#[derive(thiserror::Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("unauthorized")]
    Unauthorized {},
    #[error("invalid arguments")]
    InvalidArguments {},
}

impl From<ContractError> for StdError {
    fn from(source: ContractError) -> Self {
        Self::generic_err(source.to_string())
    }
}

pub type ContractResult<T> = std::result::Result<T, ContractError>;
