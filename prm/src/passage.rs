use std::path::Path;

use crate::{PrmFile, PrmParseError};

#[derive(Debug, thiserror::Error)]
pub enum PassageParseError {
    #[error("`name` property")]
    Name,
    #[error("`pos_x` property")]
    PosX,
    #[error("`pos_y` property")]
    PosY,
    #[error("`world_source` property")]
    WorldSource,
    #[error("`world_destination` property")]
    WorldDestination,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
}

pub struct Passage {
    /// Название коридора
    pub name: String,
    /// Название мира, в котором расположен коридор
    pub world_src_name: String,
    /// Название мира, в который ведёт коридор
    pub world_dest_name: String,
    /// Абсцисса коридора
    pub pos_x: i32,
    /// Ордината коридора
    pub pos_y: i32,
}

impl Passage {
    fn from_prmrow(row: &str) -> Result<Self, PassageParseError> {
        use PassageParseError::*;

        let mut iter = row.split_whitespace();

        let name = iter.next().ok_or(Name)?.to_owned();
        let world_src_name = iter.next().ok_or(WorldSource)?.to_owned();
        let world_dest_name = iter.next().ok_or(WorldDestination)?.to_owned();
        let pos_x = iter.next().and_then(|x| x.parse().ok()).ok_or(PosX)?;
        let pos_y = iter.next().and_then(|y| y.parse().ok()).ok_or(PosY)?;

        if iter.next().is_some() {
            Err(UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            name,
            world_src_name,
            world_dest_name,
            pos_x,
            pos_y,
        })
    }
}

pub struct TableSpot {
    pub passages: Vec<Passage>,
}

impl PrmFile for TableSpot {
    fn file_name<'a>() -> &'a str {
        "passages.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut passages = vec![];
        for row in rows.iter() {
            passages.push(Passage::from_prmrow(row)?);
        }

        Ok(Self { passages })
    }
}
