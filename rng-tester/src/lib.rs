use std::io;

// adding pub makes the function public
pub fn get_random_u32() -> io::Result<u32> {
    let mut buf = [0u8; 4];
    get_random_bytes(&mut buf)?;
    Ok(u32::from_ne_bytes(buf))
}

#[cfg(unix)]
fn get_random_bytes(buf: &mut [u8]) -> io::Result<()> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("/dev/urandom")?;
    file.read_exact(buf)?;
    Ok(())
}

#[cfg(windows)]
fn get_random_bytes(buf: &mut [u8]) -> io::Result<()> {
    use std::io::Error;
    use std::os::raw::{c_char, c_ulong};
    use std::ptr::null_mut;

    type HCRYPTPROV = usize;

    #[allow(non_snake_case)]
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
            return Err(Error::last_os_error());
        }

        if CryptGenRandom(h_prov, buf.len() as c_ulong, buf.as_mut_ptr()) == 0 {
            CryptReleaseContext(h_prov, 0);
            return Err(Error::last_os_error());
        }

        if CryptReleaseContext(h_prov, 0) == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generation() {
        match get_random_u32() {
            Ok(_) => assert!(true),
            Err(e) => panic!("Failed to generate random number: {}", e),
        }
    }
}