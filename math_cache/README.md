# The table of precalculated values

Arguments and values of some function calculated and stored to be available for extremely fast read and access

  arg1  |  arg2  |   arg3  |  arg4  |  Val1  |  Val2  |  Val3  |  Val4  |
   ---: |   ---: |    ---: |   ---: |  ---:  |  ---:  |  ---:  |  ---:  |
   1.00 |   0.00 |    0.00 |   0.00 |   7.00 |   0.00 |   0.00 |   0.00 |
   0.00 |   1.00 |    0.00 |   0.00 |   0.00 |   8.00 |   0.00 |   0.00 |
   0.00 |   0.00 |    1.00 |   0.00 |   0.00 |   0.00 |   9.00 |   0.00 |
   0.00 |   0.00 |    0.00 |   1.00 |   0.00 |   0.00 |   0.00 |  10.00 |

```rust
// reading cached data from the `path`
let cache = Cache::load(path);

let args = json!({
    "arg1": 1.0,
    "arg2": 0.0,
    "arg3": 0.0,
    "arg4": 0.0,
})
let values = cache.get(args);

println!(values);
// result:
// {
//     "val1": 7.0,
//     "val2": 0.0,
//     "val3": 0.0,
//     "val4": 0.0,
// } 

```
