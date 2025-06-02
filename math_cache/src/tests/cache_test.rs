#[cfg(test)]

mod cache {
    use bincode::{Decode, Encode};
    use indexmap::IndexMap;
    use log::{warn, info, debug};
    use sal_core::dbg::Dbg;
    use std::{collections::HashMap, fs::OpenOptions, io::Read, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{cache::Cache, field::Field, fields};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing `store` method
    #[test]
    fn store() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("cache-store");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let path = "src/tests/cache.dat";
        let fields = fields!{
            field1: vec![0.0, 0.1,  0.2,  0.3,  0.4,  0.5,  0.6,  0.7],
            field2: vec![0.0, 0.2,  0.4,  0.6,  0.8,  1.0,  1.2,  1.4],
            field3: vec![0.0, 0.4,  0.8,  1.2,  1.6,  2.0,  2.4,  2.8],
            field4: vec![0.0, 0.8,  1.6,  2.4,  3.2,  4.0,  4.8,  5.6],
            field5: vec![0.0, 1.6,  3.2,  4.8,  6.4,  8.0,  9.6, 11.2],
            field6: vec![0.0, 3.2,  6.4,  9.6, 12.8, 16.0, 19.2, 22.4],
            field7: vec![0.0, 6.4, 12.8, 19.2, 25.6, 32.0, 38.4, 44.8]
        };
        let cache = Cache::new(&dbg, fields.clone(), vec![4, 7, 12]);
        cache.store(path).unwrap();
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mut buf = vec![];
        let time = Instant::now();
        file.read_to_end(&mut buf).unwrap();
        let (cache, _): (_Cache<f64>, _) = bincode::decode_from_slice(&buf, bincode::config::standard()).unwrap();
        let target = fields;
        let result: IndexMap<String, Vec<f64>> = cache.fields.iter().map(|(k, f)| (k.to_owned(), f.values.to_owned())).collect();
        log::debug!("elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), result, target);
        assert!(result == target, "elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), result, target);
        test_duration.exit();
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
}