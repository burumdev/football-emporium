use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Copy, Clone, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum PagPerPage {
    Ten = 10,
    TwentyFive = 25,
    Fifty = 50,
    Hundred = 100,
    TwoHundredFifty = 250,
}

#[derive(Copy, Clone, Deserialize, Debug)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum HomeAwayOption {
    Both,
    Home,
    Away,
}

impl Default for HomeAwayOption {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Copy, Clone, Deserialize, Default, Debug)]
pub struct QueryParams {
    pub offset: Option<usize>,
    pub per_page: Option<PagPerPage>,
    pub home_away: Option<HomeAwayOption>,
}
