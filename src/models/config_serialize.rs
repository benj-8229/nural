#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigObj {
    pub general: ConfigObjGeneral,
    pub display: ConfigObjDisplay,
}

#[derive(Deserialize)]
pub struct ConfigObjGeneral {
    pub note_extension: String,
    pub editor: String,
    pub reader: String,
    pub lister: String,
    pub update_gitignore: bool,
}

#[derive(Deserialize)]
pub struct ConfigObjDisplay {
    show_full_paths: bool,
    show_timestamps: bool,
}
