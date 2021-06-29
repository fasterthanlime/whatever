# whatever

## How to build

(This has been tested on Linux & macOS)

Build the Rust code:

```shell
$ cargo build
```

The library is in `./target/debug/libwhatever.a`

Build the Go code:

```shell
$ cd go-code/
$ go clean
$ go build -v -x .
```

Run the Go code:

```shell
$ ./whatever
```

It should print something like:

```
2021/06/29 19:01:51 Hello from Go
Hello from Rust or whatever. Your string is: The moonlight 🦜
2021/06/29 19:01:51 Rust gave us this string: Yeah allocations, sure
2021/06/29 19:01:51 Rust took back this string: P/, sure
```
