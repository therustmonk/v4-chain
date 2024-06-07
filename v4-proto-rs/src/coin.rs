/// See also
/// https://github.com/cosmos/cosmos-rust/blob/cosmrs/v0.16.0/cosmrs/src/base/coin.rs
/// https://github.com/cosmos/cosmos-rust/blob/cosmrs/v0.16.0/cosmrs/src/base/denom.rs
use crate::cosmos::base::v1beta1::Coin as ProtoCoin;
use num_bigint::BigUint;
use std::{fmt, str::FromStr};

/// Coin defines a token with a denomination and an amount.
/// available with feature `wrappers` by default
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    pub denom: Denom,
    pub amount: BigUint,
}

impl Coin {
    /// Constructor
    pub fn new(amount: BigUint, denom: &str) -> Result<Self, String> {
        Ok(Coin {
            amount,
            denom: denom.parse()?,
        })
    }
}

impl TryFrom<ProtoCoin> for Coin {
    type Error = String;

    fn try_from(proto: ProtoCoin) -> Result<Coin, String> {
        Coin::try_from(&proto)
    }
}

impl TryFrom<&ProtoCoin> for Coin {
    type Error = String;

    fn try_from(proto: &ProtoCoin) -> Result<Coin, String> {
        let amount = BigUint::parse_bytes(proto.amount.as_bytes(), 10)
            .ok_or_else(|| format!("cannot parse {} as a non-negative integer", proto.amount))?;
        Ok(Coin {
            denom: proto.denom.parse()?,
            amount,
        })
    }
}

impl From<Coin> for ProtoCoin {
    fn from(coin: Coin) -> ProtoCoin {
        let Coin { denom, amount } = coin;
        ProtoCoin {
            denom: denom.0,
            amount: amount.to_str_radix(10),
        }
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // See: https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/types/coin.go#L643-L645
        write!(f, "{}{}", self.amount, self.denom)
    }
}

/// Denomination.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Denom(String);

impl Denom {
    /// Minimum length of a [`Denom`].
    pub const MIN_LENGTH: usize = 3;

    /// Maximum length of a [`Denom`].
    pub const MAX_LENGTH: usize = 128;
}

impl fmt::Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl FromStr for Denom {
    type Err = String;

    /// NOTE: implements the same checks as the `MatchDenom` function from the upstream Cosmos SDK.
    /// <https://github.com/cosmos/cosmos-sdk/blob/6a07568/types/coin.go#L885-L906>
    fn from_str(s: &str) -> Result<Self, String> {
        if s.len() < Self::MIN_LENGTH || s.len() > Self::MAX_LENGTH {
            return Err(format!(
                "denom length {} is out of bounds {}-{}",
                s.len(),
                Self::MIN_LENGTH,
                Self::MAX_LENGTH
            ));
        }

        if !s.chars().all(is_valid_denom_char) {
            return Err("denom contains invalid chars".to_string());
        }

        Ok(Denom(s.to_owned()))
    }
}

/// Check if a given character is allowed in a `Denom` name.
///
/// NOTE: implements the same checks as the `isValidRune` function from the upstream Cosmos SDK.
/// <https://github.com/cosmos/cosmos-sdk/blob/6a07568/types/coin.go#L879-L883>
#[inline]
fn is_valid_denom_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '/' | ':' | '.' | '_' | '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_conversion() {
        let proto_coin = crate::cosmos::base::v1beta1::Coin {
            denom: "adv4tnt".to_string(),
            amount: "123".to_string(),
        };
        let coin: Coin = proto_coin.try_into().unwrap();
        assert_eq!(coin.denom, Denom("adv4tnt".to_string()));
        assert_eq!(coin.amount, BigUint::from(123u32));
    }

    #[test]
    fn parse() {
        assert!(
            "ibc/9F53D255F5320A4BE124FF20C29D46406E126CE8A09B00CA8D3CFF7905119728"
                .parse::<Denom>()
                .is_ok()
        );
    }
}
