def user_info():
  print("Welcome to this simples calculator!")
  print("We are pleased to meet you.")
  print("What's your name?")
  username = input()
  if not isinstance(username, str):
    print(f"I apologize, {username}. I only accept valid names!")
    print("Would you mind providing me one with letters?")

  print(f"\nHello, {username}! Pleased to meet you. Shall we head to the calculator?")
  print("If you'd like that, please say yes. Otherwise, I'd interpret as nay.")
  user_response = input()
  sure = ["YES", "yes", "yeah", "yay", "sure", "ok", "cool", "nice", "yea", "k", "y"]
  if user_response in sure:
    calculator_menu(username)
  else:
    user_info()

def calculator_menu(username):
  print(f"\nWelcome to our simple calculator, {username}!")
  print("Which operation would you like?")
  print("We got sum, subtraction, multiplication and division")
  options = ["sum", "subtraction", "multiplication", "division"]
  user_option = input()
  if user_option not in options:
    print(f"\nI'm sorry, {username}. I'm afraid you didn't choose a valid option!")
    print("Please, do write either: sum; subtraction; multiplication or division")
    calculator_menu(username)
  print("Type the first number: ")
  first_num = float(input())
  if not isinstance(first_num, float):
    print("Well, that's not a valid number, you know.")
    print(f"Please, do type a number this time, {username}!") 
    calculator_menu(username)
  print("Type the second number now, please: ")
  second_num = float(input()) 
  if not isinstance(second_num, float):
    print("Well, that's not a valid number, you know.")
    print(f"Please, do type a number this time, {username}!") 
    calculator_menu(username)
  #There's an infinite loop around 'ere. Check that later!
  calculator_operation(user_option, first_num, second_num)

def calculator_operation(user_option, first_num, second_num):
  if user_option == "sum":
    calculator_sum(first_num, second_num)
  elif user_option == "subtraction":
    calculator_sub(first_num, second_num)
  elif user_option == "multiplication":
    calculator_mul(first_num, second_num)
  elif user_option == "division":
    calculator_div(first_num, second_num)
  else:
    print("That's not a valid option, I'm afraid. Please do write them correctly.")
  
def calculator_sum(first_num, second_num):
  print("The result is as follows:")
  print(f"{first_num} + {second_num} = {first_num + second_num}")
  

def calculator_sub(first_num, second_num):
  print("The result is as follows:")
  print(f"{first_num} - {second_num} = {first_num - second_num}")
  

def calculator_mul(first_num, second_num):
  print("The result is as follows:")
  print(f"{first_num} * {second_num} = {first_num * second_num}")
  

def calculator_div(first_num, second_num):
  print("The result is as follows:")
  print(f"{first_num} / {second_num} = {first_num / second_num}")
  

def main():
  user_info()

if __name__ == "__main__":
  main()
