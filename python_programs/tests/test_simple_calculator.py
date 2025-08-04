import pytest
from programs.simple_calculator import Calculator

def test_sum():
  assert sum(12,21) == 33
def test_sub():
  assert sub(21,12) == 9
def test_mul():
  assert mul(12,3) == 36
def test_div():
  assert div(12,3) == 4
def test_non_numbers():
  assert sum("hello, ", "sup") == "hello, sup"