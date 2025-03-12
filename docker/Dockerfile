# Dockerfile
FROM rust:latest

# 必要なパッケージをインストール（例: build-essential 等）
RUN apt update \
    && apt install -y curl \
    && rm -rf /var/lib/apt/lists/*

# trunk のインストール
# RUN cargo install trunk
RUN cargo install --locked trunk

# wasm32-unknown-unknown ターゲットの追加
RUN rustup target add wasm32-unknown-unknown

# 作業ディレクトリの設定
WORKDIR /workspace

# VSCode 用の devcontainer 設定に合わせるため、必要に応じてユーザ設定等を追加できます

