class Triangle:

    def __init__(self, width,length,height):
        self.width = width
        self.length = length
        self.height = height
        if self.width < 0 or self.height < 0 or self.length < 0:
            raise ValueError("Negative values not allowed")

    def is_triangle(self):
        if self.width + self.length < self.height or self.height + self.length < self.width or self.width + self.height < self.length:
            return False
        else:
            return True
        
    def perimeter(self):
        return self.width + self.height + self.length
    
    def triangle_type(self):
        if self.width == self.height and self.height == self.length:
            return "Equilateral"
        if self.width != self.height and self.height != self.length and self.width != self.length: 
            return "Scalene"
        else:
            return "Isosceles"


