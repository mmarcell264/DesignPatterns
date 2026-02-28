use super::WalletFacede;

pub fn facade_main() -> Result<(), String> {
    let mut wallet = WalletFacede::new("abc".into(), 1234);
    println!();

    // Wallet Facade interacts with the account, code, wallet, notification and
    // ledger behind the scenes.
    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
    println!();

    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)    
}