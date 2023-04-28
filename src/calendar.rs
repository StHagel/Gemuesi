use crate::gemuese::{Gemuese, Obst, Salat};
use std::collections::HashMap;
use std::hash::Hash;
use time::Month;
use time::Month::*;

use chrono::Datelike;

mod fillers;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Saisonkalender<T: Hash + Eq + PartialEq> {
    frisch: HashMap<T, Vec<Month>>,
    lagerware: HashMap<T, Vec<Month>>,
}

impl<T: Hash + Eq + PartialEq + Clone + From<Gemuese> + From<Obst> + From<Salat>>
    Saisonkalender<T>
{
    #[inline]
    pub fn new() -> Saisonkalender<T> {
        let mut frisch: HashMap<T, Vec<Month>> = HashMap::new();
        let mut lagerware: HashMap<T, Vec<Month>> = HashMap::new();
        if type_eq::<T, Gemuese>() {
            fillers::fill_gemuese(&mut frisch, &mut lagerware);
        } else if type_eq::<T, Obst>() {
            fillers::fill_obst(&mut frisch, &mut lagerware)
        } else if type_eq::<T, Salat>() {
            fillers::fill_salat(&mut frisch, &mut lagerware)
        }
        Saisonkalender { frisch, lagerware }
    }

    #[inline]
    pub fn get_seasonal_frisch(&self) -> Option<Vec<T>> {
        let current_month = get_current_month()?;

        let mut seasonal = vec![];
        for (t, season) in &self.frisch {
            if season.contains(&current_month) {
                seasonal.push(t.clone())
            }
        }

        Some(seasonal)
    }

    #[inline]
    pub fn get_seasonal_lager(&self) -> Option<Vec<T>> {
        let current_month = get_current_month()?;

        let mut lager = vec![];
        for (t, season) in &self.lagerware {
            if season.contains(&current_month) {
                lager.push(t.clone())
            }
        }

        Some(lager)
    }

    #[inline]
    pub fn get_months_for(&self, t: &T) -> Vec<Month> {
        self.frisch[t].clone()
    }
}

#[inline]
fn get_current_month() -> Option<Month> {
    match chrono::Local::now().month() {
        1 => Some(January),
        2 => Some(February),
        3 => Some(March),
        4 => Some(April),
        5 => Some(May),
        6 => Some(June),
        7 => Some(July),
        8 => Some(August),
        9 => Some(September),
        10 => Some(October),
        11 => Some(November),
        12 => Some(December),
        _ => None,
    }
}

pub trait GermanString {
    fn to_german_string(&self) -> String;
}

impl GermanString for Month {
    #[inline]
    fn to_german_string(&self) -> String {
        match self {
            January => "Januar",
            February => "Februar",
            March => "März",
            April => "April",
            May => "Mai",
            June => "Juni",
            July => "Juli",
            August => "August",
            September => "September",
            October => "Oktober",
            November => "November",
            December => "Dezember",
        }
        .to_owned()
    }
}

#[inline]
const fn type_eq<T: ?Sized, U: ?Sized>() -> bool {
    // Helper trait. `VALUE` is false, except for the specialization of the
    // case where `T == U`.
    trait TypeEq<U: ?Sized> {
        const VALUE: bool;
    }

    // Default implementation.
    impl<T: ?Sized, U: ?Sized> TypeEq<U> for T {
        default const VALUE: bool = false;
    }

    // Specialization for `T == U`.
    impl<T: ?Sized> TypeEq<T> for T {
        const VALUE: bool = true;
    }

    <T as TypeEq<U>>::VALUE
}
