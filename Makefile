# Makefile for Yew WebAssembly project

# コマンド定義
TRUNK = trunk
CARGO = cargo
GIT = git

# デフォルトのターゲット（ビルド）
all: build

# 開発用ビルド
build:
	$(TRUNK) build

# リリース用ビルド
release:
	$(TRUNK) build --release

# 開発用サーバーの起動
serve:
	$(TRUNK) serve

# プロジェクトのクリーンアップ
clean:
	$(CARGO) clean
	$(TRUNK) clean
	rm -rf dist/

# リポジトリにプッシュ
push:
	$(GIT) add .
	$(GIT) commit -m "Deploy to GitHub Pages"
	$(GIT) push origin main

# デプロイをトリガーする
deploy:
	$(MAKE) release
	$(MAKE) push

# ヘルプメッセージ
help:
	@echo "Makefile for Yew WebAssembly project"
	@echo ""
	@echo "Usage:"
	@echo "  make build           - Build the project in debug mode"
	@echo "  make release         - Build the project in release mode"
	@echo "  make serve           - Serve the project with Trunk development server"
	@echo "  make clean           - Clean the project"
	@echo "  make push            - Commit and push changes to GitHub"
	@echo "  make deploy          - Build in release mode and push to GitHub"
	@echo "  make help            - Show this help message"
