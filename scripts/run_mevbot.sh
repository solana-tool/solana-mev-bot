#!/usr/bin/env bash
# Start MEV Bot components on Unix-like systems.
./core &
./dashboard &
python3 ../analytics/ml_predictor.py &
echo "MEV Bot components started"
