use std::{
    io::{BufRead, BufReader},
    path::Path,
};

use nom::FindSubstring;

mod bunch;
mod escave;
mod item;
mod mechos;
mod passage;
mod price;
mod spot;
mod tabutask;
mod vangers;
mod world;

pub use bunch::*;
pub use escave::*;
pub use item::*;
pub use mechos::*;
pub use passage::*;
pub use price::*;
pub use spot::*;
pub use tabutask::*;
pub use vangers::*;
pub use world::*;

pub fn read_without_comments<R: BufRead>(fin: R) -> Vec<String> {
    let mut is_comment_block = false;
    let mut lines = vec![];

    for line in fin.lines() {
        if let Ok(mut line) = line {
            loop {
                let cline = line.trim();
                if cline.is_empty() {
                    break;
                }

                if !is_comment_block {
                    if let Some(start_pos) = cline.find_substring("/*") {
                        line = match (&cline[start_pos..]).find_substring("*/") {
                            Some(end_pos) => {
                                format!(
                                    "{}{}",
                                    &cline[..start_pos],
                                    &cline[start_pos + end_pos + 2..]
                                )
                            }
                            None => {
                                is_comment_block = true;
                                String::from(&cline[..start_pos])
                            }
                        };
                        continue;
                    }
                } else {
                    if let Some(end_pos) = cline.find_substring("*/") {
                        line = String::from(&cline[end_pos + 2..]);
                        is_comment_block = false;
                        continue;
                    } else {
                        break;
                    }
                }

                if let Some(pos_end) = cline.find_substring("//") {
                    line = String::from(&cline[..pos_end]);
                    continue;
                }

                lines.push(cline.to_owned());
                break;
            }
        }
    }

    lines
}

#[derive(Debug, thiserror::Error)]
pub enum PrmOpenError {
    #[error("can't open a file to read: `{0}`")]
    IO(#[from] std::io::Error),
    #[error("wrong signature")]
    WrongSignature,
}

#[derive(Debug, thiserror::Error)]
pub enum PrmParseError {
    #[error("parse error: {0}")]
    OpenFile(#[from] PrmOpenError),
    #[error("mechos parse error: {0}")]
    Mechos(#[from] MechosParseError),
    #[error("bunch parse error: {0}")]
    Bunch(#[from] BunchParseError),
    #[error("item parse error: {0}")]
    Item(#[from] ItemParseError),
    #[error("world parse error: {0}")]
    World(#[from] WorldParseError),
    #[error("price parse error: {0}")]
    Price(#[from] PriceParseError),
    #[error("escave parse error: {0}")]
    Escave(#[from] EscaveParseError),
    #[error("spot parse error: {0}")]
    Spot(#[from] SpotParseError),
    #[error("passage parse error: {0}")]
    Passage(#[from] PassageParseError),
    #[error("vangers-weight parse error: {0}")]
    VangersWeight(#[from] VangersWeightParseError),
    #[error("tabutask parse error: {0}")]
    Tabutask(#[from] TabutaskParseError),
}

pub trait PrmFile
where
    Self: Sized,
{
    fn file_name<'a>() -> &'a str;

    fn file_open<P: AsRef<Path>>(path_to_file: P) -> Result<Vec<String>, PrmOpenError> {
        let file = std::fs::File::open(path_to_file.as_ref().join(Self::file_name()))
            .map_err(|e| PrmOpenError::IO(e))?;
        let fin = BufReader::new(file);
        let mut rows = read_without_comments(fin);

        if rows.len() == 0 || rows[0] != "uniVang-ParametersFile_Ver_1" {
            return Err(PrmOpenError::WrongSignature);
        }

        Vec::remove(&mut rows, 0);

        Ok(rows)
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError>;
}
