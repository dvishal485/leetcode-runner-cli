# leetcode-runner-cli

Executes leetcode testcases and submits your solution through CLI interface

**Disclaimer :** This is not an official Leetcode tool. I am not affiliated with Leetcode in any way. This tool is not endorsed by leetcode.

---

## Installation

1. Install [rust from here](https://www.rust-lang.org/tools/install) to compile the source code.

1. Clone the repository and install the binary

    ```bash
    git clone https://github.com/dvishal485/leetcode-runner-cli.git
    cd leetcode-runner-cli
    cargo install --path .
    ```

1. Setup environment variable `LEETCODE_SESSION` with your leetcode session cookie.

   You can get your session cookie by logging in to leetcode and inspecting the cookie in your browser's developer tools.

1. Execute the tool and verify your authentication

    ```bash
    leetcode-runner-cli -a
    ```

---

## Usage

```bash
leetcode-runner-cli [FLAGS] [OPTIONS <option>]
```

### Flags

| Flag            | Description                 |
| :-------------- | :-------------------------- |
| `-h, --help`    | Prints help information     |
| `-V, --version` | Prints version information  |
| `-a, --auth`    | Authenticate with leetcode  |
| `-s, --submit`  | Submit your solution to leetcode |

### Options

| Option           | Description                 |
| :--------------- | :-------------------------- |
| `-f, --file`     | Path to your solution file  |
| `-t, --testcase` | Testcase file to run        |
| `-q, --question` | Question title to fetch     |

### File changes

The file you submit to leetcode shouldn't have driver code like main function or struct definition. But no need to manually remove it. The tool will automatically remove the driver code and submit the solution to leetcode. All you need to do is put the delimiters `#LCSTART` and `#LCEND` in your solution file in comments, and *place leetcode problem link anywhere* in the file.

For example :

```rust
struct Solution;

// https://leetcode.com/problems/two-sum/ #LCSTART

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // real magic here
    }
} // #LCEND

fn main() {
 // can have anything which may assist you
}
```

- In case link is not found, the tool will exit with error message for the same.
- In case start delimiter is not found, the tool will run till the end delimiter.
- In case end delimiter is not found, the tool will run till the end of file.
- In case both start and end delimiters are not found, the tool will default to the whole file.

### Example usage

- Fetch question [koko-eating-bananas](https://leetcode.com/problems/koko-eating-bananas/)

    ```bash
    leetcode-runner-cli -q koko-eating-bananas
    ```

Note : [File should have the link of question in the comments](#file-changes) for the following examples.

- Run src/main.rs with default testcases for question [koko-eating-bananas](https://leetcode.com/problems/koko-eating-bananas/)

    ```bash
    leetcode-runner-cli -f ./src/main.rs
    ```

- Run src/main.rs with custom testcase file

    ```bash
    leetcode-runner-cli -f ./src/main.rs -t ./testcase.txt
    ```

- Submit src/main.rs to leetcode

    ```bash
    leetcode-runner-cli -f ./src/main.rs -s
    ```

    Note : This will first execute the default testcases and then submit the solution to leetcode only if the testcases pass as a preventive measure to avoid submitting wrong solution.

---

## Languages supported

This is a generic module that can be used to run any language. It only needs a mapping to the `language` on leetcode and `extension` of the file.

Currently, the following languages are added by default :
Rust, Python3, Cpp, Java, C, Javascript, Go, Kotlin, Swift, Typescript,

More languages can be added manually as per requirement by [changing enum](https://github.com/dvishal485/leetcode-runner-cli/blob/main/src/codefile.rs#LL3)
 in the `src/codefile.rs` file.

---

## License & Copyright

- This Project is [Apache-2.0](./LICENSE) Licensed
- Copyright 2023 [Vishal Das](https://github.com/dvishal485)

---
