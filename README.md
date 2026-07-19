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

## 许可

RTML 本体基于 GPL-3.0-or-later 协议，详见 [上游仓库](https://github.com/MEKCCK/RTML)。
