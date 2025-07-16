use std::{borrow::Cow, io::Error, path::PathBuf};
use shellexpand::tilde;


pub fn dir_in_context(dir: &PathBuf, recurse: bool) -> bool {
    let mut tmp = dir.clone();

    // popped all directories and found nothing, return false
    if tmp.iter().count() == 0 {
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
    dir_in_context(&tmp, true)
}

pub fn get_dir_context(dir: &PathBuf) -> Option<PathBuf> {
    let mut tmp = dir.clone();

    if tmp.iter().count() == 0 {
        return None;
    }

    tmp.push(".nural");
    if tmp.exists() {
        return Some(tmp);
    }

    tmp.pop();
    tmp.pop();

    get_dir_context(&tmp)
}

pub fn expand_dir(dir: &str) -> String {
    tilde(dir).to_string()
}
