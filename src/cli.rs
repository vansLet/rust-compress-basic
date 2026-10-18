// use std::{env::args, path::PathBuf};

// use crate::{PositionDir, cli::{
//     ErrArgs::EmptyArguments,
//     OptionCommands::{Compress, Decompress},
// }};
#[derive(Debug)]
pub enum ErrArgs {
    EmptyArguments(String),
    InvalidEncryptionArguments,
}

use std::{
    env::args,
    path::{Path, PathBuf},
};

use crate::{
    PositionDir,
    cli::{
        ErrArgs::EmptyArguments,
        OptionCommands::{Compress, Decompress},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionCommands {
    Decompress,
    Compress,
}
const VALUE_COMMANDS: [(&'static str, OptionCommands); 2] =
    [("decompress", Decompress), ("compress", Compress)];

impl OptionCommands {
    pub fn parse(val: &str) -> Result<Self, &'static str> {
        for (s, v) in &VALUE_COMMANDS {
            if val.eq_ignore_ascii_case(s) {
                return Ok(*v);
            }
        }
        return Err("format arguments enkripsi salah");
    }
}
#[derive(Debug)]
pub struct CliArguments {
    pub type_encrypt: OptionCommands,
    pub from_path: PathBuf,
    pub to_path: PathBuf,
}
impl CliArguments {
    fn parse_path(root_dir: &Path, p: Option<&String>) -> Option<PathBuf> {
        match p {
            None => None,
            Some(s) => {
                let mut pt = PathBuf::from(root_dir);
                pt.push(s);
                Some(pt)
            }
        }
    }
    pub fn new() -> Result<Self, ErrArgs> {
        let args = args().skip(1).take(3).collect::<Vec<String>>();
        let root_dir = PositionDir::init();
        if args.is_empty() {
            return Err(ErrArgs::EmptyArguments("arguments kosong".to_owned()));
        }
        let command = {
            let val = args
                .get(0)
                .map_or_else(
                    || {
                        Err(ErrArgs::EmptyArguments(
                            "metode enkripsi tidak boleh kosong".to_owned(),
                        ))
                    },
                    |v| Ok(v),
                )?
                .to_owned();
            OptionCommands::parse(&val)
                .or_else(|_s| Err(EmptyArguments("format enkripsi salah".to_owned())))?
        };
        let from_path = Self::parse_path(&root_dir.0, args.get(1).as_deref()).map_or_else(
            || {
                Err(ErrArgs::EmptyArguments(
                    "from-path tidak boleh kosong".to_owned(),
                ))
            },
            |f| Ok(f),
        )?;
        let to_path = Self::parse_path(&root_dir.0, args.get(2).as_deref()).map_or_else(
            || {
                Err(ErrArgs::EmptyArguments(
                    "output-path tidak boleh kosong".to_owned(),
                ))
            },
            |p| Ok(p),
        )?;
        Ok(CliArguments {
            type_encrypt: command,
            from_path,
            to_path,
        })
    }
}
