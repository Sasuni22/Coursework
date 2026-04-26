def python_square(x):
    return x * x

# Static calls
print("Square of 3:", python_square(3))
print("Square of 5.5:", python_square(5.5))

# Dynamic execution
formula = "x * x + 2"

x = 4
result = eval(formula)

print("Dynamic formula result:", result)