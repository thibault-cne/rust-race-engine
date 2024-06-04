use serde::{Deserialize, Serialize};

use derives::FilterValidation;

use crate::models::graphql::{
    GetCircuitsOpts, GetConstructorStandingsOpts, GetConstructorsOpts, GetDriverStandingsOpts,
    GetDriversOpts, GetLapsOpts, GetPitStopsOpts, GetRacesOpts, GetSeasonsOpts, GetStatusOpts,
    Pagination,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

#[derive(Debug, Default)]
pub struct GetRacesParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetCircuitsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetConstructorStandingsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub constructor_ref: Option<String>,
    pub position: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetConstructorsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_standing: Option<u32>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetDriverStandingsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub driver_ref: Option<String>,
    pub position: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetDriversParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub driver_standing: Option<u32>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetLapsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub driver_ref: Option<String>,
    pub year: u32,
    pub round: u32,
    pub lap_number: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetPitStopsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub driver_ref: Option<String>,
    pub year: u32,
    pub round: u32,
    pub lap_number: Option<u32>,
    pub pit_stop_number: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetSeasonsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub driver_standing: Option<u32>,
    pub constructor_standing: Option<u32>,
}

#[derive(Debug, Default)]
pub struct GetStatusParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

impl From<(GetRacesOpts, Pagination)> for GetRacesParameters {
    fn from(value: (GetRacesOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: opts.circuit_ref,
            driver_ref: opts.driver_ref,
            constructor_ref: opts.constructor_ref,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetCircuitsOpts, Pagination)> for GetCircuitsParameters {
    fn from(value: (GetCircuitsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: None,
            driver_ref: opts.driver_ref,
            constructor_ref: opts.constructor_ref,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetConstructorStandingsOpts, Pagination)> for GetConstructorStandingsParameters {
    fn from(value: (GetConstructorStandingsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            constructor_ref: opts.constructor_ref,
            position: opts.position,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetConstructorsOpts, Pagination)> for GetConstructorsParameters {
    fn from(value: (GetConstructorsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: opts.circuit_ref,
            driver_ref: opts.driver_ref,
            constructor_standing: opts.constructor_standing,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetDriverStandingsOpts, Pagination)> for GetDriverStandingsParameters {
    fn from(value: (GetDriverStandingsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            driver_ref: opts.driver_ref,
            position: opts.position,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetDriversOpts, Pagination)> for GetDriversParameters {
    fn from(value: (GetDriversOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: opts.circuit_ref,
            constructor_ref: opts.constructor_ref,
            driver_standing: opts.driver_standing,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetLapsOpts, Pagination)> for GetLapsParameters {
    fn from(value: (GetLapsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            driver_ref: opts.driver_ref,
            lap_number: opts.lap_number,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetPitStopsOpts, Pagination)> for GetPitStopsParameters {
    fn from(value: (GetPitStopsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            driver_ref: opts.driver_ref,
            lap_number: opts.lap_number,
            pit_stop_number: opts.pit_stop_number,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetSeasonsOpts, Pagination)> for GetSeasonsParameters {
    fn from(value: (GetSeasonsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            driver_ref: opts.driver_ref,
            driver_standing: opts.driver_standing,
            circuit_ref: opts.circuit_ref,
            constructor_ref: opts.constructor_ref,
            constructor_standing: opts.constructor_standing,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
        }
    }
}

impl From<(GetStatusOpts, Pagination)> for GetStatusParameters {
    fn from(value: (GetStatusOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            driver_ref: opts.driver_ref,
            circuit_ref: opts.circuit_ref,
            constructor_ref: opts.constructor_ref,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

macros::query_parameters! {
    #[Copy] Page(u64);
    #[Copy] Limit(u64);
    DriverRef(String) => str;
    ConstructorRef(String) => str;
    CircuitRef(String) => str;
    #[Copy] DriverStanding(u32);
    #[Copy] ConstructorStanding(u32);
    #[Copy] Grid(u32);
    #[Copy] RaceResult(u32);
    #[Copy] Year(u32);
    #[Copy] Round(u32);
    #[Copy] Fastest(i32);
    #[Copy] StatusId(i32);
    #[Copy] LapNumber(u32);
    #[Copy] PitStopNumber(u32);
}

impl Year {
    pub fn get_current_year() -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self(now.year() as u32)
    }
}

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetPitStopsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub year: Year,
    pub round: Round,
    pub lap_number: Option<LapNumber>,
    pub pit_stop_number: Option<PitStopNumber>,
}

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self(30)
    }
}

impl Default for Year {
    fn default() -> Self {
        Self::get_current_year()
    }
}

impl Default for Round {
    fn default() -> Self {
        Self(1)
    }
}

mod macros {
    macro_rules! query_parameters {
        ($(#[$($traits:ident),*])* $name:ident ($type:ty) => $deref:ty; $($rest:tt)*) => {
            #[derive(Debug, Clone, Deserialize $($(, $traits)*)*)]
            pub struct $name(pub $type);

            impl From<$type> for $name {
                fn from(t: $type) -> Self {
                    Self(t)
                }
            }

            impl std::ops::Deref for $name {
                type Target = $deref;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            macros::query_parameters!{ $($rest)* }
        };
        ($(#[$($traits:ident),*])* $name:ident ($type:ty); $($rest:tt)*) => {
            #[derive(Debug, Clone, Deserialize $($(, $traits)*)*)]
            pub struct $name(pub $type);

            impl From<$type> for $name {
                fn from(t: $type) -> Self {
                    Self(t)
                }
            }

            impl std::ops::Deref for $name {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            macros::query_parameters! { $($rest)* }
        };
        () => {};
    }

    pub(super) use query_parameters;
}
