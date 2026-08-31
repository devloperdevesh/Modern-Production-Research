#!/bin/sh
set -eu

echo "[MPR-Traction]: Checking control-plane telemetry endpoint..."

if curl -fsS http://localhost:9090/api/v1/targets | grep -q "mpr-control-plane"; then
echo "[MPR-Traction]: Control plane exposing metrics successfully."
exit 0
else
echo "[MPR-Traction]: Prometheus telemetry target unreachable."
exit 1
fi
