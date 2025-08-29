#!/bin/bash
# Comprehensive MagentaDB Test Suite
# Tests all functionality before release

set -e

echo "🧪 MagentaDB Comprehensive Test Suite"
echo "====================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test database file
DB_FILE="comprehensive_test.json"

# Clean up function
cleanup() {
    rm -f "$DB_FILE"
}

# Trap to ensure cleanup happens
trap cleanup EXIT

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"
    
    ((TESTS_RUN++))
    echo -n "Testing: $test_name... "
    
    if output=$(eval "$test_command" 2>&1); then
        if [[ -z "$expected_pattern" ]] || echo "$output" | grep -q "$expected_pattern"; then
            echo -e "${GREEN}PASS${NC}"
            ((TESTS_PASSED++))
            return 0
        else
            echo -e "${RED}FAIL${NC} - Expected pattern '$expected_pattern' not found"
            echo "Output: $output"
            ((TESTS_FAILED++))
            return 1
        fi
    else
        echo -e "${RED}FAIL${NC} - Command failed"
        echo "Error: $output"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Build the project first
echo "🔨 Building MagentaDB..."
if ! cargo build --release; then
    echo "❌ Build failed"
    exit 1
fi

CLI="./target/release/magentadb-cli"
DB_ARGS="--database $DB_FILE"

echo ""
echo "🧪 Running tests with CLI: $CLI"
echo ""

# Test 1: Help and version
run_test "CLI help" "$CLI --help" "A searchable encrypted database"
run_test "CLI version" "$CLI --version" "0.1.0"

# Test 2: Basic insert operations
run_test "Insert single field" "$CLI $DB_ARGS insert user1 name 'John Doe'" "Inserted document 'user1'"
run_test "Insert additional field" "$CLI $DB_ARGS insert user1 email 'john@example.com'" "Inserted document 'user1'"
run_test "Insert new document" "$CLI $DB_ARGS insert user2 name 'Jane Smith'" "Inserted document 'user2'"

# Test 3: Show operations
run_test "Show existing document" "$CLI $DB_ARGS show user1" "Document: user1"
run_test "Show non-existent document" "$CLI $DB_ARGS show nonexistent" "not found"

# Test 4: List operations
run_test "List documents" "$CLI $DB_ARGS list" "Database contains 2 document"
run_test "List verbose" "$CLI $DB_ARGS list -v" "user1"

# Test 5: Query operations
run_test "Query existing value" "$CLI $DB_ARGS query 'John Doe'" "Found 1 document"
run_test "Query non-existent value" "$CLI $DB_ARGS query 'NonExistent'" "No documents found"

# Test 6: Decrypt operations
run_test "Decrypt existing field" "$CLI $DB_ARGS decrypt user1 name" "Decrypted user1.name: John Doe"
run_test "Decrypt non-existent field" "$CLI $DB_ARGS decrypt user1 nonexistent" "not found"

# Test 7: Stats operation
run_test "Database statistics" "$CLI $DB_ARGS stats" "Database Statistics"

# Test 8: Complex operations
run_test "Insert with special characters" "$CLI $DB_ARGS insert special data 'Hello @#$%^&*() World!'" "Inserted document 'special'"
run_test "Query special characters" "$CLI $DB_ARGS query 'Hello @#$%^&*() World!'" "Found 1 document"

# Test 9: Unicode support
run_test "Insert Unicode data" "$CLI $DB_ARGS insert unicode name '日本語 中文 العربية'" "Inserted document 'unicode'"
run_test "Query Unicode data" "$CLI $DB_ARGS query '日本語 中文 العربية'" "Found 1 document"

# Test 10: Large data
LARGE_DATA=$(head -c 1000 /dev/zero | tr '\0' 'A')
run_test "Insert large data" "$CLI $DB_ARGS insert large data '$LARGE_DATA'" "Inserted document 'large'"

# Test 11: Many documents performance test
echo ""
echo "📊 Performance Testing..."
start_time=$(date +%s)
for i in {1..100}; do
    $CLI $DB_ARGS insert "perf_$i" data "value_$i" >/dev/null 2>&1
done
end_time=$(date +%s)
duration=$((end_time - start_time))
echo "Inserted 100 documents in ${duration}s ($(echo "scale=2; 100/$duration" | bc -l) ops/sec)"

# Test 12: Search performance
start_time=$(date +%s%N)
$CLI $DB_ARGS query "value_50" >/dev/null 2>&1
end_time=$(date +%s%N)
search_time=$(((end_time - start_time) / 1000000))
echo "Search in 103 documents took ${search_time}ms"

# Test 13: Database stats after bulk insert
run_test "Stats after bulk insert" "$CLI $DB_ARGS stats" "Documents: 103"

# Test 14: Remove operations
run_test "Remove existing document" "$CLI $DB_ARGS remove user1" "Removed document 'user1'"
run_test "Remove non-existent document" "$CLI $DB_ARGS remove nonexistent" "not found"
run_test "Verify removal" "$CLI $DB_ARGS show user1" "not found"

# Test 15: Clear database with confirmation
echo "y" | $CLI $DB_ARGS clear >/dev/null 2>&1
run_test "Database cleared" "$CLI $DB_ARGS list" "No documents in database"

# Test 16: Force clear (test empty database)
run_test "Force clear empty database" "$CLI $DB_ARGS clear --force" "0 documents removed"

# Test 17: File persistence
run_test "Insert after clear" "$CLI $DB_ARGS insert persistent data 'test'" "Inserted document 'persistent'"
# Check if file exists
if [[ -f "$DB_FILE" ]]; then
    echo "Testing: File persistence... ${GREEN}PASS${NC}"
    ((TESTS_PASSED++))
else
    echo "Testing: File persistence... ${RED}FAIL${NC}"
    ((TESTS_FAILED++))
fi
((TESTS_RUN++))

# Test 18: Database loading
# Create a new CLI instance to test loading
run_test "Load existing database" "$CLI $DB_ARGS show persistent" "Document: persistent"

# Test 19: Error handling
run_test "Invalid command" "$CLI invalid_command" "error\|Error\|USAGE" || true  # This should fail, so we ignore the return code
run_test "Missing arguments" "$CLI $DB_ARGS insert" "error\|Error\|required" || true

# Test 20: Edge cases
run_test "Empty field name" "$CLI $DB_ARGS insert test '' 'value'" "Inserted document 'test'" || true
run_test "Empty value" "$CLI $DB_ARGS insert test field ''" "Inserted document 'test'"

# Final results
echo ""
echo "📋 Test Summary"
echo "==============="
echo "Tests run: $TESTS_RUN"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
if [[ $TESTS_FAILED -gt 0 ]]; then
    echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"
    exit 1
else
    echo -e "Tests failed: ${GREEN}0${NC}"
fi

echo ""
success_rate=$(echo "scale=1; $TESTS_PASSED * 100 / $TESTS_RUN" | bc -l)
echo -e "Success rate: ${GREEN}${success_rate}%${NC}"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo ""
    echo "🎉 All tests passed! MagentaDB is ready for release."
    
    # Show final database stats
    echo ""
    echo "📊 Final Database State:"
    $CLI $DB_ARGS stats 2>/dev/null || true
    
    echo ""
    echo "✅ MagentaDB v0.1.0-beta is production ready!"
else
    echo ""
    echo "❌ Some tests failed. Please fix issues before release."
    exit 1
fi