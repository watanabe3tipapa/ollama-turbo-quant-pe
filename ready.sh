#!/bin/bash

echo "Stopping existing servers..."
pkill -f "backend_turbo\|turbo_ollama_backend\|vite\|node.*frontend" 2>/dev/null
sleep 2

if [ "$1" = "turbo" ]; then
    echo "Starting Turbo Ollama..."
    cd "$(dirname "$0")/backend-turbo" && cargo run > /dev/null 2>&1 &
    sleep 3
    cd "$(dirname "$0")/frontend-turbo" && npm run dev > /dev/null 2>&1 &
    echo "✓ Turbo Ollama ready at http://localhost:3001"
else
    echo "Starting non-Turbo Ollama..."
    cd "$(dirname "$0")/backend" && cargo run > /dev/null 2>&1 &
    sleep 3
    cd "$(dirname "$0")/frontend" && npm run dev > /dev/null 2>&1 &
    echo "✓ non-Turbo Ollama ready at http://localhost:3000"
fi

sleep 2
echo ""
echo "Usage: ./ready.sh [turbo]"
echo "  ./ready.sh      - Start non-Turbo Ollama"
echo "  ./ready.sh turbo - Start Turbo Ollama"
