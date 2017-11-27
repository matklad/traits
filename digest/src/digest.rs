use super::{Input, FixedOutput};
use generic_array::GenericArray;
use generic_array::typenum::Unsigned;
#[cfg(feature = "std")]
use std::io;

type Output<N> = GenericArray<u8, N>;

/// The `Digest` trait specifies an interface common for digest functions.
///
/// It's a convinience wrapper around `Input`, `FixedOutput` and `Default`
/// traits. It also provides additional convinience methods.
pub trait Digest: Input + FixedOutput + Default {
    /// Create new hasher instance
    fn new() -> Self {
        Self::default()
    }

    /// Digest input data. This method can be called repeatedly
    /// for use with streaming messages.
    fn input(&mut self, buf: &[u8]) {
        self.process(buf);
    }

    /// Retrieve result and reset hasher instance
    fn result(&mut self) -> Output<Self::OutputSize> {
        self.fixed_result()
    }

    /// Get output size of the hasher
    fn output_size() -> usize {
        Self::OutputSize::to_usize()
    }

    /// Convinience function to compute hash of the `data`. It will handle
    /// hasher creation, data feeding and finalization.
    ///
    /// Example:
    ///
    /// ```rust,ignore
    /// println!("{:x}", sha2::Sha256::digest(b"Hello world"));
    /// ```
    #[inline]
    fn digest(data: &[u8]) -> Output<Self::OutputSize> {
        let mut hasher = Self::default();
        hasher.input(data);
        hasher.fixed_result()
    }

    /// Convinience function to compute hash of the string. It's equivalent to
    /// `digest(input_string.as_bytes())`.
    #[inline]
    fn input_str(str: &str) -> Output<Self::OutputSize> {
        Self::digest(str.as_bytes())
    }

    /// Convinience function which takes `std::io::Read` as a source and computes
    /// value of digest function `D`, e.g. SHA-2, SHA-3, BLAKE2, etc. using 1 KB
    /// blocks.
    ///
    /// Usage example:
    ///
    /// ```rust,ignore
    /// use std::fs;
    /// use sha2::{Sha256, Digest};
    ///
    /// let mut file = fs::File::open("Cargo.toml")?;
    /// let result = Sha256::digest_reader(&mut file)?;
    /// println!("{:x}", result);
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    fn digest_reader(source: &mut io::Read)
        -> io::Result<Output<Self::OutputSize>>
    {
        let mut hasher = Self::default();

        let mut buffer = [0u8; 1024];
        loop {
            let bytes_read = source.read(&mut buffer)?;
            hasher.input(&buffer[..bytes_read]);
            if bytes_read == 0 {
                break;
            }
        }

        Ok(hasher.result())
    }
}

impl<D: Input + FixedOutput + Default> Digest for D {}
