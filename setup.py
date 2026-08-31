from pathlib import Path
from setuptools import find_packages, setup

README = Path("README.md").read_text(encoding="utf-8")

setup(
name="mpr-core-labs",
version="1.0.0",
author="Devesh Chauhan",
author_email="[deveshchauhan00001@gmail.com](mailto:deveshchauhan00001@gmail.com)",
description="Enterprise-grade distributed resilience orchestration runtime for AI agent workflows.",
long_description=README,
long_description_content_type="text/markdown",
url="https://github.com/devloperdevesh/Modern-Production-Research",
packages=find_packages(include=["core", "core.*", "telemetry", "telemetry.*"]),
python_requires=">=3.11",
install_requires=[
"fastapi==0.110.0",
"uvicorn[standard]==0.28.0",
"redis==5.0.3",
"asyncpg==0.29.0",
"prometheus_client==0.20.0",
],
)
