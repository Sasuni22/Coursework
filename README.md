# Coursework
This repository demonstrates how a simple square(x) operation behaves across different programming paradigms and languages. It highlights how compilers, macros, templates, and dynamic execution handle code in distinct ways.

📌 Overview

The project explores four approaches:

C++ Macros – Preprocessor-based substitution
C++ Templates – Type-safe compile-time abstraction
Rust Macros – Hygienic and safe macro system
Python – Dynamic typing and runtime execution

Each implementation reveals subtle but important differences in evaluation, safety, and flexibility.

⚙️ Implementations
1. C++ Macros

A macro is defined using the preprocessor:

#define SQUARE(x) ((x) * (x))
Key Observation

Using:

int i = 3;
int result = SQUARE(i++);
The macro expands inline.
i++ is evaluated twice, leading to unexpected behavior.
Demonstrates the dangers of macros due to lack of evaluation control.
2. C++ Templates

A safer alternative using templates:

template <typename T>
T safe_square(T x) {
    return x * x;
}
Key Observation

Using:

int j = 3;
int safe_result = safe_square(j++);
Argument is evaluated once.
Ensures predictable and type-safe behavior.
Preferred over macros in modern C++.
3. Rust Macros

Rust uses hygienic macros:

macro_rules! square {
    ($x:expr) => {
        ($x * $x)
    };
}
Key Observation

Using:

let mut k = 3;
let result = square!({ k += 1; k });
Macros are hygienic (no unintended variable conflicts).
Expressions are explicit and controlled.
Safer than traditional macros in C/C++.
4. Python Dynamic Programming

Python uses a simple function:

def python_square(x):
    return x * x
Key Observation
python_square(3)
python_square(5.5)
Works seamlessly with different types.
No need for templates due to dynamic typing.
⚡ Dynamic Execution (Python)

Python also allows executing expressions at runtime:

formula = "x * x + 2"
x = 4
result = eval(formula)
Key Insight
Enables flexible runtime computation.
Useful for interpreters, calculators, and AI pipelines.
⚠️ Must be used carefully due to security risks.
🔍 Key Takeaways
Macros (C++): Powerful but unsafe due to multiple evaluations.
Templates (C++): Safer and type-aware alternative.
Rust Macros: Hygienic and expressive, avoiding common macro pitfalls.
Python: Highly flexible with dynamic typing and runtime execution.
🚀 How to Run
C++
g++ main.cpp -o main
./main
Rust
rustc main.rs
./main
Python
python main.py
