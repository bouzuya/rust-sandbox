use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHasher, SaltString,
    },
    Argon2, PasswordHash, PasswordVerifier,
};

#[derive(Clone, Eq, PartialEq)]
pub struct UserSecret(String);

impl UserSecret {
    pub fn generate() -> anyhow::Result<(Self, String)> {
        let mut bytes = [0u8; 48];
        OsRng.fill_bytes(&mut bytes);
        let password = hex::encode(bytes);

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok((Self(password_hash), password))
    }

    pub fn verify(&self, password: &str) -> anyhow::Result<()> {
        let password_hash = PasswordHash::new(&self.0).expect("password_hash to be valid");

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &password_hash)?)
    }
}

impl std::fmt::Debug for UserSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UserSecret").field(&"[FILTERED]").finish()
    }
}

impl std::fmt::Display for UserSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for UserSecret {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let password_hash = PasswordHash::new(s)?;
        Ok(Self(password_hash.to_string()))
    }
}
