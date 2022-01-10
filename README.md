# RSCMM

1. A C language Interpreter Library.
2. A practice project to educate myself.
3. RSCMM is a combine of Rust & CMM. CMM is contrary to CPP, means less than C.

# Pipeline
<div><a href='//sketchviz.com/@mkchaos/c5a7dd802231fdfa4444bc6348bffb02'><img src='https://sketchviz.com/@mkchaos/c5a7dd802231fdfa4444bc6348bffb02/ba341d74fa8cc4d5fb92a4668e7e45c2ef20d64d.sketchy.png' style='max-width: 100%;'></a><br/><span style='font-size: 80%;color:#555;'></div>

# Including RSCMM in your project

```toml
[dependencies]
rscmm = "0.3"
```

# Implementation

## src folder

+ parser in src/core/parser
+ semantic_analyzer in src/core/analyzer
+ compiler in src/core/compiler
+ vm in src/core/vm   vm is used to run codes after compiling.

## functionality

+ ops: + - * / % && || ! == != <= >= < >
+ funcs: declare, impls, recursive
+ types: int, void
+ controls: if & while

# Example

```c
int gcd(int a, int b) {
    if (b == 0) {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

int main() {
    int p = gcd(99, 90);
    p;   // single variable statement means print.
}
```

```rust
// Run example
rscmm::compile_and_run("example/gcd.c").unwrap();
// Get vm codes
rscmm::compile_to_code("example/gcd.c").unwrap();
```