use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigObj {
    pub general: ConfigObjGeneral,
    pub display: ConfigObjDisplay,
}

#[derive(Deserialize)]
pub struct ConfigObjGeneral {
   pub global_dir: String,
   pub note_extension: String,
   pub auto_create: bool,
   pub editor: String,
   pub update_gitignore: bool,
}

#[derive(Deserialize)]
pub struct ConfigObjDisplay {
   show_full_paths: bool,
   show_timestamps: bool,
}
