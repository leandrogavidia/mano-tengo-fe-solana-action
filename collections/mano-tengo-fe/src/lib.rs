use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::AccountMeta, instruction::Instruction, message::Message, pubkey, pubkey::Pubkey,
    transaction::Transaction,
};
use std::str::FromStr;
use znap::prelude::*;

const MEMO_PROGRAM_PUBKEY: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

#[collection]
pub mod mano_tengo_fe {

    use super::*;

    pub fn mano_tengo_fe(ctx: Context<ManoTengoFeAction>) -> Reslt<ActionTransaction> {
        let account_pubkey = Pubkey::from_str(&ctx.payload.account)
            .or_else(|_| Err(Error::from(ActionError::InvalidAccountPubkey)))?;

        let memo = CreateMemoArgs {
            memo: format!("¡MANO TENGO FE! 🇻🇪"),
        };
        let accounts: Vec<AccountMeta> = vec![AccountMeta::new(account_pubkey, true)];
        let ix = Instruction::new_with_borsh(MEMO_PROGRAM_PUBKEY, &memo, accounts);

        let msg = Message::new(&[ix], None);
        let tx = Transaction::new_unsigned(msg);

        Ok(ActionTransaction {
            message: Some(format!("¡Vamos con fe! 🇻🇪")),
            transaction: tx,
        })
    }
}

#[derive(Action)]
#[action(
    icon = "https://raw.githubusercontent.com/leandrogavidia/files/refs/heads/main/mano-tengo-fe.png",
    title = "¡MANO TENGO FE! 🇻🇪",
    description = "Blink para enviar fe a Solana",
    label = "¡MANO TENGO FE! 🇻🇪",
    link = {
        label = "¡MANO TENGO FE! 🇻🇪",
        href = "/api/mano_tengo_fe",
    },
)]
pub struct ManoTengoFeAction;

#[derive(ErrorCode)]
pub enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateMemoArgs {
    pub memo: String,
}
