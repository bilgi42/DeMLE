#!/bin/bash
echo "🚀 DEMLE Live Dashboard Starter"
echo "================================"

# Check if we're in the contracts directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: Please run this script from the contracts directory"
    echo "   cd contracts && ./start-live-dashboard.sh"
    exit 1
fi

# Generate the live dashboard
echo "📊 Setting up real-time dashboard..."
npm run realtime-demo &
DASHBOARD_PID=$!

# Wait a moment for the dashboard to generate
sleep 3

# Check if the HTML file was created
if [ -f "realtime-dashboard.html" ]; then
    echo ""
    echo "✅ Real-time dashboard ready!"
    echo ""
    echo "🌐 Open realtime-dashboard.html in your browser"
    echo ""
    echo "📍 Contract address will be shown in the dashboard"
    echo "🔗 Use this address to connect your Rust miner:"
    echo ""
    echo "   cargo run --bin demle-miner -- --contract CONTRACT_ADDRESS --rpc http://localhost:8545"
    echo ""
    echo "💡 The dashboard updates every 3 seconds and shows:"
    echo "   • Your miner when it connects"
    echo "   • Live token balance updates"
    echo "   • Mining events with animations"
    echo "   • Real-time charts and statistics"
    echo ""
    echo "🔄 Dashboard server is running (PID: $DASHBOARD_PID)"
    echo "   Press Ctrl+C to stop"
    echo ""
    
    # Keep the script running
    wait $DASHBOARD_PID
else
    echo "❌ Failed to generate dashboard"
    kill $DASHBOARD_PID 2>/dev/null
    exit 1
fi 