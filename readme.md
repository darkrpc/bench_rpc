how to test?

* run `rustup update` command update Rust compiler

* run drpc bench
```
cargo run --release --package drpc-server --bin drpc-server
```
* this is drpc log
```log
Starting tcp server on 0.0.0.0:10000
start bench
Total 100000,QPS: 21181 QPS/s
Total 100000,Time: 4.7212609s ,each:47212 ns/op
```

* run tarpc bench
```
cargo run --release --package tarpc-server --bin tarpc-server
```
* this is tarpc log
```log
Starting tcp server on 0.0.0.0:10000
start bench
Total 100000,QPS: 9787 QPS/s
Total 100000,Time: 10.2177012s ,each:102177 ns/op
```