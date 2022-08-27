"""Generates the spritesheet for explosion clouds"""


from PIL import ImageColor, Image, ImageDraw
from dataclasses import dataclass, fields, astuple
from abc import ABC
from typing import SupportsFloat, Tuple
from pathlib import Path

@dataclass
class Arithmetic(ABC):
    """
    An abstract class that provides 
    - "vector" addition/subtraction
    - scalar multiplication/division
    to child classes
    """

    def __add__(self, other):
        """Adds to another instance of the same class by adding all fields together"""
        try:
            return self.__class__(*(getattr(self, dim.name)+getattr(other, dim.name) for dim in fields(self)))
        except AttributeError:
            return NotImplemented

    def __sub__(self, other):
        """Subtracts from another instance of the same class by subtracting each field in self from matching field in other"""
        try:
            return self.__class__(*(getattr(self, dim.name)-getattr(other, dim.name) for dim in fields(self)))
        except AttributeError:
            return NotImplemented

    def __mul__(self, other: SupportsFloat):
        """Scalar multiplication across all fields"""
        try:
            return self.__class__(*(getattr(self, dim.name)*other for dim in fields(self)))
        except AttributeError:
            return NotImplemented

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other: SupportsFloat):
        """Scalar division across all fields"""
        return self * (1/other)


@dataclass
class Color(Arithmetic):
    h: float
    s: float
    v: float
    a: float = 255.0

    def to_rgba_tuple(self) -> Tuple[int, int, int, int]:
        (r, g, b) = ImageColor.getrgb(f"hsv({self.h}, {self.s}%, {self.v}%)")
        return (int(r), int(g), int(b), int(self.a))

@dataclass
class Explosion(Arithmetic):
    """
    A dataclass representing an explosion
    """
    color: Color
    radius: float

    def render_explosion(self) -> Image.Image:
        img_size = 300

        # image will always be a square
        img = Image.new("RGBA", (img_size, img_size), (0, 0, 0, 0))
        d: ImageDraw.ImageDraw = ImageDraw.Draw(img)
        top_left = (img_size / 2) - self.radius
        bot_right = (img_size / 2) + self.radius
        bbox = ((top_left, top_left), (bot_right, bot_right))
        d.ellipse(bbox, fill=self.color.to_rgba_tuple())

        
        return img
        

def interpolation_func(x: float) -> float:
    return x


if __name__ == "__main__":
    start = Explosion(
        color = Color(60, 84, 99, a=255),
        radius = 0
    )
    end = Explosion(
        color = Color(22, 84, 99, a=0),
        radius = 150.
    )

    num_frames = 20
    diff = end - start
    delta: Explosion = diff/(num_frames - 1)

    # call to interpolation func lets us do things like ease in, ease out, etc etc
    adjusted_delta_factors = (interpolation_func(x/num_frames) * num_frames for x in range(int(num_frames)))
    delta_frames_iter = (delta * x for x in adjusted_delta_factors)
    explosion_frames_iter = (start + delta_frame for delta_frame in delta_frames_iter) 

    # render each explosion frame to an image
    explosion_images = list(frame.render_explosion() for frame in explosion_frames_iter)
    
    # stack them all into a sprite sheet
    total_height = sum(img.height for img in explosion_images)
    width = explosion_images[0].width

    final_sprite_sheet = Image.new("RGBA", (width, total_height), (0, 0, 0, 0))

    y = 0
    for i, img in enumerate(explosion_images):
        final_sprite_sheet.paste(img, (0, y))
        y += img.height

    current_file = Path(__file__)
    assets_folder = current_file.parent.parent / 'tactician-bevy' / 'assets' / 'images'
    final_sprite_sheet.save(assets_folder / 'explosion_spritesheet.png')
