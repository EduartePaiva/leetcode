x = 11
y = 32
z = 51

x^=(1<<y)
x^=(1<<z)
x^=(1<<z)
x^=(1<<y)

print(x.bit_count())