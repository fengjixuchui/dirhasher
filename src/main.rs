use clap::{Arg, App};
use walkdir::WalkDir;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Read;
use sha2::{Sha256, Digest};
use hex;

fn main() {
    let matches = App::new(clap::crate_name!())
    .author(clap::crate_authors!())
    .version(clap::crate_version!())
    .about(clap::crate_description!())
    .arg(Arg::with_name("DIR")
         .help("Directory to start from")
         .required(true)
         .index(1))
    .get_matches();

    let dir = fs::canonicalize(matches.value_of("DIR").unwrap()).unwrap();

    println!("Walking directory: {}", dir.display());

    // Stack allocate a 32-byte array to store the SHA256 output
    let mut hash: [u8; 32] = [0; 32];

    for entry in WalkDir::new(dir) {
        match entry {
			Ok(entry) => {
                let file_type = entry.file_type();
                if file_type.is_file() && !file_type.is_symlink() {
                    let path = entry.path();
                    match hash_file(path, &mut hash) {
                        Ok(_) => println!("{},{}", path.display(), hex::encode(hash)),
                        Err(_) => eprintln!("failed to access: {}", path.display())
                    }
                }
			},
            Err(err) => {
				let path = err.path().unwrap_or(Path::new("")).display();
				eprintln!("failed to access: {}", path);
            }
        }
    }
}

fn hash_file(path: &Path, output: &mut[u8]) -> Result<(), io::Error> {
    let mut buffer = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut buffer)?;

    let hash = Sha256::digest(buffer.as_slice());
    output.clone_from_slice(hash.as_slice());

    Ok(())
}
