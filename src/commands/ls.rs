use chrono::{DateTime, Local};
use chrono::Datelike; // Pour .year()
use std::env;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Fonction principale cmd_ls
pub fn cmd_ls(s: &[&str]) {
    let mut show_all = false;    // -a
    let mut long_format = false; // -l
    let mut append_types = false; // -F
    let mut paths: Vec<PathBuf> = Vec::new();

    // --- 1. Lecture des arguments ---
    for &arg in s {
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'F' => append_types = true,
                    _ => {
                        eprintln!("ls: invalid option -- '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            paths.push(expand_path(arg));
        }
    }

    // --- 2. Aucun chemin → répertoire courant ---
    if paths.is_empty() {
        paths.push(env::current_dir().unwrap());
    }

    // --- 3. Boucle sur chaque chemin ---
    for (i, path) in paths.iter().enumerate() {
        if !path.exists() {
            println!("ls: cannot access '{}': No such file or directory", path.display());
            continue;
        }

        // Multi-dossiers : afficher entête si plusieurs chemins
        if paths.len() > 1 {
            if i > 0 {
                println!();
            }
            println!("{}:", path.display());
        }

        if path.is_dir() {
            match fs::read_dir(path) {
                Ok(entries) => {
                    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                    entries.sort_by_key(|e| e.file_name());

                    if long_format {
                        print_long_format(&entries, show_all, append_types, path);
                    } else {
                        print_short_format(&entries, show_all, append_types);
                    }
                }
                Err(e) => println!("ls: cannot open directory '{}': {}", path.display(), e),
            }
        } else {
            // Fichier unique
            if long_format {
                match fs::metadata(path) {
                    Ok(meta) => print_long_entry(&meta, path, append_types),
                    Err(e) => println!("ls: cannot access '{}': {}", path.display(), e),
                }
            } else {
                let mut name = path.file_name().unwrap().to_string_lossy().to_string();
                if append_types {
                    name = append_type_suffix(path, name);
                }
                println!("{}", name);
            }
        }
    }
}

// ---------------- Fonctions internes ----------------

fn print_short_format(entries: &[fs::DirEntry], show_all: bool, append_types: bool) {
    let mut display_entries = Vec::new();
    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !show_all && name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        let mut display_name = name.to_string();
        if append_types {
            display_name = append_type_suffix(&path, display_name);
        }
        display_entries.push(display_name);
    }

    // Affichage en colonnes simple
    let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
    let max_len = display_entries.iter().map(|s| s.len()).max().unwrap_or(1);
    let col_width = max_len + 2;
    let cols = std::cmp::max(1, term_width / col_width);
    let rows = (display_entries.len() + cols - 1) / cols;

    for row in 0..rows {
        for col in 0..cols {
            let idx = col * rows + row;
            if idx < display_entries.len() {
                print!("{:<width$}", display_entries[idx], width = col_width);
            }
        }
        println!();
    }
}

fn print_long_format(entries: &[fs::DirEntry], show_all: bool, append_types: bool, dir_path: &Path) {
    // Calcul total blocs
    let mut total_blocks = 0;
    let mut display_entries = Vec::new();

    if show_all {
        for special in [".", ".."] {
            let path = dir_path.join(special);
            if let Ok(meta) = fs::symlink_metadata(&path) {
                total_blocks += meta.blocks();
                display_entries.push((meta, special.to_string(), path));
            }
        }
    }

    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !show_all && name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if let Ok(meta) = entry.metadata() {
            total_blocks += meta.blocks();
            display_entries.push((meta, name.into_owned(), path));
        }
    }

    println!("total {}", total_blocks / 2); // 512-byte blocks → 1K

    for (meta, _name, path) in display_entries {
        print_long_entry(&meta, &path, append_types);
    }
}

fn print_long_entry(meta: &fs::Metadata, path: &Path, append_types: bool) {
    let mode = meta.mode();
    let file_type = file_type_char(meta);
    let perms = format_mode(mode);

    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let size = meta.size();

    let mtime: DateTime<Local> = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();
    let time_str = if mtime.year() == Local::now().year() {
        mtime.format("%b %e %H:%M").to_string()
    } else {
        mtime.format("%b %e  %Y").to_string()
    };

    let mut name = path.file_name().unwrap().to_string_lossy().to_string();
    if append_types {
        name = append_type_suffix(path, name);
    }

    print!("{}{} {:>2} {:>3} {:>3} {:>6} {} {}", file_type, perms, nlink, uid, gid, size, time_str, name);

    // Affichage lien symbolique -> cible
    if file_type == 'l' {
        if let Ok(target) = fs::read_link(path) {
            print!(" -> {}", target.display());
        }
    }

    println!();
}

fn file_type_char(meta: &fs::Metadata) -> char {
    if meta.is_dir() {
        'd'
    } else if meta.file_type().is_symlink() {
        'l'
    } else if meta.file_type().is_socket() {
        's'
    } else if meta.file_type().is_fifo() {
        'p'
    } else {
        '-'
    }
}

fn append_type_suffix(path: &Path, mut name: String) -> String {
    match fs::symlink_metadata(path) {
        Ok(meta) => {
            let ft = meta.file_type();
            if ft.is_dir() { name.push('/'); }
            else if ft.is_symlink() { name.push('@'); }
            else if ft.is_socket() { name.push('='); }
            else if ft.is_fifo() { name.push('|'); }
            else if meta.permissions().mode() & 0o111 != 0 { name.push('*'); }
        }
        Err(_) => {}
    }
    name
}

fn format_mode(mode: u32) -> String {
    let mut perms = String::new();
    let bits = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];
    for (bit, c) in bits.iter() {
        perms.push(if mode & *bit != 0 { *c } else { '-' });
    }
    perms
}

fn expand_path(path: &str) -> PathBuf {
    if path == "~" || path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            if path == "~" { return home; }
            else { return home.join(&path[2..]); }
        }
    }
    PathBuf::from(path)
}
