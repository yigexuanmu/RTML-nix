# RTML-nix

Nix flake 打包 [RTML](https://github.com/MEKCCK/RTML) — 一个用 Rust 编写的 TUI Minecraft 启动器。

源码从上游 `MEKCCK/RTML` 拉取，本仓库只包含 nix 打包文件。

## 使用

### 安装

```bash
nix profile install github:yigexuanmu/RTML-nix
```

### 临时运行

```bash
nix run github:yigexuanmu/RTML-nix
```

### 内置 JDK

打包参考 PrismLauncher 方案，内置 4 个 JDK 版本，通过 `RTML_JAVA_PATHS` 环境变量暴露：

| 版本 | 路径 |
|------|------|
| JDK 25 | `openjdk-25.0.4+1` |
| JDK 21 | `openjdk-21.0.12+2` |
| JDK 17 | `openjdk-17.0.20+2` |
| JDK 8 | `openjdk-8u502-b01` |

构建时默认使用 JDK 17（满足 build.rs 编译 Java shim 的需求）。

### 开发环境

```bash
nix develop
```

提供 Rust stable 工具链、pkg-config、JDK 17。

## 更新

上游更新后，刷新 flake.lock 即可：

```bash
nix flake update rtml-src
```

版本号从上游 `Cargo.toml` 动态读取，无需手动修改。

## 许可

RTML 本体基于 GPL-3.0-or-later 协议，详见 [上游仓库](https://github.com/MEKCCK/RTML)。
