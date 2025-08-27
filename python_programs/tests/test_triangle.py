from programs.triangle import Triangle

import pytest

def test_if_triangle():
    triangle = Triangle(3,4,5)
    assert triangle.is_triangle() is True

def test_invalid_triangle():
    triangle = Triangle(2,4,9)
    assert triangle.is_triangle() is False

def test_perimeter():
    triangle = Triangle(3,4,6)
    assert triangle.perimeter() == 13

def test_equilateral():
    triangle = Triangle(3,3,3)
    assert triangle.triangle_type() is "Equilateral"

def test_scalene():
    triangle = Triangle(3,4,5)
    assert triangle.triangle_type() is "Scalene"

def test_isosceles():
    triangle = Triangle(3,5,3)
    assert triangle.triangle_type() is "Isosceles"

def test_if_equilateral_be_isosceles():
    triangle = Triangle(4,4,4)
    assert not triangle.triangle_type() is "Isosceles"

def test_if_negatives_allowed():
    with pytest.raises(ValueError, match="Negative values not allowed"):
        triangle = Triangle(-3,-5,-3)

def test_if_floats_accepted():
    triangle = Triangle(2.3,4.5,6.7)
    assert triangle.is_triangle() is True 

def test_different_data_types():
    triangle = Triangle(4,9.8,12)
    assert triangle.is_triangle() is True

def test_if_strings_allowed():
    with pytest.raises(TypeError):
        triangle = Triangle("oi", "a", "b")

def test_if_tuples_allowed():
    with pytest.raises(TypeError):
        triangle = Triangle(("a"),(3),([5.67]))
