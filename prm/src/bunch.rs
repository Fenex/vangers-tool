use ::enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use std::{path::Path, str::FromStr};

use crate::{PrmFile, PrmParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum Bios {
    Eleepods = 0,
    Beeboorats = 1,
    Zeexes = 2,
}

impl Bios {
    /// Итератор по всем существующим в PRM-файле биосам
    pub fn into_iter() -> impl Iterator<Item = Bios> {
        BiosIntoIterator(0)
    }

    /// Количество биосов
    pub fn total() -> usize {
        Self::into_iter().count()
    }
}

struct BiosIntoIterator(usize);

impl Iterator for BiosIntoIterator {
    type Item = Bios;

    fn next(&mut self) -> Option<Self::Item> {
        Bios::from_usize(self.0).and_then(|bios| {
            self.0 += 1;
            Some(bios)
        })
    }
}

impl FromStr for Bios {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Eleepods" => Bios::Eleepods,
            "Beeboorats" => Bios::Beeboorats,
            "Zeexes" => Bios::Zeexes,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum CultGameType {
    Race = 0,
    Harvest = 1,
}

impl FromStr for CultGameType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RACE" => CultGameType::Race,
            "HARVEST" => CultGameType::Harvest,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CultGameHarvestParseError {
    #[error("game type name")]
    GameTypeName,
    #[error("goods name")]
    GoodsName,
    #[error("goods count")]
    GoodsCount,
    #[error("destination")]
    Destination,
    #[error("rotten goods name")]
    RottenGoodsName,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
}

pub trait CultGameTrait {
    fn get_type(&self) -> CultGameType;
}

pub struct CultGameHarvest {
    /// Название товара, учитываемого в гонке
    pub goods_type_name: String,
    /// Количество товара (?)
    pub goods_count: u32,
    /// Название конечного пункта назначения для товара
    pub destination_name: String,
    /// Название испорченного товара
    pub rotten_goods_type_name: String,
}

impl CultGameHarvest {
    /// Tries create CultGame from row of the file PRM format
    fn from_prmrow(row: &str) -> Result<Self, CultGameHarvestParseError> {
        let mut iter = row.split_whitespace();
        if iter.next().unwrap() != "HARVEST" {
            Err(CultGameHarvestParseError::GameTypeName)?;
        }

        let goods_type_name = iter.next().ok_or(CultGameHarvestParseError::GoodsName)?;
        let goods_count = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(CultGameHarvestParseError::GoodsCount)?;

        let destination_name = iter.next().ok_or(CultGameHarvestParseError::Destination)?;

        let rotten_goods_type_name = iter
            .next()
            .ok_or(CultGameHarvestParseError::RottenGoodsName)?;

        if iter.next().is_some() {
            Err(CultGameHarvestParseError::UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            goods_type_name: goods_type_name.to_owned(),
            goods_count,
            destination_name: destination_name.to_owned(),
            rotten_goods_type_name: rotten_goods_type_name.to_owned(),
        })
    }
}

impl CultGameTrait for CultGameHarvest {
    fn get_type(&self) -> CultGameType {
        CultGameType::Harvest
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CultGameRaceParseError {
    #[error("game type name")]
    GameTypeName,
    #[error("source name")]
    SoutceName,
    #[error("destination name")]
    DestinationName,
    #[error("goods-begin name")]
    GoodsBeginName,
    #[error("goods-end name")]
    GoodsEndName,
    #[error("goods-begin count")]
    GoodsBeginCount,
    #[error("goods-end count")]
    GoodsEndgCount,
    #[error("rotten goods name")]
    RottenGoodsName,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
}

pub struct CultGameRace {
    /// Название отправного пункта в гонке
    pub source_name: String,

    /// Название конечного пункта в гонке
    pub destination_name: String,

    /// Название отпускаемого товара
    pub goods_type_beg_name: String,

    /// Количество отпускаемого товара
    pub goods_count_beg: u32,

    /// Название принимаемого товара
    pub goods_type_end_name: String,

    /// Количество принимаемого товара
    pub goods_count_end: u32,

    /// Название испорченного товара
    pub rotten_goods_type_name: String,
}

impl CultGameRace {
    /// Tries create CultGame from row of the file PRM format
    fn from_prmrow(row: &str) -> Result<Self, CultGameRaceParseError> {
        use CultGameRaceParseError::*;

        let mut iter = row.split_whitespace();
        if iter.next().unwrap() != "RACE" {
            Err(GameTypeName)?
        }

        let source_name = iter.next().ok_or(SoutceName)?.to_owned();
        let goods_type_beg_name = iter.next().ok_or(GoodsBeginName)?.to_owned();
        let goods_count_beg = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(GoodsBeginCount)?;

        let destination_name = iter.next().ok_or(DestinationName)?.to_owned();
        let goods_type_end_name = iter.next().ok_or(GoodsEndName)?.to_owned();
        let goods_count_end = iter
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or(GoodsEndgCount)?;

        let rotten_goods_type_name = iter.next().ok_or(RottenGoodsName)?.to_owned();

        if iter.next().is_some() {
            Err(UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            source_name,
            goods_type_beg_name,
            goods_count_beg,
            destination_name,
            goods_type_end_name,
            goods_count_end,
            rotten_goods_type_name,
        })
    }
}

impl CultGameTrait for CultGameRace {
    fn get_type(&self) -> CultGameType {
        CultGameType::Race
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CultGameParseError {
    #[error("Harvest cult game: [{0}]")]
    Harvest(#[from] CultGameHarvestParseError),
    #[error("Race cult game: [{0}]")]
    Race(#[from] CultGameRaceParseError),
    #[error("Incorrect type of a cult game")]
    IncorrectGameType,
    #[error("incorrect count properties (gametype `{0:?}`)")]
    IncorrectCountProperties(Option<CultGameType>),
    #[error("empty data")]
    Empty,
}

pub enum CultGame {
    Harvest(CultGameHarvest),
    Race(CultGameRace),
}

impl CultGame {
    /// Tries create CultGame from row of the file PRM format
    fn from_prmrow(row: &str) -> Result<Option<Self>, CultGameParseError> {
        use CultGameParseError::*;

        let mut iter = row.split_whitespace();

        match (iter.next(), iter.count()) {
            (None, _) => Err(Empty),
            (Some("none"), i) if i == 0 => Ok(None),
            (Some("none"), _) => Err(IncorrectCountProperties(None)),
            (Some("HARVEST"), i) if i == 4 => {
                let game = CultGameHarvest::from_prmrow(row)?;
                Ok(Some(Self::Harvest(game)))
            }
            (Some("HARVEST"), _) => Err(IncorrectCountProperties(Some(CultGameType::Harvest))),
            (Some("RACE"), i) if i == 7 => {
                let game = CultGameRace::from_prmrow(row)?;
                Ok(Some(Self::Race(game)))
            }
            (Some("RACE"), _) => Err(IncorrectCountProperties(Some(CultGameType::Race))),
            (Some(_), _) => Err(IncorrectGameType),
        }
    }
}

impl CultGameTrait for CultGame {
    fn get_type(&self) -> CultGameType {
        match self {
            CultGame::Harvest(_) => CultGameType::Harvest,
            CultGame::Race(_) => CultGameType::Race,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CultStageParseError {
    #[error("`name` property")]
    Name,
    #[error("`cirt` property")]
    Cirt,
    #[error("`time` property")]
    Time,
    #[error("`price koeff` property")]
    Price,
    #[error("path to `.pal` file")]
    Palette,
    #[error("unexpected additional parameter")]
    UnexpectedAdditionalParameter,
    #[error("empty data")]
    Empty,
}

/// Описание цикла
pub struct CultStage {
    /// Название цикла
    pub name: String,
    /// количество нюхи (cirt), необходимое для завершения периода
    pub cirt: u32,
    /// время полураспада в мин
    pub time: u32,
    /// коэффициент цен
    pub price: u32,
    /// путь к файлу ресурсов с описанием палитры для текущего цикла
    pub palette: String,
}

impl CultStage {
    /// Tries create CultStage from row of the file PRM format
    fn from_prmrow(row: &str) -> Result<Self, CultStageParseError> {
        use CultStageParseError::*;

        let mut iter = row.split_whitespace();
        let name = iter
            .next()
            .and_then(|s| Some(s.trim_start_matches('"').trim_end_matches('"')))
            .ok_or(Name)?
            .to_owned();

        let cirt = iter.next().and_then(|s| s.parse().ok()).ok_or(Cirt)?;
        let time = iter.next().and_then(|s| s.parse().ok()).ok_or(Time)?;
        let price = iter.next().and_then(|s| s.parse().ok()).ok_or(Cirt)?;
        let palette = iter.next().ok_or(Name)?.to_owned();

        if iter.next().is_some() {
            Err(UnexpectedAdditionalParameter)?
        }

        Ok(Self {
            name,
            cirt,
            time,
            price,
            palette,
        })
    }
}

pub struct Cult {
    /// Цикл
    stage: CultStage,
    /// Культовая гонка соответсвюущая циклу (если есть)
    game: Option<CultGame>,
}

impl Cult {
    /// Описание цикла
    pub fn stage(&self) -> &CultStage {
        &self.stage
    }

    /// Культовая гонка соответсвюущая циклу (если есть)
    pub fn game(&self) -> Option<&CultGame> {
        self.game.as_ref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BunchParseError {
    #[error("title of the bunch block corrupt")]
    Title,
    #[error("cult stage: {0}")]
    CultStage(#[from] CultStageParseError),
    #[error("cult game: {0}")]
    CultGame(#[from] CultGameParseError),
}

pub struct Bunch {
    /// Название биоса, к которому относится банч
    pub bios: Bios,
    /// Число периодов в цикле банча
    // pub cycles: u8,
    /// Название эскейва, в котором находится банч
    pub escave_name: String,
    /// Список циклов и их культовые гонки
    pub cults: Vec<Cult>,
    __: (),
}

impl Bunch {
    /// Число периодов в цикле банча
    pub fn cycles(&self) -> usize {
        self.cults.len()
    }

    fn from_prmrow_iter<'a, 'b>(
        iter: &'b mut impl Iterator<Item = &'a String>,
    ) -> Result<Self, BunchParseError> {
        let title = iter.next().ok_or(BunchParseError::Title)?;

        let mut title_iter = title.split_whitespace();

        let escave_name = title_iter
            .next()
            .and_then(|s| if s.trim().len() == 0 { None } else { Some(s) })
            .ok_or(BunchParseError::Title)?
            .to_owned();
        let bios = title_iter
            .next()
            .and_then(|bios_index| bios_index.parse().ok())
            .and_then(|bios_index| Bios::from_u8(bios_index))
            .ok_or(BunchParseError::Title)?;
        let cycles = title_iter
            .next()
            .and_then(|c| c.parse().ok())
            .and_then(|c| if c == 0 { None } else { Some(c) })
            .ok_or(BunchParseError::Title)?;

        if title_iter.next().is_some() {
            Err(BunchParseError::Title)?
        }

        let mut cults: Vec<Cult> = Vec::with_capacity(cycles as usize);

        for _ in 0..cycles {
            let stage = iter.next().map_or_else(
                || Err(CultStageParseError::Empty),
                |s| CultStage::from_prmrow(s),
            )?;

            let game = iter.next().map_or_else(
                || Err(CultGameParseError::Empty),
                |s| CultGame::from_prmrow(s),
            )?;

            cults.push(Cult { stage, game });
        }

        Ok(Self {
            bios,
            cults,
            escave_name,
            __: (),
        })
    }
}

pub struct TableBunch {
    pub bunches: Vec<Bunch>,
    __: (),
}

impl TableBunch {
    pub fn len(&self) -> usize {
        self.bunches.len()
    }
}

impl PrmFile for TableBunch {
    fn file_name<'a>() -> &'a str {
        "bunches.prm"
    }

    fn file_parse<P: AsRef<Path>>(path_to_folder: P) -> Result<Self, PrmParseError> {
        let rows = Self::file_open(path_to_folder).map_err::<PrmParseError, _>(|e| e.into())?;

        let mut iter = rows.iter();
        let count = Bios::total();
        let mut bunches = Vec::with_capacity(count);
        for _ in 0..count {
            bunches.push(Bunch::from_prmrow_iter(&mut iter)?);
        }

        Ok(Self { bunches, __: () })
    }
}
