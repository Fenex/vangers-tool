use crate::{PrmFile, PrmParseError};
use std::{collections::HashMap, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum PriceParseError {
    #[error("`name` property")]
    Name,
    #[error("`buy` property")]
    Buy,
    #[error("`sell` property")]
    Sell,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
    #[error("title of a block that includes an escave name is not found")]
    ExpectedTitleBlock,
}

pub struct Price {
    pub name: String,
    pub buy: u32,
    pub sell: u32,
}

impl Price {
    fn from_prmrow(row: &str) -> Result<Self, PriceParseError> {
        let mut iter = row.split_whitespace();

        let name = iter.next().ok_or(PriceParseError::Name)?.to_owned();
        let buy = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(PriceParseError::Buy)?;
        let sell = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(PriceParseError::Sell)?;

        if iter.next().is_some() {
            Err(PriceParseError::UnexpectedAdditionalParameter)?
        }

        Ok(Price { name, buy, sell })
    }
}

pub struct TablePrice {
    pub prices: HashMap<String, Vec<Price>>,
}

impl PrmFile for TablePrice {
    fn file_name<'a>() -> &'a str {
        "price.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut prices = HashMap::new();
        let mut tmp_vec = None;
        let mut curr_shop = None;
        for row in rows.iter() {
            if row.split_whitespace().count() == 1 {
                // escave name detected
                if let (Some(k), Some(v)) = (curr_shop.take(), tmp_vec.take()) {
                    prices.insert(k, v);
                }

                tmp_vec = Some(vec![]);
                curr_shop = Some(row.to_owned());
                continue;
            }

            if curr_shop.is_none() {
                Err(PriceParseError::ExpectedTitleBlock)?
            }

            let price = Price::from_prmrow(row)?;
            tmp_vec.as_mut().unwrap().push(price);
        }

        if let (Some(k), Some(v)) = (curr_shop.take(), tmp_vec.take()) {
            prices.insert(k, v);
        }

        Ok(Self { prices })
    }
}
