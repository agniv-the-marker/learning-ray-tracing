# Ray Tracing in a Weekend

Uploading my old code from this course to motivate me to finish it. Please check out `notes` folder for a full picture of what's going on. TODO: write the notes for rust as well, since it has its own things.

Learning `c++` and the basics of `c` made for a much smoother experience during this whole thing.

![render](image.jpg)

## Rust

Rewriting it in rust. Please check out blog post.  I also want to make sure my rust is up to speed. Advent of code is fun, but I like this more.

## Extra

Use the following code to get the render within the tutorial directory:

```bash
g++ -o main main.cc
./main > image.ppm
python converter.py
```

For the rust version, use the following:

```bash
cargo run > image.ppm # don't run quietly
python converter.py
```

Also, copilot can generate tests if you want, but make sure to do `cargo test` as well! Linting game is strong.
