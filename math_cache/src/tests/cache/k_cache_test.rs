#[cfg(test)]

mod cache_sofia {
    use bincode::{Decode, Encode};
    use indexmap::IndexMap;
    use sal_core::dbg::Dbg;
    use std::{collections::HashMap, fs::OpenOptions, io::{Read, Write}, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{cache::Cache, fields};
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
        let dbg = Dbg::own("cache-sofia-get");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let data = vec![
            vec![0.0, 0.0, 1.0],
            //   0.2  0.7   ?
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 2.0],
            vec![1.0, 1.0, 3.0],
        ];
        let keys = vec![Some(0.2), Some(0.7), None];
        // let data = vec![
        //     //   f1   f2   f3
        //     vec![0.0, 0.0, 0.0],
        //     vec![0.1, 0.2, 0.4],
        //     vec![0.2, 0.4, 0.8],
        //     vec![0.3, 0.6, 1.2],
        //     // vec![0.4, 0.8, 1.6],
        //     // vec![0.5, 1.0, 2.0],
        //     // vec![0.6, 1.2, 2.4],
        //     // vec![0.7, 1.4, 2.8],
        // ];
        // let keys = vec![Some(0.22), Some(0.42), None];

        let keys_and_delta: Vec<_> = keys.iter().enumerate()
        .map(|(i, key)| {
            let Some(key) = *key else {
                return None;
            };
            let mut data: Vec<_> = data.iter().map(|v| v[i]).collect();
            data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            data.dedup();        
    //      println!("{i} {:?}", data);
            if data.len() == 1 {
                assert!(key == data[0]);
                Some((key, 1.)) // ключ всегда будет равен значению, дельта не важна
            } else {
                assert!(data.len() == 2);
                assert!(data[0] < key && key < data[1]);
                Some((key, data[1] - data[0]))
            }
        })
        .collect();

        let result = data.iter().map(|v| {
            let multipler 
                = keys_and_delta.iter()
                .zip(v.iter()).filter(|(k, v)| k.is_some())
                .fold(1. ,|acc, (k, v)| {
                    let (key, delta) = k.unwrap();
                    acc*(1. - ((key - v) as f64).abs()/delta)
                });
            v.iter().map(|v| v*multipler).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let result = (0..keys.len()).map(|i| {
            result.iter().map(|v| v[i]).sum::<f64>()
        }).collect::<Vec<_>>();

        println!("{:?}", &result);
        // for (step, args, target ) in test_data {
        //     let time = Instant::now();
        //     let result: Vec<IndexMap<String, f64>> = cache.get(&args);
        //     let target: Vec<IndexMap<String, f64>> = target.into_iter().map(|row| {
        //         row.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
        //     }).collect();
        //     let elapsed = time.elapsed();
        //     log::debug!("step {step}   elapsed: {:?} \nresult: {:?}\ntarget: {:?}", time.elapsed(), result, target);
        //     assert!(result == target, "step {step}   elapsed: {:?} \nresult: {:?}\ntarget: {:?}", elapsed, result, target);
        // }
        test_duration.exit();
    }
}