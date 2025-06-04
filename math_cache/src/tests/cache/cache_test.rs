#[cfg(test)]

mod cache {
    use bincode::{Decode, Encode};
    use indexmap::IndexMap;
    use sal_core::dbg::Dbg;
    use std::{collections::HashMap, fs::OpenOptions, io::{Read, Write}, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{cache::Cache, fields, tests::approx_eq::AproxEq};
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
    /// Testing `get` method
    #[test]
    fn get() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("cache-get");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let fields = fields!{
            //            0     1     2     3     4     5     6     7
            field1: vec![0.0,  0.1,  0.2,  0.3,  0.4,  0.5,  0.6,  0.7],
            field2: vec![0.0,  0.2,  0.4,  0.6,  0.8,  1.0,  1.2,  1.4],
            field3: vec![0.0,  0.4,  0.8,  1.2,  1.6,  2.0,  2.4,  2.8],
            field4: vec![0.0,  0.8,  1.6,  2.4,  3.2,  4.0,  4.8,  5.6],
            field5: vec![0.0,  1.6,  3.2,  4.8,  6.4,  8.0,  9.6, 11.2],
            field6: vec![0.0,  3.2,  6.4,  9.6, 12.8, 16.0, 19.2, 22.4],
            field7: vec![0.0,  6.4, 12.8, 19.2, 25.6, 32.0, 38.4, 44.8],
            field8: vec![0.0, 12.8, 25.6, 38.4, 51.2, 64.0, 76.8, 89.6]
        };
        // let fields = fields!{
        //     //            0     1     2     3     4     5     6     7
        //     field1: vec![0.0,  1.0,  2.0,  1.0],
        //     field2: vec![0.0,  1.0,  0.0,  1.0],
        //     field3: vec![1.0,  2.0,  2.0,  3.0]
        // };
        let exclude = vec![4, 7, 12];
        let test_data = [
            // (01, 
            //     vec![
            //         ("field1", 0.2),
            //         ("field2", 0.4),
            //         ("field3", 0.8),
            //     ],
            //     vec![
            //         vec![
            //             ("field1",  0.2),
            //             ("field2",  0.4),
            //             ("field3",  0.8),
            //             ("field4",  1.6),
            //             ("field5",  3.2),
            //             ("field6",  6.4),
            //             ("field7", 12.8),
            //             ("field8", 25.6),
            //         ],
            //     ],
            // ),
            // (02, 
            //     vec![
            //         ("field3", 1.2),
            //     ],
            //     vec![
            //         vec![
            //             ("field1",  0.3),
            //             ("field2",  0.6),
            //             ("field3",  1.2),
            //             ("field4",  2.4),
            //             ("field5",  4.8),
            //             ("field6",  9.6),
            //             ("field7", 19.2),
            //             ("field8", 38.4),
            //         ],
            //     ],
            // ),
            // (03,
            //     vec![
            //         ("field1", 0.15),
            //     ],
            //     vec![
            //         vec![
            //             ("field1",  0.15),
            //             ("field2",  0.3),
            //             ("field3",  0.6),
            //             ("field4",  1.2),
            //             ("field5",  2.4),
            //             ("field6",  4.8),
            //             ("field7",  9.6),
            //             ("field8", 19.2),
            //         ],
            //     ],
            // ),
            (04,
                vec![
                    ("field4", 1.62),
                ],
                vec![
                    vec![
                        ("field1",   0.2025),
                        ("field2",   0.4050),
                        ("field3",   0.8100),
                        ("field4",   1.6200),
                        ("field5",   3.2400),
                        ("field6",   6.4800),
                        ("field7",  12.9600),
                        ("field8",  25.9200),
                    ],
                ],
            ),
            (05,
                vec![
                    ("field1", 0.3),
                    ("field2", 0.7),
                ],
                vec![
                    vec![
                        ("field1",   0.3000),
                        ("field2",   0.7000),
                        ("field3",   1.3000),
                        ("field4",   2.6000),
                        ("field5",   5.2000),
                        ("field6",  10.4000),
                        ("field7",  20.8000),
                        ("field8",  41.6000),
                    ],
                ],
            ),
            (05,
                vec![
                    ("field1", 0.39),
                    ("field2", 0.7),
                ],
                vec![
                    vec![
                        ("field1",   0.3900),
                        ("field2",   0.7000),
                        ("field3",   1.4800),
                        ("field4",   2.9600),
                        ("field5",   5.9200),
                        ("field6",  11.8400),
                        ("field7",  23.6800),
                        ("field8",  47.3600),
                    ],
                ],
            ),
        ];
        let cache = Cache::new(&dbg, fields.clone(), exclude);
        for (step, args, target ) in test_data {
            let time = Instant::now();
            let results: Vec<IndexMap<String, f64>> = cache.get(&args);
            let targets: Vec<IndexMap<String, f64>> = target.into_iter().map(|row| {
                row.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
            }).collect();
            let elapsed = time.elapsed();
            log::debug!("step {step}   elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), results, targets);

            assert!(results.len() == targets.len(), "step {step}   elapsed: {:?} \nresult: {:?}\ntarget: {:?}", elapsed, results.len(), targets.len());
            for (i, target) in targets.iter().enumerate() {
                let result = results[i].clone();
                for (tk, target) in target {
                    let result = result[tk];
                    assert!(result.aprox_eq(*target, 3), "step {step}   elapsed: {:?} \nresult: {:?}\ntarget: {:?}", elapsed, results, targets);
                }
            }
        }
        test_duration.exit();
    }
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
        let path = "src/tests/cache_store.dat";
        let fields = fields!{
            field1: vec![0.0,  0.1,  0.2,  0.3,  0.4,  0.5,  0.6,  0.7],
            field2: vec![0.0,  0.2,  0.4,  0.6,  0.8,  1.0,  1.2,  1.4],
            field3: vec![0.0,  0.4,  0.8,  1.2,  1.6,  2.0,  2.4,  2.8],
            field4: vec![0.0,  0.8,  1.6,  2.4,  3.2,  4.0,  4.8,  5.6],
            field5: vec![0.0,  1.6,  3.2,  4.8,  6.4,  8.0,  9.6, 11.2],
            field6: vec![0.0,  3.2,  6.4,  9.6, 12.8, 16.0, 19.2, 22.4],
            field7: vec![0.0,  6.4, 12.8, 19.2, 25.6, 32.0, 38.4, 44.8],
            field8: vec![0.0, 12.8, 25.6, 38.4, 51.2, 64.0, 76.8, 89.6]
        };
        for _ in 0..3 {
            let time = Instant::now();
            let cache = Cache::new(&dbg, fields.clone(), vec![4, 7, 12]);
            cache.store(path).unwrap();
            let elapsed = time.elapsed();
            let mut file = OpenOptions::new().read(true).open(path).unwrap();
            let mut buf = vec![];
            file.read_to_end(&mut buf).unwrap();
            let (cache, _): (_Cache<f64>, _) = bincode::decode_from_slice(&buf, bincode::config::standard()).unwrap();
            let target = fields.clone();
            let result: IndexMap<String, Vec<f64>> = cache.fields.iter().map(|(k, f)| (k.to_owned(), f.values.to_owned())).collect();
            log::debug!("elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), result, target);
            assert!(result == target, "elapsed: {:?} \nresult: {:?}\ntarget: {:?}", elapsed, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing `load` method
    #[test]
    fn load() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("cache-store");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let path = "src/tests/cache_load.dat";
        let fields = fields!{
            field1: vec![0.0,  0.1,  0.2,  0.3,  0.4,  0.5,  0.6,  0.7],
            field2: vec![0.0,  0.2,  0.4,  0.6,  0.8,  1.0,  1.2,  1.4],
            field3: vec![0.0,  0.4,  0.8,  1.2,  1.6,  2.0,  2.4,  2.8],
            field4: vec![0.0,  0.8,  1.6,  2.4,  3.2,  4.0,  4.8,  5.6],
            field5: vec![0.0,  1.6,  3.2,  4.8,  6.4,  8.0,  9.6, 11.2],
            field6: vec![0.0,  3.2,  6.4,  9.6, 12.8, 16.0, 19.2, 22.4],
            field7: vec![0.0,  6.4, 12.8, 19.2, 25.6, 32.0, 38.4, 44.8],
            field8: vec![0.0, 12.8, 25.6, 38.4, 51.2, 64.0, 76.8, 89.6]
        };
        let exclude = vec![4, 7, 12];
        for _ in 0..3 {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();
            let buf = bincode::encode_to_vec(
                _Cache {
                    fields: fields.iter().map(|(k, f)| (k.to_owned(), _Field { values: f.to_owned() })).collect(),
                    exclude: exclude.clone(),
                },
                bincode::config::standard(),
            ).unwrap();
            file.write_all(&buf).unwrap();

            let time = Instant::now();
            let cache: Cache<f64> = Cache::load(&dbg, path).unwrap();
            let elapsed = time.elapsed();
            let target = fields.clone();
            let result: IndexMap<String, Vec<f64>> = cache.fields().iter().map(|(k, f)| (k.to_owned(), f.values())).collect();
            log::debug!("elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), result, target);
            assert!(result == target, "elapsed: {:?} \nresult: {:?}\ntarget: {:?}", elapsed, result, target);
        }
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