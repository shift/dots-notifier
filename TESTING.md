# dots-notifier Testing Guide

This document provides comprehensive information about the testing infrastructure for the dots-notifier project.

## Test Structure

The project includes several types of tests to ensure 100% code coverage and robust functionality:

### 1. Unit Tests (`src/*/tests/`)
- **Location**: Embedded in each module (`src/cli.rs`, `src/types.rs`, etc.)
- **Purpose**: Test individual functions, structs, and modules in isolation
- **Coverage**: 
  - CLI argument parsing validation
  - TargetUser struct operations
  - Notification validation
  - Session filtering logic
  - D-Bus interface definitions

### 2. Integration Tests (`tests/integration_tests.rs`)
- **Purpose**: Test interaction between components and real-world scenarios
- **Coverage**:
  - Full CLI parsing workflows
  - User session filtering and collection
  - Notification validation boundaries
  - Unicode and special character handling
  - Error scenarios and edge cases
  - Concurrent operations

### 3. Property-Based Tests (`src/proptests.rs`)
- **Framework**: [proptest](https://crates.io/crates/proptest)
- **Purpose**: Test properties that should hold for all inputs
- **Coverage**:
  - TargetUser consistency properties
  - Notification validation boundaries
  - Session type detection consistency

### 4. QuickCheck Tests (`tests/quickcheck_tests.rs`)
- **Framework**: [quickcheck](https://crates.io/crates/quickcheck)
- **Purpose**: Random input generation for property testing
- **Coverage**:
  - Equality and hash consistency
  - User collection behavior
  - Session filtering properties

### 5. Performance Tests (`tests/benchmarks.rs`)
- **Purpose**: Ensure performance requirements are met
- **Coverage**:
  - User creation and operations
  - Session filtering performance
  - Notification validation speed
  - Memory usage patterns
  - Concurrent operation simulation

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test Categories
```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration_tests

# Property-based tests
cargo test --test quickcheck_tests

# Performance benchmarks
cargo test --test benchmarks
```

### With Coverage
```bash
# Run the coverage script
./coverage.sh

# Or manually with tarpaulin
cargo tarpaulin --all-features --workspace --timeout 120
```

## Test Dependencies

The following testing frameworks and utilities are used:

- `tokio-test`: Async testing utilities
- `proptest`: Property-based testing
- `quickcheck`: Random input generation
- `quickcheck_macros`: Macros for quickcheck
- `tempfile`: Temporary file operations
- `serial_test`: Sequential test execution when needed
- `futures-test`: Async futures testing
- `pretty_assertions`: Better assertion failure output

## Coverage Goals

The project aims for 100% code coverage across:

1. **All public APIs**: Every public function and method
2. **Error paths**: All error conditions and edge cases
3. **Branch coverage**: Every branch in match statements and conditionals
4. **Integration scenarios**: Real-world usage patterns
5. **Performance characteristics**: Ensure efficient operation

## Test Features

### Mocking and Simulation
Since the application relies on D-Bus and system resources, tests include:
- Simulated session data for filtering tests
- Mock notification scenarios
- Concurrent operation simulation
- Error condition simulation

### Unicode and Internationalization
Tests include:
- Unicode usernames and notification content
- Special characters and formatting
- Edge cases with empty/malformed strings

### Performance Validation
- Algorithmic complexity verification
- Memory usage patterns
- Concurrent operation safety
- Stress testing with large datasets

### Error Handling
- Invalid input validation
- System resource unavailability
- D-Bus communication failures
- Boundary condition testing

## Continuous Integration

The tests are designed to run in CI environments:
- No external dependencies required for core tests
- Timeouts configured for CI execution
- Deterministic test behavior
- Clear error reporting

## Adding New Tests

When adding new functionality:

1. **Add unit tests** in the appropriate module
2. **Update integration tests** for new workflows
3. **Consider property-based tests** for new data types
4. **Add performance tests** for computational operations
5. **Test error conditions** and edge cases

## Debugging Tests

For debugging failing tests:

```bash
# Run with debug output
RUST_LOG=debug cargo test

# Run specific test with output
cargo test test_name -- --nocapture

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo test
```

## Test Organization

Tests are organized by complexity and scope:
- **Unit tests**: Fast, isolated, deterministic
- **Integration tests**: Moderate complexity, component interaction
- **Property tests**: High coverage, random inputs
- **Performance tests**: System resource usage validation

This comprehensive testing approach ensures the reliability, performance, and maintainability of the dots-notifier application.