use std::path::Path;

use crate::{PrmFile, PrmParseError};

#[derive(Debug, thiserror::Error)]
pub enum ItemParseError {
    #[error("title")]
    Title,
    #[error("`name` property")]
    Name,
    #[error("`type` property")]
    Type,
    #[error("`steeler_full` property")]
    SteelerFull,
    #[error("`steeler_empty` property")]
    SteelerEmpty,
    #[error("`size` property")]
    Size,
    #[error("`count` property")]
    Count,
    #[error("`param1` property")]
    Param1,
    #[error("`param2` property")]
    Param2,
    #[error("`title` is set to great then actual items")]
    ExpectedAdditionalItem,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
}

#[derive(Debug, Clone, Copy)]
pub struct SteelerType {
    pub full: i32,
    pub empty: i32,
}

pub struct Item {
    pub name: String,
    pub r#type: i32,
    pub steeler: SteelerType,
    pub size: u32,
    pub count: u32,
    pub param1: i32,
    pub param2: i32,
    __: (),
}

impl Item {
    fn from_prmrow(row: &str) -> Result<Self, ItemParseError> {
        use ItemParseError::*;

        let mut iter = row.split_whitespace();
        let name = iter.next().ok_or(Name)?.to_owned();
        let r#type = iter.next().and_then(|s| s.parse().ok()).ok_or(Type)?;

        let steeler = {
            let full = iter
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(SteelerFull)?;
            let empty = iter
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(SteelerFull)?;
            SteelerType { full, empty }
        };

        let size = iter.next().and_then(|s| s.parse().ok()).ok_or(Size)?;
        let count = iter.next().and_then(|s| s.parse().ok()).ok_or(Count)?;
        let param1 = iter.next().and_then(|s| s.parse().ok()).ok_or(Param1)?;
        let param2 = iter.next().and_then(|s| s.parse().ok()).ok_or(Param2)?;

        if iter.next().is_some() {
            Err(UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            name,
            r#type,
            steeler,
            size,
            count,
            param1,
            param2,
            __: (),
        })
    }
}

pub struct TableItem {
    pub items: Vec<Item>,
    __: (),
}

impl PrmFile for TableItem {
    fn file_name<'a>() -> &'a str {
        "item.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut iter = rows.iter();
        let count = iter
            .next()
            .and_then(|s| s.trim().parse().ok())
            .ok_or(ItemParseError::Title)?;

        let mut items = Vec::with_capacity(count);

        for _ in 0..count {
            let item = iter
                .next()
                .and_then(|s| Some(Item::from_prmrow(&s)))
                .ok_or(ItemParseError::ExpectedAdditionalItem)??;
            items.push(item);
        }

        // if iter.next().is_some() {
        //     Err(ItemParseError::UnexpectedAdditionalItem)?
        // }

        Ok(Self { items, __: () })
    }
}
