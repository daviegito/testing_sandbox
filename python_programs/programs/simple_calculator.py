import math
class Calculator():
  def __init__(self, a, b, option):
    self.a = a
    self.b = b
    self.option = option

  def sum(self, a, b):
    sum = a+b
    return sum

  def sub(self, a, b):
    sub = a-b
    return sub

  def mul(self, a, b):
    return a*b
  
  def div(self, a, b): 
    return a/b
  
  def exp(self, a, b):
    return math.pow(a,b)
  
  def sqrt_a(self, a, b):
    return math.sqrt(a)
    
  def sqrt_b(self, a, b):
    return math.sqrt(b)

  def operations(self, a, b, option):
    if option == "sum":
      sum(a,b)
    if option == "sub":
      sub(a,b)
    if option == "mul":
      mul(a,b)
    if option == "div":
      div(a,b)
    if option == "exp":
      exp(a,b)
    if option == "sqrt":
      a_or_b = input("a or b?")
      if a_or_b == "a":
        sqrt_a(a,b)
      elif a_or_b == "b":
        sqrt_b(a,b)
      else:
        print("Something is wrong. Try again!")
    else:
      print("No option was typed. Terminating program!")

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