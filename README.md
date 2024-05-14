
# 🦀 Rust Command-Line Tool

## Overview
This project involves a command-line tool where commands like `ping` can be executed interactively. It accepts parameters like `destination_ip` and is designed to be lightweight and efficient.

## Project Goals
- Create a command-line tool using Rust with minimal external dependencies.
- Focus on performance and reliability.
- Developed and tested with Rust 1.58.0.

## Getting Started

### Installation
Clone the repository and navigate to the project directory:
```bash
git clone https://github.com/dev-nolant/Rust-Command-Line-Tool.git
cd Rust-Command-Line-Tool
```

Build the project using Cargo:
```bash
cargo build
```

### Usage Example
Here's a simple example to demonstrate how to use the command-line tool:

```bash
cargo run 
```

### Project Structure
- `main.rs`: Contains the main entry point for the application.
- `ping.rs`: Handles the ping functionality.
- `build.rs`: Contains build script logic.
- `.gitignore`: Specifies files and directories to ignore in Git.
- `Cargo.toml`: Contains metadata and dependencies for the project.
- `Cargo.lock`: Records the exact versions of dependencies used.
- `Packet.lib`: Contains packet handling logic.

### External Usage
If you plan to implement this project into your program, please add a comment noting the contribution and create an "Issue" with a link to your code. I'd love to see the cool implementations you create!

## Contributing
Contributions are welcome! Feel free to fork the repository, make changes, and submit a pull request. You will be listed as a contributor.

## Contact
I will respond to pull requests and issues. Don't hesitate to reach out with any questions or feedback.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
