mod reward;
mod task;

use std::convert::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString, EnumIter};

pub use reward::{RewardIcon, RewardCategory};
pub use task::{TaskIcon, TaskCategory};

#[derive(Display, EnumString, EnumIter, Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    #[strum(serialize = "r")]
    Red,
    #[strum(serialize = "g")]
    Green,
    #[strum(serialize = "b")]
    Blue,
    #[strum(serialize = "y")]
    Yellow,
}

pub trait BadgeIcon<CAT>
    where CAT: Clone
{
    fn new_ptr(cat: CAT, color: Color) -> Box<Self>;
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
    fn get_category(&self) -> CAT;
    fn set_category(&mut self, color: CAT);
}