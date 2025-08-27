class Triangle:

    def __init__(self, width: float,length: float,height: float):
        self.width = width
        self.length = length
        self.height = height
        if self.width < 0 or self.height < 0 or self.length < 0:
            raise ValueError("Negative values not allowed")

    def is_triangle(self) -> bool:
        if self.width + self.length < self.height or self.height + self.length < self.width or self.width + self.height < self.length:
            return False
        else:
            return True
        
    def perimeter(self) -> float:
        return self.width + self.height + self.length
    
    def triangle_type(self) -> str:
        if self.width == self.height and self.height == self.length:
            return "Equilateral"
        if self.width != self.height and self.height != self.length and self.width != self.length: 
            return "Scalene"
        else:
            return "Isosceles"


