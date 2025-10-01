#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
INTEGRATION_DIR="$PROJECT_ROOT/integration"
SURREALDB_DIR="$PROJECT_ROOT/surrealdb"

# Read current transformation version
TRANSFORM_VERSION=$(cat "$INTEGRATION_DIR/current.txt" 2>/dev/null || echo "v2.0")
TRANSFORM_SCRIPT="$INTEGRATION_DIR/transformations/$TRANSFORM_VERSION/transform.sh"

# Parse arguments
SURREALDB_VERSION="${1:-}"
FORCE="${2:-}"

usage() {
    echo "Usage: $0 <surrealdb-version> [--force]"
    echo ""
    echo "Examples:"
    echo "  $0 2.3.10          # Integrate SurrealDB v2.3.10"
    echo "  $0 2.3.10 --force  # Force re-download and transform"
    echo ""
    echo "Current transformation: $TRANSFORM_VERSION"
    exit 1
}

[[ -z "$SURREALDB_VERSION" ]] && usage

echo -e "${BLUE}🚀 SurrealX Integration Script${NC}"
echo -e "${BLUE}================================${NC}"
echo ""
echo -e "SurrealDB Version: ${GREEN}$SURREALDB_VERSION${NC}"
echo -e "Transformation:    ${GREEN}$TRANSFORM_VERSION${NC}"
echo ""

# Check if already integrated
if [[ -f "$SURREALDB_DIR/.surrealx-transformed" ]] && [[ "$FORCE" != "--force" ]]; then
    CURRENT_VERSION=$(tail -1 "$SURREALDB_DIR/.surrealx-transformed" 2>/dev/null || echo "unknown")
    if [[ "$CURRENT_VERSION" == "$SURREALDB_VERSION" ]]; then
        echo -e "${GREEN}✓${NC} Already integrated: SurrealDB v$SURREALDB_VERSION"
        echo -e "  Use --force to re-integrate"
        exit 0
    fi
fi

# Clean existing SurrealDB directory if forcing or version mismatch
if [[ -d "$SURREALDB_DIR" ]] && [[ "$FORCE" == "--force" || -f "$SURREALDB_DIR/.surrealx-transformed" ]]; then
    echo -e "${YELLOW}🗑️  Cleaning existing SurrealDB directory...${NC}"
    rm -rf "$SURREALDB_DIR"
fi

# Download SurrealDB source
echo -e "${BLUE}📥 Downloading SurrealDB v$SURREALDB_VERSION...${NC}"
DOWNLOAD_URL="https://github.com/surrealdb/surrealdb/archive/refs/tags/v$SURREALDB_VERSION.tar.gz"

if ! curl -fsSL "$DOWNLOAD_URL" | tar -xz -C "$PROJECT_ROOT"; then
    echo -e "${RED}❌ Failed to download SurrealDB v$SURREALDB_VERSION${NC}"
    exit 1
fi

# Rename extracted directory
mv "$PROJECT_ROOT/surrealdb-$SURREALDB_VERSION" "$SURREALDB_DIR"

echo -e "${GREEN}✓${NC} Downloaded successfully"
echo ""

# Verify transformation script exists
if [[ ! -f "$TRANSFORM_SCRIPT" ]]; then
    echo -e "${RED}❌ Transformation script not found: $TRANSFORM_SCRIPT${NC}"
    exit 1
fi

# Make transformation script executable
chmod +x "$TRANSFORM_SCRIPT"

# Run transformation
echo -e "${BLUE}🔧 Running transformation $TRANSFORM_VERSION...${NC}"
echo ""

if ! "$TRANSFORM_SCRIPT" "$SURREALDB_DIR" "$SURREALDB_VERSION"; then
    echo -e "${RED}❌ Transformation failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✨ Integration complete!${NC}"
echo ""
echo -e "Next steps:"
echo -e "  1. Review changes: ${BLUE}git diff surrealdb/${NC}"
echo -e "  2. Build workspace: ${BLUE}cargo build --workspace${NC}"
echo -e "  3. Run tests:       ${BLUE}cargo test --workspace${NC}"
