use std::{path::Path, str::FromStr};

use ::enum_primitive_derive::Primitive;
use ::num_traits::FromPrimitive;

use crate::{PrmFile, PrmParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum Type {
    Raffa = 0,
    Light = 1,
    Microbus = 2,
    Atw = 3,
    Track = 4,
    Special = 5,
}

pub struct Price {
    pub buy: u32,
    pub sell: u32,
}

/// Характеристики мехоса взятые из файла `car.prm`
pub struct Mechos {
    /// название мехоса
    pub name: String,
    /// тип мехоса,
    pub r#type: Type,
    /// цена мехоса
    pub price: Price,
    /// вместимость (количество слотов для каждого вида)
    pub r#box: (u8, u8, u8, u8),
    pub speed: u32,
    pub armor: u32,
    pub energy: u32,
    pub energy_delta: u32,
    pub energy_drop: u32,
    pub drop_time: u32,
    pub fire: u32,
    pub water: u32,
    pub oxygen: u32,
    pub fly: u32,
    pub damage: u32,
    pub teleport: u32,
    __: (),
}

#[derive(Debug, thiserror::Error)]
pub enum MechosParseError {
    #[error("block `total item number in Chain at one moment` parse error")]
    DigitCounters,
    #[error("field parse error: {0}")]
    FieldParseError(#[from] MechosFieldParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum MechosFieldParseError {
    #[error("name")]
    Name,
    #[error("type: {0}")]
    Type(String),
    #[error("given wrong type: `{0}`")]
    WrongType(u8),
    #[error("price buy")]
    PriceBuy,
    #[error("price sell")]
    PriceSell,
    #[error("box #({0})")]
    Box(usize),
    #[error("speed")]
    Speed,
    #[error("armor")]
    Armor,
    #[error("energy")]
    Energy,
    #[error("energy_delta")]
    EnergyDelta,
    #[error("energy_drop")]
    EnergyDrop,
    #[error("drop_time")]
    DropTime,
    #[error("fire")]
    Fire,
    #[error("water")]
    Water,
    #[error("oxigen")]
    Oxygen,
    #[error("fly")]
    Fly,
    #[error("damage")]
    Damage,
    #[error("teleport")]
    Teleport,
}

impl From<MechosFieldParseError> for PrmParseError {
    fn from(from: MechosFieldParseError) -> Self {
        Self::Mechos(MechosParseError::FieldParseError(from))
    }
}

/// Таблица с характеристиками всех мехосов из файла `car.prm`
pub struct TableMechos {
    mechoses: Vec<Mechos>,
}

impl TableMechos {
    pub fn mechoses(&self) -> &[Mechos] {
        &self.mechoses
    }
}

impl PrmFile for TableMechos {
    fn file_name<'a>() -> &'a str {
        "car.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        use MechosFieldParseError::*;

        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mechoses_total = rows
            .iter()
            .take(3)
            .map(|r| r.parse::<usize>())
            .try_fold(0, |acc, x| x.map(|x| acc + x))
            .map_err(|_| MechosParseError::DigitCounters)?;

        let mut mechoses = Vec::with_capacity(mechoses_total);

        for row in rows.iter().skip(3) {
            let mut values = row.split_whitespace();
            let name = values.next().ok_or(Name)?;

            let r#type = {
                let type_id = values
                    .next()
                    .ok_or(Type(String::from("[expected a value]")))
                    .map(|s| s.parse::<u8>().map_err(|e| Type(e.to_string())))??;

                self::Type::from_u8(type_id).ok_or(WrongType(type_id))?
            };

            let price = Price {
                buy: get_atom(&mut values).ok_or(PriceBuy)?,
                sell: get_atom(&mut values).ok_or(PriceSell)?,
            };

            let boxes = {
                let mut boxes = [0u8; 4];
                for (i, b) in boxes.iter_mut().enumerate() {
                    *b = values
                        .next()
                        .ok_or(Box(i))
                        .map(|v| v.parse().map_err(|_| Box(i)))??;
                }
                (boxes[0], boxes[1], boxes[2], boxes[3])
            };

            let mechos = Mechos {
                name: name.to_owned(),
                r#type,
                price,
                r#box: boxes,
                speed: get_atom(&mut values).ok_or(Speed)?,
                armor: get_atom(&mut values).ok_or(Armor)?,
                energy: get_atom(&mut values).ok_or(Energy)?,
                energy_delta: get_atom(&mut values).ok_or(EnergyDelta)?,
                energy_drop: get_atom(&mut values).ok_or(EnergyDrop)?,
                drop_time: get_atom(&mut values).ok_or(DropTime)?,
                fire: get_atom(&mut values).ok_or(Fire)?,
                water: get_atom(&mut values).ok_or(Water)?,
                oxygen: get_atom(&mut values).ok_or(Oxygen)?,
                fly: get_atom(&mut values).ok_or(Fly)?,
                damage: get_atom(&mut values).ok_or(Damage)?,
                teleport: get_atom(&mut values).ok_or(Teleport)?,
                __: (),
            };

            mechoses.push(mechos);
        }

        Ok(Self { mechoses })
    }
}

#[inline]
fn get_atom<'a, 'b, I, T: FromStr>(iter: &'a mut I) -> Option<T>
where
    I: Iterator<Item = &'b str>,
{
    iter.next()?.parse::<T>().ok()
}
