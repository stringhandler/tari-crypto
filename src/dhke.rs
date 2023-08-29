// Copyright 2022 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

//! The robotic innards of a Diffie-Hellman key exchange (DHKE) producing a shared secret.
//! Even though the result of a DHKE is the same type as a public key, it is typically treated as a secret value.
//! To make this work more safely, we ensure that a DHKE result is cleared after use (but beware of subsequent copies or
//! moves). Because a DHKE shared secret is intended to be used in further key derivation, the only visibility into it
//! is as a byte array; it's not possible to directly extract the underlying public key type, and you probably shouldn't
//! clone the byte array without a very good reason. If you need the underlying public key itself, you probably should
//! be using something else.

use core::ops::Mul;

use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::keys::{PublicKey, SecretKey};

/// The result of a Diffie-Hellman key exchange
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DiffieHellmanSharedSecret<P>(P)
where P: Zeroize;

impl<P> DiffieHellmanSharedSecret<P>
where
    P: PublicKey + Zeroize,

{
    /// Perform a Diffie-Hellman key exchange
    pub fn new<'a, K>(sk: &K, pk: &P) -> Self
       where &'a K: SecretKey + Mul<&'a P, Output = P>,
    {
        Self(sk * pk)
    }

    /// Get the shared secret as a byte array
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[cfg(test)]
mod test {
    use rand_core::OsRng;

    use super::DiffieHellmanSharedSecret;
    use crate::{
        keys::{PublicKey, SecretKey},
        ristretto::{RistrettoPublicKey, RistrettoSecretKey},
    };

    #[test]
    fn test_dhke() {
        // Generate two key pairs
        let mut rng = OsRng;

        let sk1 = RistrettoSecretKey::random(&mut rng);
        let pk1 = RistrettoPublicKey::from_secret_key(&sk1);

        let sk2 = RistrettoSecretKey::random(&mut rng);
        let pk2 = RistrettoPublicKey::from_secret_key(&sk2);

        // Assert that both sides of a key exchange match
        let left = DiffieHellmanSharedSecret::<RistrettoPublicKey>::new(&sk1, &pk2);
        let right = DiffieHellmanSharedSecret::<RistrettoPublicKey>::new(&sk2, &pk1);

        assert_eq!(left.as_bytes(), right.as_bytes());
    }
}
