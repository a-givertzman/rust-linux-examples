# Results on `kanal::channel`

```log
[2025-03-25T10:39:17Z INFO  kanal_channel] main | kanal channel 
[2025-03-25T10:39:17Z INFO  kanal_channel] main | ---------------------------
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Events: 300000
[2025-03-25T10:39:17Z INFO  kanal_channel] main | ---------------------------
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Producers: 7
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Total produced: 2100000
[2025-03-25T10:39:17Z INFO  kanal_channel] main | ---------------------------
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Receivers: 5
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Total received: 10500000
[2025-03-25T10:39:17Z INFO  kanal_channel] main | ---------------------------
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Loads: 10
[2025-03-25T10:39:17Z INFO  kanal_channel] main | ---------------------------
[2025-03-25T10:39:17Z INFO  kanal_channel] main | Total elapsed: 1.4181259s
```

# Results on `tokio::channel`

```log
[2025-03-25T10:40:59Z INFO  main] main | tokio channel
[2025-03-25T10:40:59Z INFO  main] main | ---------------------------
[2025-03-25T10:40:59Z INFO  main] main | Events: 300000
[2025-03-25T10:40:59Z INFO  main] main | ---------------------------
[2025-03-25T10:40:59Z INFO  main] main | Producers: 7
[2025-03-25T10:40:59Z INFO  main] main | Total produced: 2100000
[2025-03-25T10:40:59Z INFO  main] main | ---------------------------
[2025-03-25T10:40:59Z INFO  main] main | Receivers: 5
[2025-03-25T10:40:59Z INFO  main] main | Total received: 10500000
[2025-03-25T10:40:59Z INFO  main] main | ---------------------------
[2025-03-25T10:40:59Z INFO  main] main | Loads: 10
[2025-03-25T10:40:59Z INFO  main] main | ---------------------------
[2025-03-25T10:40:59Z INFO  main] main | Total elapsed: 2.591961916s
```

# Results on `std::channel`

```log
[2025-03-25T10:38:31Z INFO  std_channel] main | std channel
[2025-03-25T10:38:31Z INFO  std_channel] main | ---------------------------
[2025-03-25T10:38:31Z INFO  std_channel] main | Events: 300000
[2025-03-25T10:38:31Z INFO  std_channel] main | ---------------------------
[2025-03-25T10:38:31Z INFO  std_channel] main | Producers: 7
[2025-03-25T10:38:31Z INFO  std_channel] main | Total produced: 2100000
[2025-03-25T10:38:31Z INFO  std_channel] main | ---------------------------
[2025-03-25T10:38:31Z INFO  std_channel] main | Receivers: 5
[2025-03-25T10:38:31Z INFO  std_channel] main | Total received: 10500000
[2025-03-25T10:38:31Z INFO  std_channel] main | ---------------------------
[2025-03-25T10:38:31Z INFO  std_channel] main | Loads: 10
[2025-03-25T10:38:31Z INFO  std_channel] main | ---------------------------
[2025-03-25T10:38:31Z INFO  std_channel] main | Total elapsed: 7.270552699s
```
