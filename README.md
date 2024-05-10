# CommitGuard

CommitGuard is a Rust command-line application designed to manage git hooks for [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). It simplifies adding and removing a custom commit message validator in your git repository by modifying the `commit-msg` hook.

## Features

- **Initialize Commit Message Hook**: Sets up a commit message hook in your git repository to enforce conventional commit formats.
- **Remove Commit Message Hook**: Removes the commit message hook from your git repository.

## Installation

To install and run CommitGuard, you'll need to have Rust and Cargo installed on your machine.

1. Install Rust and Cargo using [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:<br>
    ```bash
    git clone [TODO: add url]
    ```

    ```bash
    cd commitguard
    ```

3. Build the project:
    ```bash
    cargo build --release
    ```

4. The executable will be in ./target/release/guard. You can move it to a location in your PATH for easier access. <br> Linux:
    ```bash
    sudo mv target/release/guard /usr/local/bin/
    ```

## Usage

After installation, you can run CommitGuard using the following commands:
| command | long    | short | description                       |
|---------|---------|-------|-----------------------------------|
| init    |--init   |  -i   |Sets up commit guard for the repo  |
| remove  |--remove |  -r   |Removes the guard                  |
| version |--version|  -v   |Prints version number              |
| help    |--help   |  -h   |Print help information             |

## Example

```bashrc
cd myRepo
guard --init
touch README.md
git add README.md
git commit -m "rubbish commit message"
---
Error: Invalid commit-message. Please follow the conventional commit format:
    <type>[optional scope]: <description>
    [optional body]
    [optional footer(s)]
Valid types include: feat, fix, docs, style, refactor, perf, test, chore
---
git commit -m "docs: add readme file"
```

If you need to remove the hook, simply run:
```
guard --remove
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open issues to improve the functionality or documentation of CommitGuard.

## License

Copyright 2024 Johannes Haukland

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
