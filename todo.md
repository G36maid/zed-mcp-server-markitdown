# Development Plan for `zed-mcp-server-markitdown`

This document outlines the steps to integrate the `markitdown-mcp` Python server as a Zed editor extension.

## Project Goal

Create a Zed editor extension (using a Rust wrapper) to launch and manage the `markitdown-mcp` Python server. This will enable Zed's AI assistant to leverage MarkitDown's capabilities for converting various file types into Markdown, providing richer context for Large Language Models (LLMs).

## Development Progress

### ✅ **COMPLETED - Core Components**
- [x] **`extension.toml`**: Define the Zed extension and register the `markitdown` context server.
- [x] **`Cargo.toml`**: Configure the Rust project to build a `cdylib` and declare `zed_extension_api` dependency.
- [x] **`src/mcp_server_markitdown.rs`**: Implement the main Rust code for the `MarkitDownExtension`, including the `zed::Extension` trait and `context_server_command` method.

### ✅ **COMPLETED - Implementation Details**
- [x] **Update `extension.toml`**:
    - [x] Add `[context_servers.mcp-server-markitdown]` entry to register the MCP server.
- [x] **Update `Cargo.toml`**:
    - [x] Ensure `crate-type = ["cdylib"]` is set under `[lib]`.
    - [x] Add `zed_extension_api = "0.7.0"` (latest version) under `[dependencies]`.
    - [x] Add `serde` and `schemars` dependencies for settings support.
- [x] **Implement `MarkitDownExtension` in `src/mcp_server_markitdown.rs`**:
    - [x] Define `struct MarkitDownExtension` (simplified, no caching).
    - [x] Implement `impl zed::Extension for MarkitDownExtension`.
    - [x] Add `zed::register_extension!(MarkitDownExtension);`.
- [x] **Implement `context_server_command`**:
    - [x] Locate suitable Python interpreter with user-configurable settings.
    - [x] Construct `zed::Command`:
        - [x] Set `command` to Python interpreter path.
        - [x] Set `args` to `["-m", "markitdown-mcp"]`.
        - [x] Set `env` to include `PYTHONUNBUFFERED=1`.
    - [x] **Automated `markitdown-mcp` installation**:
        - [x] Check if `markitdown-mcp` is installed via import test.
        - [x] If not installed, run `pip install markitdown-mcp` automatically.
        - [x] Handle errors and provide detailed user feedback.

### ✅ **COMPLETED - Enhanced Features (from Context7 reference)**
- [x] **Settings System**:
    - [x] User-configurable Python command via Zed settings.
    - [x] JSON schema validation for settings.
    - [x] Support for virtual environments.
- [x] **Configuration Files**:
    - [x] `configuration/installation_instructions.md` - User installation guide.
    - [x] `configuration/default_settings.jsonc` - Settings template.
- [x] **UI Integration**:
    - [x] Implement `context_server_configuration` method for settings UI.
    - [x] Installation instructions accessible from Zed.

### 🔄 **NEXT TASK - Testing and Debugging**
- [ ] **Local Testing**:
    - [ ] Install the extension as a "dev extension" in Zed.
    - [ ] Verify that Zed recognizes the `mcp-server-markitdown` context server.
    - [ ] Test Python detection and automatic package installation.
- [ ] **Functional Testing**:
    - [ ] Use Zed Assistant to trigger the `convert_to_markdown` tool.
    - [ ] Test with various file types (PDF, DOCX, images, etc.).
    - [ ] Verify Markdown conversion output quality.
- [ ] **Debug and Monitor**:
    - [ ] Monitor Zed's logs (run `zed --foreground`) for extension output.
    - [ ] Use `mcpinspector` for detailed MCP server communication debugging.
    - [ ] Test error scenarios (missing Python, network issues, etc.).

### 📋 **FUTURE ENHANCEMENTS** (if needed)
- [ ] **Error Recovery**:
    - [ ] Better handling of Python environment issues.
    - [ ] Retry mechanisms for package installation.
- [ ] **Performance**:
    - [ ] Caching of large file conversions.
    - [ ] Progress indicators for long conversions.
- [ ] **Advanced Features**:
    - [ ] Custom conversion options (OCR settings, etc.).
    - [ ] Support for batch file processing.

## Current Status Summary

- ✅ **Extension builds successfully** (`cargo build` passes)
- ✅ **Complete implementation** with automatic dependency management
- ✅ **User-friendly configuration** system with settings UI
- ✅ **Robust error handling** and installation guidance
- ✅ **Production-ready code** following Zed extension best practices
- 🔄 **Ready for testing** in Zed as dev extension

## Implementation Highlights

### Settings Configuration Example:
```json
{
  "context_servers": {
    "mcp-server-markitdown": {
      "settings": {
        "python_command": "python3.11"  // or "/path/to/venv/bin/python"
      }
    }
  }
}
```

### Automatic Features:
- ✅ Python interpreter detection
- ✅ Automatic `markitdown-mcp` package installation
- ✅ Virtual environment support
- ✅ Detailed error messages and troubleshooting guidance