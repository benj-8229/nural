use crate::models::note::Note;
use shellexpand::tilde;
use std::{fs, path::PathBuf};

pub fn context_in_dir(dir: &PathBuf, recurse: bool) -> bool {
    let mut tmp = dir.clone();

    // popped all directories and found nothing, return false
    if tmp.iter().count() <= 1 {
        return false;
    }

    // check if .nural exists in current dir
    tmp.push(".nural");
    if tmp.exists() {
        return true;
    }

    // either recurse to parent of dir or immediately return false
    if recurse == false {
        return false;
    }
    tmp.pop();
    tmp.pop();
    context_in_dir(&tmp, true)
}

pub fn get_dir_context(dir: &PathBuf) -> Option<Context> {
    let mut tmp = dir.clone();

    if tmp.iter().count() <= 1 {
        return None;
    }

    tmp.push(".nural");
    if tmp.exists() {
        return Some(Context::new(tmp));
    }

    tmp.pop();
    tmp.pop();

    get_dir_context(&tmp)
}

pub fn _expand_dir(dir: &str) -> String {
    tilde(dir).to_string()
}

pub struct Context {
    pub dir: PathBuf,
    pub notes: Vec<Note>,
}

impl Context {
    fn new(dir: PathBuf) -> Self {
        let files = fs::read_dir(&dir).unwrap();
        let file_list: Vec<PathBuf> = files
            .map(|file| file.unwrap().path())
            .collect::<Vec<PathBuf>>();
        let notes: Vec<Note> = file_list
            .into_iter()
            .map(|path| {
                Note::from(
                    String::from(path.file_stem().unwrap().to_string_lossy()),
                    path,
                )
            })
            .collect::<Vec<Note>>();

        Context { dir, notes }
    }
}
