use std::io;
use std::num::NonZeroU32;

#[derive(Debug)]
pub enum RngError {
    IoError(io::Error),
    EntropyError,
    BufferTooLarge,
}

impl From<io::Error> for RngError {
    fn from(error: io::Error) -> Self {
        RngError::IoError(error)
    }
}

pub struct SecureRng {
    buffer: Vec<u8>,
    position: usize,
}

impl SecureRng {
    pub fn new() -> Self {
        SecureRng {
            buffer: Vec::with_capacity(1024), // Preallocate buffer
            position: 0,
        }
    }


    pub fn gen_range(&mut self, min: u32, max: u32) -> Result<u32, RngError> {
        if min >= max {
            return Err(RngError::EntropyError);
        }

        let range = max - min;
        loop {
            let value = self.next_u32()?;
            
            if value >= u32::MAX - (u32::MAX % range) {
                continue;
            }
            
            return Ok(min + (value % range));
        }
    }

    // Generate a random u32
    pub fn next_u32(&mut self) -> Result<u32, RngError> {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }

    pub fn fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), RngError> {
        if buf.len() > 1024 * 1024 {
            return Err(RngError::BufferTooLarge);
        }

        if self.position + buf.len() > self.buffer.len() {
            self.buffer.resize(1024, 0);
            self.position = 0;
            get_random_bytes(&mut self.buffer)?;
        }

        buf.copy_from_slice(&self.buffer[self.position..self.position + buf.len()]);
        self.position += buf.len();
        Ok(())
    }


    pub fn gen_normal(&mut self, mean: f64, std_dev: f64) -> Result<f64, RngError> {
        let u1 = self.next_u32()? as f64 / u32::MAX as f64;
        let u2 = self.next_u32()? as f64 / u32::MAX as f64;
        
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        Ok(mean + std_dev * z)
    }

    pub fn next_nonzero_u32(&mut self) -> Result<NonZeroU32, RngError> {
        loop {
            if let Some(nz) = NonZeroU32::new(self.next_u32()?) {
                return Ok(nz);
            }
        }
    }
}

#[cfg(unix)]
fn get_random_bytes(buf: &mut [u8]) -> Result<(), RngError> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("/dev/urandom")?;
    file.read_exact(buf)?;
    Ok(())
}

#[cfg(windows)]
fn get_random_bytes(buf: &mut [u8]) -> Result<(), RngError> {
    use std::io::Error;
    use std::os::raw::{c_char, c_ulong};
    use std::ptr::null_mut;

    type HCRYPTPROV = usize;

    extern "system" {
        fn CryptAcquireContextA(
            phProv: *mut HCRYPTPROV,
            pszContainer: *const c_char,
            pszProvider: *const c_char,
            dwProvType: c_ulong,
            dwFlags: c_ulong,
        ) -> i32;

        fn CryptGenRandom(
            hProv: HCRYPTPROV,
            dwLen: c_ulong,
            pbBuffer: *mut u8,
        ) -> i32;

        fn CryptReleaseContext(
            hProv: HCRYPTPROV,
            dwFlags: c_ulong,
        ) -> i32;
    }

    const PROV_RSA_FULL: c_ulong = 1;
    const CRYPT_VERIFYCONTEXT: c_ulong = 0xF0000000;

    unsafe {
        let mut h_prov: HCRYPTPROV = 0;
        
        if CryptAcquireContextA(
            &mut h_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_FULL,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            return Err(RngError::IoError(Error::last_os_error()));
        }

        let result = if CryptGenRandom(h_prov, buf.len() as c_ulong, buf.as_mut_ptr()) == 0 {
            Err(RngError::IoError(Error::last_os_error()))
        } else {
            Ok(())
        };

        if CryptReleaseContext(h_prov, 0) == 0 {
            return Err(RngError::IoError(Error::last_os_error()));
        }

        result
    }
}

fn main() -> Result<(), RngError> {
    let mut rng = SecureRng::new();

    println!("Random u32: {}", rng.next_u32()?);

    println!("Random range (1-100): {}", rng.gen_range(1, 101)?);

    println!("Normal distribution (mean=0, std_dev=1): {}", rng.gen_normal(0.0, 1.0)?);

    println!("Non-zero random: {}", rng.next_nonzero_u32()?);
    
    Ok(())
}