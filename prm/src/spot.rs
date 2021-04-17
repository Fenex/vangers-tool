// TODO: merge spot & escave parsers

use std::path::Path;

use crate::{PrmFile, PrmParseError};

#[derive(Debug, thiserror::Error)]
pub enum SpotParseError {
    #[error("expected first line of the block not found")]
    ExpectedEscaveTitleLine,
    #[error("unexpected additional parameter at first line of the block")]
    UnexpectedAdditionalParameterAtEscaveTitleLine,
    #[error("`name` property")]
    Name,
    #[error("`world` property")]
    World,
    #[error("`pos_x` property")]
    PosX,
    #[error("`pos_y` property")]
    PosY,
    #[error("`personal_item` property")]
    PersonalItem,
    #[error("expected `none` terminate line not found")]
    ExpectedTerminateLine,
    #[error("`goods` line")]
    GoodsLine,
    #[error("unexpected additional parameter at goods line")]
    UnexpectedAdditionalParameterAtGoodsLine,
}

pub struct Spot {
    /// Название эскейва
    pub name: String,
    /// Название мира, в котором расположен эскейв
    pub world_name: String,
    /// Абсцисса эскейва
    pub pos_x: i32,
    /// Ордината эскейва
    pub pos_y: i32,
    /// Личная вещь советика этого эскейва (если есть)
    pub personal_item_name: Option<String>,
    /// Список производимых продуктов в эскейве и места их назначения
    pub goods: Vec<(String, String)>,
}

impl Spot {
    fn from_prmrow_iter<'a, 'b>(
        iter: &'b mut impl Iterator<Item = &'a String>,
    ) -> Result<Self, SpotParseError> {
        use SpotParseError::*;

        let row = iter.next().ok_or(ExpectedEscaveTitleLine)?;
        let mut iter = row.split_whitespace();

        let name = iter.next().ok_or(Name)?.to_owned();
        let world_name = iter.next().ok_or(World)?.to_owned();
        let pos_x = iter.next().and_then(|x| x.parse().ok()).ok_or(PosX)?;
        let pos_y = iter.next().and_then(|y| y.parse().ok()).ok_or(PosY)?;
        let personal_item_name = iter.next().and_then(|s| {
            if s == "none" {
                None
            } else {
                Some(s.to_owned())
            }
        });

        if iter.next().is_some() {
            Err(UnexpectedAdditionalParameterAtEscaveTitleLine)?
        }

        let mut goods = Vec::with_capacity(2);

        while let Some(row) = iter.next() {
            if row == "none" {
                break;
            }

            let mut iter = row.split_whitespace();
            let (item, dest) = iter
                .next()
                .and_then(|item| iter.next().and_then(|dest| Some((item, dest))))
                .ok_or(GoodsLine)?;

            if iter.next().is_some() {
                Err(UnexpectedAdditionalParameterAtGoodsLine)?
            }

            goods.push((item.to_owned(), dest.to_owned()));
        }

        Ok(Self {
            name,
            world_name,
            pos_x,
            pos_y,
            personal_item_name,
            goods,
        })
    }
}

pub struct TableSpot {
    pub spots: Vec<Spot>,
}

impl PrmFile for TableSpot {
    fn file_name<'a>() -> &'a str {
        "spot.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut spots = vec![];

        let mut iter = rows.iter();
        while iter.clone().next().is_some() {
            let spot = Spot::from_prmrow_iter(&mut iter)?;
            spots.push(spot);
        }

        Ok(Self { spots })
    }
}
