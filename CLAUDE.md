# STINGRAY DEVELOPMENT GUIDE

## Build Commands
- `cargo build` - Build the project
- `cargo run` - Run the raytracer application
- `cargo build --release` - Build optimized release version

## Lint/Format Commands
- `cargo fmt` - Format code using rustfmt
- `cargo clippy` - Run the Clippy linter to catch common mistakes

## Test Commands
- `cargo test` - Run all tests
- `cargo test test_name` - Run a specific test by name
- `cargo test module::test_name` - Run tests in specific module
- `cargo test -- --nocapture` - Run tests with println output

## Code Style Guidelines
- **Imports**: stdlib first, then external crates, then internal modules
- **Naming**: PascalCase for structs/enums, snake_case for functions/variables
- **Formatting**: 4-space indentation, default rustfmt rules
- **Types**: Use appropriate numeric types for geometric operations (f64)
- **Error Handling**: Propagate errors in critical paths, use Result where appropriate
- **Testing**: Place unit tests in `#[cfg(test)]` modules, use descriptive test names
- **Documentation**: Add doc comments to public functions and types