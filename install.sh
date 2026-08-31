#!/bin/sh
set -eu

echo "⚡ [MPR Installer]: Initializing bare-metal platform validation loop..."

OS_TYPE=$(uname -s)

if [ "$OS_TYPE" != "Linux" ] && [ "$OS_TYPE" != "Darwin" ]; then
echo "❌ [MPR Error]: Sub-kernel dependencies require Linux/Unix environments. Aborting install."
exit 1
fi

python3 -m venv venv
. venv/bin/activate

python -m pip install --upgrade pip
python -m pip install -r requirements.txt

if command -v cargo >/dev/null 2>&1; then
cargo build --release
echo "✅ [MPR Installer]: Rust binary artifacts compiled successfully."
else
echo "⚠️ [MPR Warning]: Cargo toolchain not detected. Rust compilation deferred."
fi

if [ -d ".githooks" ]; then
git config core.hooksPath .githooks >/dev/null 2>&1 || true
chmod +x .githooks/* >/dev/null 2>&1 || true
fi

echo "🚀 [MPR Success]: MPR Core Labs successfully deployed."

