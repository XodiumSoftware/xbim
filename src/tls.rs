#![warn(clippy::all)]
#![forbid(unsafe_code)]

use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, SanType, date_time_ymd};
use std::path::PathBuf;
use std::{fs, io};

/// TLS Certificate Management & Generation.
#[derive(Debug)]
pub struct Tls {
    cert_path: PathBuf,
    key_path: PathBuf,
}

impl Tls {
    /// Creates a new instance of `Tls` with default values.
    ///
    /// # Returns
    /// A `Self` instance containing the default configuration.
    pub fn new(cert_path: PathBuf, key_path: PathBuf) -> io::Result<Self> {
        let tls = Self {
            cert_path,
            key_path,
        };

        if !tls.cert_path.exists() || !tls.key_path.exists() {
            tls.generate_certificates()?;
            println!(
                "WARNING: Auto-generated self-signed TLS certificates are for development only."
            );
            println!("For production use, provide CA-signed certificates at the specified paths:");
            println!("  Certificate: {}", tls.cert_path.display());
            println!("  Private Key: {}", tls.key_path.display());
        }

        Ok(tls)
    }

    /// Generates self-signed TLS certificates and writes them to disk.
    ///
    /// This function will:
    /// 1. Create parent directories if they don't exist
    /// 2. Generate certificate parameters with:
    ///    - Long validity period (1975-4096)
    ///    - Organization name "Xodium Software"
    ///    - Subject Alternative Name for "localhost"
    /// 3. Generate a new cryptographic key pair
    /// 4. Create a self-signed certificate
    /// 5. Write both certificate and private key to specified paths
    ///
    /// # Errors
    /// Returns `io::Result` with detailed error messages if any step fails, including:
    /// - Directory creation failures
    /// - Invalid DNS name format
    /// - Key generation failures
    /// - Certificate signing failures
    /// - File writing failures
    ///
    /// # Security Note
    /// The generated certificates are self-signed and should only be used for development.
    /// Production environments should use properly signed certificates from a Certificate Authority.
    fn generate_certificates(&self) -> io::Result<()> {
        for path in [&self.cert_path, &self.key_path] {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut params: CertificateParams = Default::default();
        params.not_before = date_time_ymd(1975, 1, 1);
        params.not_after = date_time_ymd(4096, 1, 1);
        params.distinguished_name = DistinguishedName::new();
        params
            .distinguished_name
            .push(DnType::OrganizationName, "Xodium Software");
        params.subject_alt_names =
            vec![SanType::DnsName("localhost".try_into().map_err(|e| {
                io::Error::other(format!("Invalid DNS name: {e}"))
            })?)];

        let key_pair = KeyPair::generate()
            .map_err(|e| io::Error::other(format!("Key generation failed: {e}")))?;
        let cert = params
            .self_signed(&key_pair)
            .map_err(|e| io::Error::other(format!("Certificate generation failed: {e}")))?;

        fs::write(&self.cert_path, cert.pem().as_bytes())?;
        fs::write(&self.key_path, key_pair.serialize_pem().as_bytes())?;

        Ok(())
    }
}
