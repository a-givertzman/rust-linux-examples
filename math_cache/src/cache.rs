use std::{collections::HashMap, fmt::Display, fs::OpenOptions, io::{Read, Write}, path::Path};
use indexmap::IndexMap;
use num::Num;
use sal_core::{dbg::Dbg, error::Error};
use crate::{field::Field, pair::Pair};
use bincode::{Encode, Decode};
///
/// 
pub struct Cache<T> {
    dbg: Dbg,
    fields: IndexMap<String, Field<T>>,
    exclude: Vec<usize>,
}
//
//
impl<T: Num + PartialOrd + Copy + Display + Encode + Decode<()>> Cache<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(parent: impl Into<String>, fields: impl Into<IndexMap<String, Vec<T>>>, exclude: Vec<usize>) -> Self {
        let dbg = Dbg::new(parent, "Cache");
        let fields = fields.into().into_iter().map(|(key, values)| {
            (key, Field::new(&dbg, values))
        }).collect();
        Self {
            dbg,
            fields,
            exclude,
        }
    }
    ///
    /// Returns all containing fields
    pub fn fields(&self) -> IndexMap<String, Field<T>> {
        self.fields.clone()
    }
    ///
    /// Loads data from the file
    pub fn load<P: AsRef<Path>>(parent: impl Into<String>, path: P) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Cache");
        let error = Error::new(&dbg, "load");
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| error.pass(e.to_string()))?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .map_err(|e| error.pass(e.to_string()))?;
        let (cache, _): (_Cache<T>, _) = bincode::decode_from_slice(&buf, bincode::config::standard())
            .map_err(|e| error.pass(e.to_string()))?;
        Ok(Self {
            fields: cache.fields.iter().map(|(k, f)| (k.to_owned(), Field::new(&dbg, f.values.to_owned()))).collect(),
            exclude: cache.exclude,
            dbg,
        })
    }
    ///
    /// Stores data into the file
    pub fn store<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "store");
        let cache = _Cache {
            fields: self.fields.iter().map(|(k, f)| (k.to_owned(), _Field { values: f.values() })).collect(),
            exclude: self.exclude.clone(),
        };
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| error.pass(e.to_string()))?;
        let buf = bincode::encode_to_vec(cache, bincode::config::standard())
            .map_err(|e| error.pass(e.to_string()))?;
        file.write_all(&buf)
            .map_err(|e| error.pass(e.to_string()))
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
                let base = [lo, up].iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_owned();
                let interpolation =  base + delta * ratio;
                result.push((key.to_owned(), interpolation));
            }
        }
        result
    }
}
///
/// Used for binarisation to be stored / loaded
#[derive(Encode, Decode)]
struct _Field<T> {
    values: Vec<T>,
}
#[derive(Encode, Decode)]
struct _Cache<T> {
    fields: HashMap<String, _Field<T>>,
    exclude: Vec<usize>,
}

#[macro_export]
macro_rules! fields(
    { $($key:ident: $value:expr),+ } => {
        {
            let mut m = ::indexmap::IndexMap::new();
            $(
                m.insert(stringify!($key).to_owned(), $value);
            )+
            m
        }
     };
);

