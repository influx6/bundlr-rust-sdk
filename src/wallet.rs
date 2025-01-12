use std::fs;

use jsonwebkey as jwk;

use crate::error::BundlrError;

pub fn load_from_file(path: &str) -> Result<jwk::JsonWebKey, BundlrError> {
    let jwt_str =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file {}", path));
    jwt_str.parse().map_err(|_| BundlrError::InvalidWallet)
}

mod tests {
    #[test]
    fn should_load_wallet_correctly() {
        let _res = super::load_from_file("res/test_wallet.json");
    }
}
