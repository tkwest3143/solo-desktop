#!/bin/bash

# Simple validation script for compact window feature
echo "🔍 Validating Compact Window Feature Implementation"
echo "=================================================="

# Check if required files exist
echo "✅ Checking required files..."

files=(
  "src/components/ResizeWindowButton.vue"
  "src/components/CompactAttendance.vue" 
  "src/components/CompactTodo.vue"
  "src/layouts/compact.vue"
)

for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    echo "  ✓ $file exists"
  else
    echo "  ✗ $file missing"
    exit 1
  fi
done

# Check if Header component includes ResizeWindowButton
echo ""
echo "✅ Checking Header integration..."
if grep -q "ResizeWindowButton" src/components/Header.vue; then
  echo "  ✓ ResizeWindowButton integrated in Header"
else
  echo "  ✗ ResizeWindowButton not found in Header"
  exit 1
fi

# Check if app.vue detects compact mode
echo ""
echo "✅ Checking app.vue compact mode detection..."
if grep -q "isCompactMode" src/app.vue; then
  echo "  ✓ Compact mode detection implemented"
else
  echo "  ✗ Compact mode detection missing"
  exit 1
fi

# Check if compact layout exists
echo ""
echo "✅ Checking compact layout..."
if grep -q 'name="compact"' src/app.vue; then
  echo "  ✓ Compact layout referenced in app.vue"
else
  echo "  ✗ Compact layout not referenced"
  exit 1
fi

# Check build success
echo ""
echo "✅ Checking build..."
if npm run build > /dev/null 2>&1; then
  echo "  ✓ Build successful"
else
  echo "  ✗ Build failed"
  exit 1
fi

echo ""
echo "🎉 All validations passed!"
echo ""
echo "📋 Summary of implemented features:"
echo "  • ResizeWindowButton with bottom-right positioning"
echo "  • Compact header with simplified navigation"
echo "  • CompactAttendance view with work control buttons"
echo "  • CompactTodo view with recent incomplete todos"
echo "  • Automatic layout switching between normal and compact modes"
echo "  • Window state management and resize detection"
echo ""
echo "🚀 The compact window feature is ready for testing!"