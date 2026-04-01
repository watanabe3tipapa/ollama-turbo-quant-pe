#!/bin/bash
cd "$(dirname "$0")/backend" && cargo run > /dev/null 2>&1 &
sleep 2
cd "$(dirname "$0")/frontend" && npm run dev > /dev/null 2>&1 &
echo "non-Turbo Ollama starting..."
echo "Frontend: http://localhost:3000"
echo "Backend: http://localhost:8000"
