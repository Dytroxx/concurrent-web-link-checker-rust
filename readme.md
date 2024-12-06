# Concurrent Web Link Checker

## Overview

This project is a concurrent web link checker that tests the validity of links found on a set of target webpages. By providing a list of starting URLs via a configuration file or command-line arguments, the tool fetches each page, extracts all hyperlinks, and then checks the HTTP status codes of those links concurrently.

## Getting Started

1. **Install Rust and Cargo**:  
   Ensure you have Rust and Cargo set up.   
   [Install Rust](https://www.rust-lang.org/tools/install)

2. **Clone and Build**:
   ```bash
   git clone https://github.com/your-username/your-repo.git
   cd your-repo
   cargo build --release
   ```

3. **Run the program**:
    ```bash
    cargo run -- --config url_list.json
    ```

## Key Features

- **Input Flexibility**: Supply a list of target URLs either through a configuration file (JSON, TOML, etc.) or as command-line arguments.
- **Concurrent Processing**: Leverages asynchronous tasks to process multiple URLs and their respective links in parallel, improving speed and responsiveness.
- **HTML Parsing**: Automatically extracts links from fetched webpages using HTML parsing.
- **Comprehensive Reporting**: After checks are complete, the tool prints a summary including:
  - Total number of links discovered.
  - Count of valid links (e.g., those returning HTTP 200).
  - Count of links returning errors or non-success status codes.

## How It Works

1. **Read Input**: The program takes a list of starting URLs from a configuration file or directly from the command line.
2. **Fetch Pages**: Each URL is fetched concurrently, ensuring that slow or large pages don’t block the entire process.
3. **Extract Links**: The downloaded HTML is parsed to find all `<a href="...">` links.
4. **Check Links**: Each extracted link is probed to determine if it leads to a valid destination.
5. **Summarize Results**: Once all checks are finished, the tool prints a detailed summary of valid, invalid, and total links.

## Example Use Case

- Provide a `url_list.json` file containing:
  ```json
  {
    "urls": [
      "https://www.rust-lang.org"
    ]
  }