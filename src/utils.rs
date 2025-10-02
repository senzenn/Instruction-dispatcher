use solana_program::account_info::AccountInfo;

pub fn is_signer(account: &AccountInfo) -> bool {
    account.is_signer
}

pub fn is_writable(account: &AccountInfo) -> bool {
    account.is_writable
}
