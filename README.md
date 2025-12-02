# Guessing Game (Rust)

A simple command-line **number guessing game** written in Rust.  
The program randomly chooses a number and asks the user to guess it, giving feedback if the guess is too low, too high, or correct.

---

## 🧩 How the game works

1. The program generates a random secret number between **1 and 100**.
2. It asks you to **type your guess**.
3. Your input (a string) is:
   - read from standard input,
   - trimmed,
   - converted to a number using `.parse()`.
4. Rust compares your guess with the secret number using `Ordering`:
   - `Ordering::Less` → **“too less”**
   - `Ordering::Greater` → **“too high”**
   - `Ordering::Equal` → **“you won”**

At the end, it prints a small “thanks / ok byeee” message and exits.

---

## 🛠 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed (via `rustup`)
- `cargo` available in your PATH

You can verify with:

```bash
rustc --version
cargo --version
