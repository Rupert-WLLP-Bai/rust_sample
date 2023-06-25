# Rust Sample Proj

这个项目是一个 Rust 项目的示例结构，展示了一种常见的项目组织方式。它包含了多个二进制可执行文件、库文件、示例代码、基准测试和集成测试。

## 目录结构

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

## 文件说明

- `Cargo.toml`：项目的依赖配置文件。
- `src/lib.rs`：项目的库文件。
- `src/main.rs`：项目的主可执行文件入口。
- `src/bin/named-executable.rs`：命名的可执行文件。
- `src/bin/another-executable.rs`：另一个可执行文件。
- `src/bin/multi-file-executable/main.rs`：包含多个文件的可执行文件入口。
- `src/bin/multi-file-executable/some_module.rs`：多文件可执行文件的模块文件。
- `benches/large-input.rs`：基准测试文件。
- `benches/multi-file-bench/main.rs`：包含多个文件的基准测试入口。
- `benches/multi-file-bench/bench_module.rs`：多文件基准测试的模块文件。
- `examples/simple.rs`：简单示例代码文件。
- `examples/multi-file-example/main.rs`：包含多个文件的示例代码入口。
- `examples/multi-file-example/ex_module.rs`：多文件示例代码的模块文件。
- `tests/some-integration-tests.rs`：集成测试文件。
- `tests/multi-file-test/main.rs`：包含多个文件的集成测试入口。
- `tests/multi-file-test/test_module.rs`：多文件集成测试的模块文件。

## 使用方法

1. 确保已安装 Rust 工具链。如果没有安装，请参考 [Rust 官方网站](https://www.rust-lang.org/) 安装指南。
2. 克隆项目到本地：
   ```
   git clone https://github.com/Rupert-WLLP-Bai/rust-sample-proj.git
   ```
3. 进入项目目录：
   ```
   cd rust-sample-proj
   ```
4. 运行可执行文件：
   ```
   cargo run --bin named-executable
   cargo run --bin another-executable
   cargo run --bin multi-file-executable
   ```
5. 运行基准测试：
   ```
   cargo bench
   ```
6. 运行集成测试：
   ```
   cargo test
   ```
