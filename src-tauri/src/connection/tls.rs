use std::sync::OnceLock;

use tokio_postgres_rustls::MakeRustlsConnect;

static CONNECTOR: OnceLock<MakeRustlsConnect> = OnceLock::new();

pub fn install_crypto_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

pub fn connector() -> Result<MakeRustlsConnect, String> {
    if let Some(existing) = CONNECTOR.get() {
        return Ok(existing.clone());
    }

    let (connect, _warnings) = MakeRustlsConnect::with_native_certs()
        .map_err(|errors| format!("failed to load native TLS certificates: {errors:?}"))?;

    Ok(CONNECTOR.get_or_init(|| connect).clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_native_certificates() {
        install_crypto_provider();

        connector().expect("native certificate store should be readable in CI/dev environments");
    }

    #[test]
    fn installing_the_crypto_provider_twice_does_not_panic() {
        install_crypto_provider();
        install_crypto_provider();
    }
}
