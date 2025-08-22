## Frankenpenguin

This is an attempt to eke more performance out of [bouncing-retro-rectangle-penguins](https://github.com/gen-alpha-xtor/bouncing-retro-rectangle-penguins)

#### Naive perf test
Currently the `webgl` crate performance diff stands at:

| rects | original (fps) | this (fps) |
|-------|----------------|------------|
| 100k  | 120            | 120        |
| 200k  | 60             | 120        |
| 400k  | 24             | 96         |
| 1m    | 12             | 28         |

<img width="1377" height="649" alt="Screenshot 2025-08-21 at 5 19 42 PM" src="https://github.com/user-attachments/assets/5f09c5ff-abe1-4596-a1c9-893c403408ca" />
