# MarkitDown MCP Server for Zed

This extension integrates [MarkitDown](https://github.com/microsoft/markitdown) as a Model Context Protocol (MCP) Server for for Zed's Assistant.

for upstream information, check [MarkitDown-MCP](https://github.com/microsoft/markitdown/tree/main/packages/markitdown-mcp)

## What This Extension Does

**Turn any Documents into AI Context** - Upload or reference any file, and this extension converts it to clean Markdown that AI can understand and work with without Multimodality.

### Supported File Types

- **Single Tool**: `convert_to_markdown` handles all conversions
- **URI Support**: Accepts `http:`, `https:`, `file:`, and `data:` URIs
- **Streaming**: Efficient processing of large documents

| Category | Formats | AI Use Cases |
|----------|---------|-------------|
| **Documents** | PDF, Word (.docx), Excel (.xlsx), PowerPoint (.pptx) | Analyze reports, summarize presentations, extract data |
| **Images** | JPG, PNG, GIF (with OCR) | Read text from screenshots, analyze charts, describe images |
| **Audio** | MP3, WAV (with transcription) | Transcribe meetings, analyze speech content |
| **Web** | HTTP/HTTPS URLs | Summarize articles, extract key information |
| **Data** | CSV, JSON, XML | Process structured data, generate insights |
| **Archives** | ZIP files | Analyze multiple files at once |

## Quick Start

### Installation (BETA)

```bash
# Clone the repo
git clone https://github.com/G36maid/zed-mcp-server-markitdown.git
cd zed-mcp-server-markitdown

# Build the extension
cargo build --release

# Add as a dev extension in Zed:
# In Zed, go to Extensions → Install Dev Extension → Select this directory
```

### Usage Examples

**Convert a PDF report:**
```
Please analyze this PDF report and summarize the key findings.
```

**Extract data from a spreadsheet:**
```
Convert this Excel file to markdown and show me the top 5 entries.
```

**Analyze a webpage:**
```
Summarize the main points from this article form link in README.
```

##  Configuration

### Optional Settings

Access through Zed's settings or the Assistant configuration UI:

```json
{
  "context_servers": {
    "mcp-server-markitdown": {
      "settings": {
        "package_version": "latest",
        "debug_mode": false
      }
    }
  }
}
```

- **`package_version`**: Specify npm package version (default: "latest")
- **`debug_mode`**: Enable detailed logging for troubleshooting (default: false)

### Prerequisites

- **Node.js and npm** (automatically handled by Zed)
- **Rust** (automatically handled by Zed)
- **Internet connection** (for initial package installation and web calls)



### Local Development

```bash
# Clone the repo
git clone https://github.com/G36maid/zed-mcp-server-markitdown.git
cd zed-mcp-server-markitdown

# Build the extension
cargo build --release

# Add as a dev extension in Zed:
# In Zed, go to Extensions → Install Dev Extension → Select this directory
```

## Related Projects

- **[Microsoft MarkItDown](https://github.com/microsoft/markitdown)** - The core conversion tool (Python)
- **[MarkItDown MCP](https://github.com/microsoft/markitdown/tree/main/packages/markitdown-mcp)** - Official MCP server
- **[markitdown-mcp-npx](https://github.com/xkiranj/markitdown-mcp-npx)** - NPX wrapper used by this extension
- **[Context7 MCP](https://github.com/akbxr/zed-mcp-server-context7)** - Similar Zed MCP extension example


##  License

MIT License - see [LICENSE](LICENSE) file for details.

##  Acknowledgments

- **Microsoft AutoGen Team** for creating MarkItDown
- **Zed Industries** for the excellent extension platform
- **xkiranj** for create the npx wrapper for MarkItDown MCP

---

**Transform your documents into AI-ready context with MarkItDown MCP Server for Zed.**
