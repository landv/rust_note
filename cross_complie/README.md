# rust 交叉编译  
这个命令会列出已经安装和可以被安装的运行库。  
`rustup target list`   
```txt
golang 复习知识点：
go tool dist list
命令来查看支持的操作系统列表。
```
这个命令会列出已经安装和可以被安装的编译支持库  
`rustup component list`  

1.在Linux上面交叉编译成windows程序  
安装运行库  
`rustup target add x86_64-pc-windows-gnu`  
`rustup target add i686-pc-windows-gnu`  
安装编译支持库  
`rustup component add x86_64-w64-mingw32-gcc`  
编译运行(可以不用传递linker)  
`rustc -C linker="rust-std-x86_64-pc-windows-gnu" --target="x86_64-pc-windows-gnu" main.rs`  
`rustc --target="x86_64-pc-windows-gnu" main.rs`  
cargo 编译运行  
`cargo build --release --target=x86_64-pc-windows-gnu --verbose`  

如果出现以下情况  
```txt
error: linker `x86_64-w64-mingw32-gcc` not found
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error
```
请按照以下方法操作  
```txt
sudo apt install mingw-w64
sudo apt install clang
sudo apt install gcc
sudo apt install mingw-w64-i686-dev
```

2.在Linux上面交叉编译成android程序  
`rustup target add aarch64-linux-android`


