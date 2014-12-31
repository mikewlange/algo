# algo [![Build Status](https://drone.io/github.com/peferron/algo/status.png)](https://drone.io/github.com/peferron/algo/latest)

This is just me fooling around and implementing various algorithms and data structures in Go and JavaScript. Swift might join the list if it becomes cross-platform enough.

**Warning!** The code in this repository is:

- Not optimized for performance;
- Not particularly well-tested;
- Not intended for production use.

Modules do not share code and use built-in language features only. This makes each module easiest to reason about individually.

Test all modules:

```shell
$ ./test
```

To test a specific module, for example `mergesort`:

Go:

```shell
$ go test ./mergesort/go
```

JavaScript:

```shell
$ node mergesort/javascript/*_test.js
```
