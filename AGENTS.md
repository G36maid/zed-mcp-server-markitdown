# Agent Guidelines for zed-mcp-server-markitdown

## Quick Start
1. Check `README.md` for project overview and development links
2. Review `todo.md` for current tasks
3. Use `cargo build` to compile and test changes
4. Install as dev extension in Zed for testing

## Development Workflow
- **Task Management**: Update `todo.md` after completing tasks
- **Complex Tasks**: Break down into smaller, prioritized subtasks
- **Testing**: stop and ask human for help

## Commands
```bash
cargo build    # Compile extension (cdylib)
cargo check    # Fast syntax/type checking
cargo fmt      # Format code
cargo clippy   # Lint for best practices
cargo test     # Run tests
```

## Code Standards
- **Style**: snake_case functions, PascalCase types
- **Imports**: std → external crates → local modules
- **Errors**: Use `Result<T, E>` with `?` operator
- **Extension API**: Implement `zed::Extension` trait with `context_server_command`

## Project Architecture
- `src/mcp_server_markitdown.rs` - Main Rust extension code
- `extension.toml` - Extension metadata and MCP server registration
- Wraps Python `markitdown-mcp` server for Zed integration
