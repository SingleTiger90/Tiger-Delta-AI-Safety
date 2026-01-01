#!/bin/bash
# TigerΔ Performance Benchmark Tool

INTERFACE=$1
DURATION=30

if [ -z "$INTERFACE" ]; then
    echo "Usage: sudo ./benchmark.sh <interface>"
    exit 1
fi

echo "🚀 Starting TigerΔ Benchmark on $INTERFACE for ${DURATION}s..."
echo "📊 Monitoring CPU SoftIRQ load (lower is better)..."

# Запускаємо збір статистики в фоні
mpstat -P ALL 1 $DURATION > cpu_stats.txt &
MPSTAT_PID=$!

# Вимірюємо кількість пакетів через nstat
nstat -n
sleep $DURATION
echo "📈 Statistics gathered."

# Розрахунок результатів
TOTAL_PACKETS=$(nstat | grep IpInReceives | awk '{print $2}')
PPS=$(echo "$TOTAL_PACKETS / $DURATION" | bc)

echo "-------------------------------------------"
echo "TigerΔ Performance Report:"
echo "Total Packets Processed: $TOTAL_PACKETS"
echo "Average Throughput: $PPS packets/sec"
echo "Check 'cpu_stats.txt' for per-core efficiency."
echo "-------------------------------------------"

kill $MPSTAT_PID 2>/dev/null
