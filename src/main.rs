use std::{
    collections::HashMap,
    fs::{read_dir, rename},
    path::PathBuf,
};

use clap::{Arg, command};

fn main() {
    let matches = command!().arg(Arg::new("first")).get_matches();

    //let dir_path = PathBuf::from("/home/koushikk/Documents/Rust2/clapnew/");
    let dir_path = PathBuf::from("/home/koushikk/Downloads/");

    let first_op = matches.get_one::<String>("first").unwrap();
    if first_op == &"dir".to_string() {
        dir_list(
            dir_path
                .as_os_str()
                .to_owned()
                .into_string()
                .as_ref()
                .unwrap(),
            dir_path,
        );
    } else {
        dir_list_one(
            dir_path
                .as_os_str()
                .to_owned()
                .into_string()
                .as_ref()
                .unwrap(),
            first_op.to_string(),
        );
    }

    // println!("dir: {:?}", matches.get_one::<String>("dir"));
}
// ok so whats the plan first il make that into a funct
// i also need to make sure this does not touch any programming stuff so add restrictions to like
// only mp4 or pdf .txt etc etc

fn dir_list(path: &str, dir_path: PathBuf) {
    let mut entries = read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort();

    let _dir_path2 = dir_path.clone();
    let mut dirs = HashMap::new();
    let mut files = HashMap::new();

    let mut i = 0;
    let mut ifile = 0;

    entries.iter().for_each(|f| {
        if f.is_dir() {
            // println!("this a dir yp {:?}", f);
            dirs.insert(f, i);
            //let fp = PathBuf::from(f);
            //println!("file stem {:?}", fp.extension());
            i += 1;
        } else {
            files.insert(f, ifile);
            // println!("files {:?}", f);
            ifile += 1;
        }
    });

    //let files_clone = files.i.map(|(e, _)| e.to_owned().to_owned());
    let files_clone: Vec<PathBuf> = files
        .iter()
        .filter_map(|(f, _)| Some(f.to_owned().to_owned()))
        .by_ref()
        .collect();
    let mut files_extenstion_unique = HashMap::new();
    let mut suf2: Vec<_> = files
        .iter()
        .filter_map(|(f, _)| {
            let didwa = f
                .extension()
                .map(|e| e.to_os_string().into_string().unwrap());
            let wtbr = match didwa.clone() {
                Some(t) => t,
                None => {
                    println!("ndada");
                    "nada".to_string()
                }
            };
            files_extenstion_unique.insert(wtbr.clone(), 0);
            didwa
        })
        .collect();

    //println!("FILES HASH {:?}\n", files);
    suf2.sort();
    let (_fupes, _dutes, fp) = check_dupes_comp(&files_clone);

    let mut all_mp4s = Vec::new();
    let mut all_pdfs = Vec::new();

    for full in fp {
        //println!("going into {} Folder", full.extenstion);

        for s in files_extenstion_unique.clone().into_keys() {
            if *s == full.extenstion {
                if full.extenstion.ends_with("mp4") {
                    all_mp4s.push(full.clone());
                }
                if full.extenstion.ends_with("pdf") {
                    all_pdfs.push(full.clone());
                }

                println!(
                    "Folder {},File {} ",
                    full.extenstion,
                    full.full_path.display(),
                );
            }
        }
    }
    for mp4 in all_mp4s {
        println!(
            "Mp4 file {}, into the folder {}",
            mp4.full_path.display(),
            mp4.extenstion
        );
    }

    for pdf in all_pdfs {
        println!(
            " PDF File {}, into the Folder {}",
            pdf.full_path.display(),
            pdf.extenstion
        );
    }
}

