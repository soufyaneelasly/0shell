use std::fs;
use std::path::Path;

pub fn cmd_mv(s: &[&str]) {
    if s.len() < 2 {
        eprintln!("mv: missing file operand");
        return;
    }

    let dest_path = Path::new(s.last().unwrap());

    // Si plusieurs fichiers à déplacer et la destination n'est pas un dossier
    if s.len() > 2 && (!dest_path.exists() || !dest_path.is_dir()) {
        eprintln!("mv: target '{}' is not a directory", dest_path.display());
        return;
    }

    // Déplacement de plusieurs fichiers dans un dossier
    if dest_path.is_dir() {
        for file in &s[..s.len()-1] {
            let src = Path::new(file);
            if !src.exists() {
                eprintln!("mv: cannot stat '{}': No such file or directory", file);
                continue;
            }

            // nom du fichier sans le chemin
            let filename = match src.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("mv: invalid path '{}'", file);
                    continue;
                }
            };

            let new_dest = dest_path.join(filename);
            if let Err(e) = fs::rename(src, &new_dest) {
                eprintln!("mv: cannot move '{}': {}", file, e);
            } else {
                println!("Moved '{}' → '{}'", file, new_dest.display());
            }
        }
    } else {
        // Un seul fichier -> renommage ou déplacement simple
        let src = Path::new(s[0]);
        if let Err(e) = fs::rename(src, dest_path) {
            eprintln!("mv: cannot move '{}': {}", src.display(), e);
        }
    }
}
//