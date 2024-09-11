use bip39::{Language, Mnemonic};
// use derivation_path::{ChildIndex, DerivationPath};
use solana_sdk::derivation_path::DerivationPath;
use solana_sdk::signature::SeedDerivable;
use solana_sdk::signature::{Keypair, Signer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://vault12.com/learn/crypto-security-basics/what-is-bip39/
    // mnemonic words bip39
    let mnemonic = "neither lonely flavor argue grass remind eye tag avocado spot unusual intact";
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic)?;
    let seed = mnemonic.to_seed("");

    for i in 0..10 {
        // derivation path bip44
        // https://trezor.io/learn/a/what-is-bip44
        let path = format!("m/44'/501'/{}'/0'", i);
        let derivation_path = DerivationPath::from_absolute_path_str(&path).unwrap();
        print_pubkey(&seed, derivation_path, &path);

        let path = format!("{}'/0'", i);
        let derivation_path = DerivationPath::try_from(path.as_str()).unwrap();
        print_pubkey(&seed, derivation_path, &path);
    }

    Ok(())
}

pub fn print_pubkey(seed: &[u8], derivation_path: DerivationPath, path: &str) {
    let keypair = Keypair::from_seed_and_derivation_path(
        seed,
        Some(derivation_path)
    ).expect(format!("Failed to derive keypair from path {}", path).as_str());
    let pubkey = keypair.pubkey();
    println!("{} => {}", path, pubkey);
}
