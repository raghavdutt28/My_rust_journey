use std::fs;
use std::io;

fn main() {
    std::process::exit(logic())
}

fn logic() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <filename> <output_folder>", args[0]);
        return 1;
    }

    let filename = std::path::Path::new(&args[1]);
    let output_folder = std::path::Path::new(&args[2]);

    let file = fs::File::open(filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    fs::create_dir_all(output_folder).unwrap(); // Create the specified output directory

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => output_folder.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File: {}, Comment: {}", i, comment);
            }
        }

        if file.name().ends_with('/') {
            println!("File: {}, extracted to: \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File: {}, extracted to: \"{}\" ({}bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}
