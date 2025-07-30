import math

class Calculator():
  def __init__(self, a, b, option):
    self.a = a
    self.b = b
    self.option = option
    
  def operations(self, a, b, option):
    if option == "sum":
      print(f"{a} + {b} = {a+b}")
    if option == "sub":
      print(f"{a} - {b} = {a-b}")
    if option == "mul":
      print(f"{a} * {b} = {a*b}")
    if option == "div":
      print(f"{a} / {b} = {a/b}")    
    if option == "exp":
      print(f"{a} ^ {b} = {math.pow(a,b)}")
    if option == "sqrt":
      print(f"Square root of {a} = {math.sqrt(a)}")
      print(f"Square root of {b} = {math.sqrt(b)}")

print("Could you type your name?")
name = input()

print(f"Hello, {name}. What are we gonna calculate today?")
print("A couple of options for you to type: sum, sub, mul, div, exp (exponential) and sqrt (square root)")
options_possible = ["sum", "sub", "mul", "div", "exp", "sqrt"]
option = input()
if option in options_possible:
  print("Choose 2 numbers to work with in the operation!")
  a = int(input())
  b = int(input())
  calculator = Calculator(a, b, option).operations(a, b, option)  
else:
  print("Please, do type a valid option")