use clap::{Arg, App};
use walkdir::WalkDir;
use std::path::Path;
use std::fs;
use std::io;
use std::io::{BufWriter, Write};
use sha2::{Sha256, Digest};
use hex;

fn main() -> Result<(), io::Error> {
    let matches = App::new(clap::crate_name!())
    .author(clap::crate_authors!())
    .version(clap::crate_version!())
    .about(clap::crate_description!())
    .arg(Arg::with_name("DIR")
         .help("Directory to hash")
         .required(true)
         .index(1))
    .arg(Arg::with_name("OUTFILE")
         .help("Output file")
         .required(true)
         .index(2))
    .get_matches();

    let dir = fs::canonicalize(matches.value_of("DIR").unwrap()).unwrap();
    let outfile = matches.value_of("OUTFILE").unwrap();

    let f = fs::File::create(outfile).unwrap();
    let mut f = BufWriter::new(f);


    println!("Hashing directory: {}", dir.display());
    println!("Writing output to {}", outfile);

    // Stack allocate a 32-byte array to store the SHA256 output
    let mut hash: [u8; 32] = [0; 32];

    for entry in WalkDir::new(dir) {
        match entry {
			Ok(entry) => {
                let file_type = entry.file_type();
                if file_type.is_file() && !file_type.is_symlink() {
                    let path = entry.path();
                    match hash_file(path, &mut hash) {
                        Ok(_) => {
                            write!(f, "{},{}\n", path.display(), hex::encode(hash))?;
                        },
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

    f.flush()?;
    Ok(())
}

fn hash_file(path: &Path, output: &mut[u8]) -> Result<(), io::Error> {
    let file_content = fs::read(path)?;

    let hash = Sha256::digest(file_content.as_slice());
    output.clone_from_slice(hash.as_slice());

    Ok(())
}
