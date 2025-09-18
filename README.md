# MyRustLeetCode

使用 Rust 语言练习 LeetCode 算法题的项目。

## 项目结构

```
src/
├── lib.rs          # 库入口文件
├── two_sum/        # Two Sum 问题
│   ├── mod.rs      # 模块声明
│   └── two_sum.rs  # 具体实现
└── ...             # 其他算法题目录
```

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test two_sum

# 显示测试输出
cargo test -- --nocapture
```

## 性能测试

```bash
# 运行基准测试
cargo bench
```

## 添加新题目

1. 在 `src/` 下创建新的题目目录
2. 创建 `mod.rs` 和具体实现文件
3. 在 `src/lib.rs` 中添加模块声明
4. 编写测试用例
5. 可选：在 `benches/` 中添加性能测试

## CI/CD

项目使用 GitHub Actions 进行持续集成，会自动：
- 构建项目
- 运行测试
- 检查代码格式（不阻止构建）
- 运行 Clippy 静态分析（不阻止构建）
