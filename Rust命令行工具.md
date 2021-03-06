# 工具
抄一抄
https://raw.githubusercontent.com/xuesongbj/Rust-Notes/main/Rust%E5%AD%A6%E4%B9%A0%E7%AC%94%E8%AE%B0_2021/%E5%B7%A5%E5%85%B7/%E5%B7%A5%E5%85%B7.md
### rustc工具


```
$ rustc [OPTIONS] INPUT
```

* `--crate-type`：编译类型。(lib，bin)
* `--edition`：版本。(2015, 2018)
* `-g`：生成调试信息。
* `-O`：优化代码。(-C opt-level=2)
* `-o`：输出文件名。
* `-v`：详情输出。

### cargo工具

大部分情况并不直接使用rust编译器 rustc工具，而是使用cargo工具进行管理。cargo将所有工具链整合在一起。

```
$ cargo [+toolchain] [OPTIONS] [SUBCOMMAND]
```

* `build, b`: 编译。
    * `-v`: 输出`rustc`详细信息。
    * `--release`: 编译优化过的发布版本。
* `check, c`: 分析并报告错误，不生成目标文件。（比 build 快，参数基本相同）
* `clean`: 清除target目录
* `new`: 创建新包。
    * `--bin`: 使用binray模版。
    * `--lib`: 使用library模版。
    * `--name`: 包名。(默认使用目录名)
* `run, r`: 运行包。(debug)
    * `--release`: 发行版本。
* `update`: 更新依赖项。


#### 创建一个项目

```
root@8d75790f92f5:~/rs# cargo new ddd
     Created binary (application) `ddd` package

root@8d75790f92f5:~/rs# tree ddd
ddd
|-- Cargo.toml              // 配置文件
`-- src                     // 源码目录
    `-- main.rs             // 入口文件
```

#### 编译

```
root@8d75790f92f5:~/rs/ddd# cargo build                             // 第一次全量变异，速度较慢
   Compiling ddd v0.1.0 (/root/rs/ddd)
    Finished dev [unoptimized + debuginfo] target(s) in 5.20s

root@8d75790f92f5:~/rs/ddd# cargo build                             // 增量编译，使用缓存，速度较快
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

root@8d75790f92f5:~/rs/ddd# tree .
.
|-- Cargo.lock
|-- Cargo.toml
|-- src
|   `-- main.rs
`-- target
    |-- CACHEDIR.TAG
    `-- debug                                                       // 编译默认生成debug版本
        |-- build
        |-- ddd                                                     // 编译后，二进制文件
        |-- ddd.d
        |-- deps
        |   |-- ddd-1b863e9c0cbe51a4
        |   `-- ddd-1b863e9c0cbe51a4.d
        |-- examples
        `-- incremental                                             // 编译过程中，生成临时文件
            `-- ddd-7unyy5ydpys3                                    // 增量编译会使用这些临时文件信息，提升编译速度
                |-- s-fyxkxgi37u-lv9gsv-2md2shsn0hxh
                |   |-- 1faaksb01oshajgl.o
                |   |-- 1jdduolkp1j8q7k6.o
                |   |-- 2ti9ztogg3wim9e5.o
                |   |-- 2w47p718pyn72fe8.o
                |   |-- 394ciuwx6g0z5rec.o
                |   |-- 47e1z3bwj4l073fy.o
                |   |-- 4rbbrwy6sz0p5v6g.o
                |   |-- dep-graph.bin
                |   |-- query-cache.bin
                |   |-- work-products.bin
                |   `-- zzseqnri7pwhfmu.o
                `-- s-fyxkxgi37u-lv9gsv.lock

9 directories, 20 files
```

&nbsp;

编译发行版本

```
root@8d75790f92f5:~/rs/ddd# cargo build --release                   // 编译发行版本
   Compiling ddd v0.1.0 (/root/rs/ddd)
    Finished release [optimized] target(s) in 3.65s

root@8d75790f92f5:~/rs/ddd# tree .
.
|-- Cargo.lock
|-- Cargo.toml
|-- src
|   `-- main.rs
`-- target
    |-- CACHEDIR.TAG
    `-- release                                                     // 生成发布版本
        |-- build
        |-- ddd
        |-- ddd.d
        |-- deps
        |   |-- ddd-e7ca8e178054b270
        |   `-- ddd-e7ca8e178054b270.d
        |-- examples
        `-- incremental

14 directories, 24 files
```

&nbsp;

不编译，仅检查代码是否正确。
```
root@8d75790f92f5:~/rs/ddd# cargo check
    Checking ddd v0.1.0 (/root/rs/ddd)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
```

&nbsp;

清理编译数据。

```
root@8d75790f92f5:~/rs/ddd# cargo clean
root@8d75790f92f5:~/rs/ddd# tree .
.
|-- Cargo.lock
|-- Cargo.toml
`-- src
    `-- main.rs

1 directory, 3 files
```