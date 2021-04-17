// TODO: merge spot & escave parsers

use std::{collections::HashMap, path::Path};

use crate::{PrmFile, PrmParseError};

#[derive(Debug, thiserror::Error)]
pub enum VangersWeightParseError {
    #[error("`vangers_total` property")]
    VangersTotal,
    #[error("one of a line with relative weights is bad")]
    RelativeWeight,
    #[error("unexpected additional parameter at vangers_total line")]
    UnexpectedAdditionalParameterAtRelativeWeightLine,
}

pub struct TableVangersWeight {
    /// total c-vangers number in Chain at one moment
    pub vangers_total: u32,
    /// relative weight of total world c-vanger density
    /// (String) world -> (u32) relative weight
    pub weights: HashMap<String, u32>,
}

impl PrmFile for TableVangersWeight {
    fn file_name<'a>() -> &'a str {
        "vangers.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        use VangersWeightParseError::*;

        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;
        let mut iter = rows.iter();

        let vangers_total = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(VangersTotal)?;

        let mut weights = HashMap::new();
        while let Some(row) = iter.next() {
            let mut iter = row.split_whitespace();

            let (world, weight) = iter
                .next()
                .and_then(|world| {
                    iter.next().and_then(|weight| {
                        if let Ok(weight) = weight.parse() {
                            Some((world, weight))
                        } else {
                            None
                        }
                    })
                })
                .ok_or(RelativeWeight)?;

            if iter.next().is_some() {
                Err(UnexpectedAdditionalParameterAtRelativeWeightLine)?
            }

            weights.insert(world.to_owned(), weight);
        }

        Ok(Self {
            vangers_total,
            weights,
        })
    }
}
