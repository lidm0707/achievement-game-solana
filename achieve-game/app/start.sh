#!/bin/bash

# Achievement Game Frontend Quick Start Script

echo "🚀 Starting Achievement Game Frontend..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm first."
    exit 1
fi

# Navigate to the app directory
cd "$(dirname "$0")"

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install --legacy-peer-deps
fi

# Check if Phantom wallet is available
echo "🔗 Make sure you have Phantom wallet installed in your browser!"
echo "📱 Install Phantom from: https://phantom.app/"

# Start the development server
echo "🌐 Starting development server on http://localhost:3001"
echo "🎮 Connect your Phantom wallet and start using the Achievement Game!"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Start the server on port 3001
PORT=3001 npm start
