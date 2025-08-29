#!/bin/bash
# MagentaDB Test Script
# Tests basic functionality and generates sample data

set -e  # Exit on any error

echo "🧪 Testing MagentaDB v0.1.0"
echo "=========================="

# Clean up any existing test database
rm -f test_magentadb.json

# Build the project
echo "📦 Building MagentaDB..."
cargo build --release

CLI="./target/release/magentadb-cli"
DB_FILE="test_magentadb.json"

# Test 1: Basic Insert and Show
echo "🔧 Test 1: Basic Operations"
$CLI --database $DB_FILE insert user1 name "Alice Johnson"
$CLI --database $DB_FILE insert user1 email "alice@example.com"
$CLI --database $DB_FILE insert user1 age "28"

# Show the document
echo "📄 Document contents:"
$CLI --database $DB_FILE show user1

echo ""

# Test 2: Multiple Documents
echo "🔧 Test 2: Multiple Documents"
$CLI --database $DB_FILE insert user2 name "Bob Smith"
$CLI --database $DB_FILE insert user2 department "Engineering"
$CLI --database $DB_FILE insert user3 name "Carol Davis"
$CLI --database $DB_FILE insert user3 department "Marketing"

# Test 3: Query Functionality
echo "🔧 Test 3: Query Operations"
echo "Searching for 'Engineering':"
$CLI --database $DB_FILE query "Engineering"

echo ""
echo "Searching for 'Alice Johnson':"
$CLI --database $DB_FILE query "Alice Johnson"

echo ""

# Test 4: List All Documents
echo "🔧 Test 4: List Documents"
$CLI --database $DB_FILE list --verbose

echo ""

# Test 5: Decryption
echo "🔧 Test 5: Decryption"
$CLI --database $DB_FILE decrypt user1 name
$CLI --database $DB_FILE decrypt user2 department

echo ""

# Test 6: Database Statistics
echo "🔧 Test 6: Database Statistics"
$CLI --database $DB_FILE stats

echo ""

# Test 7: Performance Test
echo "🔧 Test 7: Performance Test"
echo "Inserting 1000 documents..."
start_time=$(date +%s%N)

for i in {1..1000}; do
    $CLI --database $DB_FILE insert "perf_test_$i" data "test_value_$i" >/dev/null 2>&1
done

end_time=$(date +%s%N)
duration=$((($end_time - $start_time) / 1000000))  # Convert to milliseconds

echo "Inserted 1000 documents in ${duration}ms"
echo "Average: $((duration / 1000))ms per document"

# Show final stats
echo ""
echo "📊 Final Database Statistics:"
$CLI --database $DB_FILE stats

# Test 8: Search Performance
echo ""
echo "🔧 Test 8: Search Performance"
echo "Searching in database with 1003 documents:"
start_time=$(date +%s%N)
$CLI --database $DB_FILE query "test_value_500" >/dev/null 2>&1
end_time=$(date +%s%N)
search_duration=$((($end_time - $start_time) / 1000000))

echo "Search completed in ${search_duration}ms"

# Cleanup test (optional)
echo ""
echo "🧹 Cleanup (removing test database)"
$CLI --database $DB_FILE clear --force

echo ""
echo "✅ All tests completed successfully!"
echo "📁 Test database saved as: $DB_FILE"