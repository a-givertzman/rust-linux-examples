use std::{fmt::Display, path::Path};
use indexmap::IndexMap;
use num::Num;
use sal_core::{dbg::Dbg, error::Error};
use crate::{field::Field, pair::Pair};
///
/// 
pub struct Cache<T> {
    dbg: Dbg,
    fields: IndexMap<String, Field<T>>,
    exclude: Vec<usize>,
}
//
//
impl<T: Num + PartialOrd + Ord + Copy + Display> Cache<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(parent: impl Into<String>, fields: IndexMap<String, Vec<T>>, exclude: Vec<usize>) -> Self {
        let dbg = Dbg::new(parent, "Cache");
        let fields = fields.into_iter().map(|(key, values)| {
            (key, Field::new(&dbg, values))
        }).collect();
        Self {
            dbg,
            fields,
            exclude,
        }
    }
    ///
    /// Loads data from the file
    pub fn load<P: AsRef<Path>>(parent: impl Into<String>, path: P) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Cache");
        let error = Error::new(&dbg, "load");
        Err(error.err("Not implemented"))
    }
    ///
    /// Stores data into the file
    pub fn store<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "load");
        Err(error.err("Not implemented"))
    }
    ///
    /// Returns the row's, associated with requested arguments
    pub fn get(&self, args: &[(String, T)]) -> Vec<Vec<(String, T)>> {
        let mut result = vec![];
        let mut pairs: IndexMap<(usize, usize), Vec<(String, Pair<T>)>> = IndexMap::new();
        let requested_keys: Vec<&String> = args.iter().map(|(k, _)| k).collect();
        let keys: Vec<String> = self.fields
            .keys()
            .filter(|key| requested_keys.contains(key)).cloned().collect();
        // Collects pairs sorted by them indexes
        for (key, val) in args {
            match self.fields.get(key) {
                Some(field) => {
                    for pair in field.get(*val) {
                        pairs
                            .entry((pair.lower, pair.upper))
                            .or_insert(vec![])
                            .push((key.clone(), pair));
                    }
                }
                None => log::warn!("Cache.get | Requested key `{key}` - is not found"),
            }
        }
        // If on tuple of indexes number of pairs equals to number of requested args => match is found
        for ((lo, up), p) in pairs {
            if p.len() == args.len() {
                let r = self.interpolation(lo, up, p, &keys);
                result.push(r);
            }
        }
        result
    }
    ///
    /// Returns interpolation values
    /// - lower - index of first row
    /// - upper - index of next row
    /// - pairs - values found in associated fields
    /// - keys - keys of fields to be interpolated
    fn interpolation(&self, lower: usize, upper: usize, pairs: Vec<(String, Pair<T>)>, keys: &Vec<String>) -> Vec<(String, T)> {
        let mut result = vec![];

        // Коэффициент из первого найденого значения
        let ratio = pairs.first().unwrap().1.ratio;
        
        // достаем поля (столбики) из которых надо взять значения
        for key in keys {
            if let Some(field) = self.fields.get(key) {
                // lower value from field
                let lo = field.get_by_idx(lower);
                // upper value from field
                let up = field.get_by_idx(upper);
                let delta = up - lo;
                let base = [lo, up].iter().min().unwrap().to_owned();
                let interpolation =  base + delta * ratio;
                result.push((key.to_owned(), interpolation));
            }
        }
        result
    }
}