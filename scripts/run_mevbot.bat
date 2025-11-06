@echo off
REM This script starts the packaged MEV bot components on Windows.
REM Ensure config.yaml and wallet.json are present in the same folder.

start /B core.exe
start /B dashboard.exe
REM Python ML predictor can run in background (requires python on PATH)
start /B python.exe ..\analytics\ml_predictor.py
echo MEV Bot components started.