fn dir_list_one(path: &str, extention: String) {
    let mut entries = read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort();

    let this_exention = extention.clone();
    let mut dirs = HashMap::new();
    let mut files = HashMap::new();
    let mut i = 0;
    let mut ifile = 0;
    entries.iter().for_each(|f| {
        if f.is_dir() {
            // println!("this a dir yp {:?}", f);
            dirs.insert(f, i);
            //let fp = PathBuf::from(f);
            //println!("file stem {:?}", fp.extension());
            i += 1;
        } else {
            files.insert(f, ifile);
            // println!("files {:?}", f);
            ifile += 1;
        }
    });

    //let files_clone = files.i.map(|(e, _)| e.to_owned().to_owned());
    let mut files_clone: Vec<PathBuf> = files
        .iter()
        .filter_map(|(f, _)| Some(f.to_owned().to_owned()))
        .by_ref()
        .collect();
    files_clone.sort();
    let mut files_extenstion_unique = HashMap::new();
    let mut suf2: Vec<_> = files
        .iter()
        .filter_map(|(f, _)| {
            let didwa = f
                .extension()
                .map(|e| e.to_os_string().into_string().unwrap());
            let wtbr = match didwa.clone() {
                Some(t) => t,
                None => {
                    println!("ndada");
                    "nada".to_string()
                }
            };
            files_extenstion_unique.insert(wtbr.clone(), 0);
            didwa
        })
        .collect();

    //println!("FILES HASH {:?}\n", files);
    suf2.sort();
    let (_fupes, _dutes, fp) = check_dupes_comp(&files_clone);

    let mut all_this = Vec::new();

    for full in fp {
        //println!("going into {} Folder", full.extenstion);

        for s in files_extenstion_unique.clone().into_keys() {
            if *s == full.extenstion {
                if full.extenstion.ends_with(&this_exention) {
                    all_this.push(full.clone());
                }

                // println!(
                //     "Folder {},File {} ",
                //     full.extenstion,
                //     full.full_path.display(),
                // );
            }
        }
    }
    for this in all_this {
        println!(
            "file {}, into the folder {}",
            this.full_path.display(),
            this.extenstion
        );
    }
}

#[derive(Clone)]
struct FilePlus {
    full_path: PathBuf,
    extenstion: String,
}

fn check_dupes<T: Eq + std::hash::Hash + Clone>(vec: &[T]) -> (Vec<T>, Vec<T>) {
    let vec = vec;
    // for file in vec {
    //     let path = PathBuf::from(file.to_owned().clone());
    //     let f = FilePlus {
    //         full_path: path,
    //         extenstion: path.extension(),
    //     };
    // }

    let mut counts = HashMap::new();
    // i need to make a struct which has both the full path and the
    // extension
    for item in vec {
        *counts.entry(item).or_insert(0) += 1;
        // dude hashmaps must be unique
        // its deadass just insert the entry but if you dont
        // (beacause its already in there) then add 1 to the value
    }
    let bomba = counts.clone();

    let herediddy = counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(item, _)| item.clone())
        .collect();

    let folders = bomba.into_iter().map(|(item, _)| item.clone()).collect();

    return (herediddy, folders);
}
fn check_dupes_comp<T: Eq + std::hash::Hash + Clone>(vec: &[T]) -> (Vec<T>, Vec<T>, Vec<FilePlus>)
where
    PathBuf: From<T>,
{
    let vec = vec;

    let mut fp_vec = Vec::new();
    for file in vec {
        let path = PathBuf::from(file.to_owned().clone());
        //println!("{}", path.display());

        let extention = match path.extension() {
            Some(e) => e.to_string_lossy().into_owned(),
            None => "DONOT".to_string(),
        };

        let f = FilePlus {
            full_path: path.clone(),
            extenstion: extention,
        };
        // println!(
        //     "FILES PLUS {} EXTENSTION{:?}",
        //     f.full_path.display(),
        //     f.extenstion
        // );
        fp_vec.push(f);
    }

    let mut counts = HashMap::new();
    // i need to make a struct which has both the full path and the
    // extension
    for item in vec {
        *counts.entry(item).or_insert(0) += 1;
        // dude hashmaps must be unique
        // its deadass just insert the entry but if you dont
        // (beacause its already in there) then add 1 to the value
    }
    let bomba = counts.clone();

    let herediddy = counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(item, _)| item.clone())
        .collect();

    let folders = bomba.into_iter().map(|(item, _)| item.clone()).collect();

    return (herediddy, folders, fp_vec);
}

fn moving() {
    rename(
        "/home/koushikk/Documents/Rust2/clapnew/random.txt",
        "/home/koushikk/Documents/Rust2/clapnew/here/random.txt",
    )
    .unwrap();
    // ok so this defenetly works
    // now i need to make it so that for every type of file we create a hash for that,
    // then all those hashes get their own proper folder
    // then thats step 1 done
    // ? how would i check the files already in folder?
    println!("moved filed diddy blud");
}
