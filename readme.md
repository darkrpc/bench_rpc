how to test?

```
cargo run --release --package drpc-server --bin drpc-server

Starting tcp server on 0.0.0.0:10000
start bench
Total 100000,QPS: 21181 QPS/s
Total 100000,Time: 4.7212609s ,each:47212 ns/op
```


```
cargo run --release --package drpc-server --bin drpc-server

Starting tcp server on 0.0.0.0:10000
start bench
Total 100000,QPS: 20602 QPS/s
Total 100000,Time: 4.8539222s ,each:48539 ns/op
```