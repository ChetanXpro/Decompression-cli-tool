use std::fs;
use std::io;
use std::io::Read;

fn main() {
    std::process::exit(decode_file());
}

fn decode_file() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    println!("{:#?}", args[1]);

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 0;
    }

    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        println!("{:#?}", i);
        let mut file = archive.by_index(i).unwrap();

        // println!("{:#?}", file[i]);

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

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

    return 0;
}
