use crate::{PrmFile, PrmParseError};
use std::{collections::HashMap, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum TabutaskParseError {
    #[error("`cash` property")]
    Cash,
    #[error("`luck` property")]
    Luck,
    #[error("`cycle` property")]
    Cycle,
    #[error("`target` property")]
    Target,
    #[error("`work` property")]
    Work,
    #[error("`item` property")]
    Item,
    #[error("`count` property")]
    Count,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
    #[error("title of a block that includes an escave name is not found")]
    ExpectedTitleBlock,
}

pub struct Tabutask {
    pub name: String,
    pub buy: u32,
    pub sell: u32,
}

impl Tabutask {
    fn from_prmrow(_row: &str) -> Result<Self, TabutaskParseError> {
        unimplemented!();
    }
}

pub struct TableTabutask {
    pub tabutasks: HashMap<String, Vec<Tabutask>>,
}

impl PrmFile for TableTabutask {
    fn file_name<'a>() -> &'a str {
        "tabutask.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut tabutasks = HashMap::new();
        let mut tmp_vec = None;
        let mut curr_escave = None;
        for row in rows.iter() {
            if row.split_whitespace().count() == 1 {
                // escave name detected
                if let (Some(k), Some(v)) = (curr_escave.take(), tmp_vec.take()) {
                    tabutasks.insert(k, v);
                }

                tmp_vec = Some(vec![]);
                curr_escave = Some(row.to_owned());
                continue;
            }

            if curr_escave.is_none() {
                Err(TabutaskParseError::ExpectedTitleBlock)?
            }

            let tabutask = Tabutask::from_prmrow(row)?;
            tmp_vec.as_mut().unwrap().push(tabutask);
        }

        if let (Some(k), Some(v)) = (curr_escave.take(), tmp_vec.take()) {
            tabutasks.insert(k, v);
        }

        Ok(Self { tabutasks })
    }
}
