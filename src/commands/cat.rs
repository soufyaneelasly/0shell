use std::fs::File;
use std::io::{self, stdin, stdout, BufReader, Read};
use std::path::Path;

pub fn cmd_cat(args: &[&str]) {
    // 🪜 1. Si aucun argument → lire depuis stdin
    if args.is_empty() {
        if let Err(e) = io::copy(&mut stdin(), &mut stdout()) {
            eprintln!("cat: error reading stdin: {}", e);
        }
        return;
    }

    // 🪜 2. Parcourir les arguments
    for file in args {
        // Cas spécial : "-" ou "--" → lire depuis stdin
        if *file == "-" || *file == "--" {
            if let Err(e) = io::copy(&mut stdin(), &mut stdout()) {
                eprintln!("cat: error reading stdin: {}", e);
            }
            continue;
        }

        // 🪜 3. Essayer d'ouvrir le fichier
        let path = Path::new(file);
        let file_handle = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("cat: {}: No such file or directory", file);
                continue;
            }
        };

        // 🪜 4. Lire le contenu efficacement avec BufReader
        let mut reader = BufReader::new(file_handle);
        let mut buffer = Vec::new();
        if let Err(e) = reader.read_to_end(&mut buffer) {
            eprintln!("cat: error reading file '{}': {}", file, e);
            continue;
        }

        // 🪜 5. Afficher le contenu lu
        let content = String::from_utf8_lossy(&buffer);
        print!("{}", content);
    }
}
