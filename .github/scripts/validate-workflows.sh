#!/bin/bash

# Script to validate GitHub Actions workflow files
# This script checks for common issues in workflow files

set -e

echo "🔍 Validating GitHub Actions workflows..."

WORKFLOW_DIR=".github/workflows"
ACTIONS_DIR=".github/actions"

# Check if workflow directory exists
if [ ! -d "$WORKFLOW_DIR" ]; then
    echo "❌ Workflow directory not found: $WORKFLOW_DIR"
    exit 1
fi

# Function to validate YAML syntax
validate_yaml() {
    local file="$1"
    echo "  Checking YAML syntax: $file"
    
    # Check if file exists
    if [ ! -f "$file" ]; then
        echo "    ❌ File not found: $file"
        return 1
    fi
    
    # Validate YAML syntax (requires yq or python)
    if command -v yq >/dev/null 2>&1; then
        if ! yq eval '.' "$file" >/dev/null 2>&1; then
            echo "    ❌ Invalid YAML syntax in $file"
            return 1
        fi
    elif command -v python3 >/dev/null 2>&1; then
        if ! python3 -c "import yaml; yaml.safe_load(open('$file'))" >/dev/null 2>&1; then
            echo "    ❌ Invalid YAML syntax in $file"
            return 1
        fi
    else
        echo "    ⚠️  Cannot validate YAML syntax (yq or python3 not found)"
    fi
    
    echo "    ✅ YAML syntax is valid"
    return 0
}

# Function to check for common workflow issues
check_workflow_issues() {
    local file="$1"
    echo "  Checking workflow issues: $file"
    
    # Check for required fields
    if ! grep -q "^name:" "$file"; then
        echo "    ❌ Missing 'name' field"
        return 1
    fi
    
    if ! grep -q "^on:" "$file"; then
        echo "    ❌ Missing 'on' field"
        return 1
    fi
    
    if ! grep -q "^jobs:" "$file"; then
        echo "    ❌ Missing 'jobs' field"
        return 1
    fi
    
    # Check for checkout action version
    if grep -q "actions/checkout@v[123]" "$file"; then
        echo "    ⚠️  Consider updating checkout action to v4"
    fi
    
    # Check for setup-node version
    if grep -q "actions/setup-node@v[123]" "$file"; then
        echo "    ⚠️  Consider updating setup-node action to v4"
    fi
    
    echo "    ✅ No major workflow issues found"
    return 0
}

# Function to validate action files
validate_action() {
    local action_dir="$1"
    local action_file="$action_dir/action.yml"
    
    echo "  Checking action: $action_dir"
    
    if [ ! -f "$action_file" ]; then
        echo "    ❌ Action file not found: $action_file"
        return 1
    fi
    
    # Validate YAML syntax
    validate_yaml "$action_file"
    
    # Check for required action fields
    if ! grep -q "^name:" "$action_file"; then
        echo "    ❌ Missing 'name' field in action"
        return 1
    fi
    
    if ! grep -q "^description:" "$action_file"; then
        echo "    ❌ Missing 'description' field in action"
        return 1
    fi
    
    if ! grep -q "^runs:" "$action_file"; then
        echo "    ❌ Missing 'runs' field in action"
        return 1
    fi
    
    echo "    ✅ Action is valid"
    return 0
}

# Main validation
echo ""
echo "📋 Validating workflow files..."

WORKFLOW_FILES=(
    "$WORKFLOW_DIR/ci.yml"
    "$WORKFLOW_DIR/update-cache.yml"
    "$WORKFLOW_DIR/security.yml"
    "$WORKFLOW_DIR/release.yml"
    "$WORKFLOW_DIR/dependency-updates.yml"
)

VALIDATION_FAILED=0

for workflow in "${WORKFLOW_FILES[@]}"; do
    echo ""
    echo "🔍 Validating: $workflow"
    
    if ! validate_yaml "$workflow"; then
        VALIDATION_FAILED=1
        continue
    fi
    
    if ! check_workflow_issues "$workflow"; then
        VALIDATION_FAILED=1
        continue
    fi
    
    echo "  ✅ Workflow validation passed"
done

echo ""
echo "📋 Validating action files..."

if [ -d "$ACTIONS_DIR" ]; then
    for action_dir in "$ACTIONS_DIR"/*; do
        if [ -d "$action_dir" ]; then
            echo ""
            if ! validate_action "$action_dir"; then
                VALIDATION_FAILED=1
            fi
        fi
    done
else
    echo "  ⚠️  Actions directory not found: $ACTIONS_DIR"
fi

echo ""
if [ $VALIDATION_FAILED -eq 0 ]; then
    echo "✅ All workflow validations passed!"
    exit 0
else
    echo "❌ Some validations failed. Please fix the issues above."
    exit 1
fi
