#!/bin/bash
cd "$(dirname "$0")/backend-turbo" && cargo run > /dev/null 2>&1 &
sleep 2
cd "$(dirname "$0")/frontend-turbo" && npm run dev > /dev/null 2>&1 &
echo "Turbo Ollama starting..."
echo "Frontend: http://localhost:3001"
echo "Backend: http://localhost:8001"
