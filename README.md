## Frankenpenguin

This is an attempt to eke more performance out of [bouncing-retro-rectangle-penguins](https://github.com/gen-alpha-xtor/bouncing-retro-rectangle-penguins)

#### Naive perf test

Currently the `webgl` crate performance diff stands at:

| rects | TS + WebGL (fps) | Rust + WebGL (fps) | this (fps) |
| ----- | ---------------- | ------------------ | ---------- |
| 100k  | 120              | 35                 | 120        |
| 200k  | 60               | 17                 | 120        |
| 400k  | 24               | 8                  | 96         |
| 1m    | 12               | 3                  | 28         |

https://github.com/user-attachments/assets/3a1ca212-f745-43b5-b0cb-6a4de9b5f293
