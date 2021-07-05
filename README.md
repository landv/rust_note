# rust_note
rust study notes;rust学习笔记  
# 官方网站
https://www.rust-lang.org/zh-CN/  
Rust 一门赋予每个人 构建可靠且高效软件能力的语言  
## 为什么选择 Rust?
*高性能*
Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。  
*可靠性*
Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。  
*生产力*
Rust 拥有出色的文档、友好的编译器和清晰的错误提示信息， 还集成了一流的工具——包管理器和构建工具， 智能地自动补全和类型检验的多编辑器支持， 以及自动格式化代码等等。  
# 安装Rust
使用 Rustup（推荐）
https://www.rust-lang.org/zh-CN/tools/install
linux/mac
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
windows
https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

*用 rustup 管理工具链*
Rust 由工具 rustup 安装和管理。Rust 有着以 6 星期为周期的 快速版本迭代机制，支持 大量平台，因而不同时期存在大量不同的 Rust 构建版本。  
rustup 用于管理不同平台下的 Rust 构建版本并使其互相兼容， 支持安装由 Beta 和 Nightly 频道发布的版本，并支持其他用于交叉编译的编译版本。  
如果您曾经安装过 rustup，可以执行 rustup update 来升级 Rust。  
*配置 PATH 环境变量*
在 Rust 开发环境中，所有工具都安装在 ~/.cargo/bin 目录中，您可以在这里找到包括 rustc、cargo 和 rustup 在内的 Rust 工具链。  
Rust 开发者通常会将该目录加入 PATH环境变量中。在安装过程中，rustup 会尝试配置 PATH。 由于不同平台、命令行 Shell 之间存在差异，  
rustup 中也可能存在 Bug，因此在终端重启或用户重新登录之前，rustup 对 PATH 的修改可能不会生效，甚至完全无效。  
如果安装后在终端尝试执行 rustc --version 失败，那么，以上内容就是最可能的原因。  
*卸载 Rust*
在任何时候如果您想卸载 Rust，您可以运行 rustup self uninstall。但我们会想念您的！  
# Cargo：Rust 的构建工具和包管理器
您在安装 Rustup 时，也会安装 Rust 构建工具和包管理器的最新稳定版，即 Cargo。Cargo 可以做很多事情：  
cargo build 可以构建项目  
cargo run 可以运行项目  
cargo test 可以测试项目  
cargo doc 可以为项目构建文档  
cargo publish 可以将库发布到 crates.io。  
要检查您是否安装了 Rust 和 Cargo，可以在终端中运行：  
cargo --version  
# 入门图书
https://www.rust-lang.org/zh-CN/learn  
https://doc.rust-lang.org/book/  
https://kaisery.github.io/trpl-zh-cn/  
Rust 程序设计语言（第二版 & 2018 edition） 简体中文版 https://github.com/KaiserY/trpl-zh-cn  
# 社区资源（非本作者创建社区）
Rust语言中文社区：https://rust.cc/  
Rust 中文 Wiki：https://wiki.rust-china.org/  
