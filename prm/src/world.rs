use std::path::Path;

use crate::{PrmFile, PrmParseError};

#[derive(Debug, thiserror::Error)]
pub enum WorldParseError {
    #[error("`name` property")]
    Name,
    #[error("`width` property")]
    Width,
    #[error("`height` property")]
    Height,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
}

pub struct World {
    pub name: String,
    pub width: u32,  // x
    pub height: u32, // y
}

impl World {
    fn from_prmrow(row: &str) -> Result<Self, WorldParseError> {
        let mut iter = row.split_whitespace();

        let name = iter.next().ok_or(WorldParseError::Name)?.to_owned();
        let width = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(WorldParseError::Width)?;
        let height = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(WorldParseError::Height)?;

        if iter.next().is_some() {
            Err(WorldParseError::UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            name,
            height,
            width,
        })
    }
}

/// Таблица со всеми мирами из `world.prm`
pub struct TableWorld {
    worlds: Vec<World>,
}

impl TableWorld {
    pub fn worlds(&self) -> &[World] {
        &self.worlds
    }
}

impl PrmFile for TableWorld {
    fn file_name<'a>() -> &'a str {
        "worlds.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut worlds = vec![];
        for row in rows.iter() {
            worlds.push(World::from_prmrow(row)?);
        }

        Ok(Self { worlds })
    }
}
