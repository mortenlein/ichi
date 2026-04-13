from PIL import Image, ImageOps, ImageDraw

def get_red_circle_center(im):
    r, g, b = im.convert("RGB").split()
    mask = r.point(lambda p: 255 if p > 150 else 0)
    g_mask = g.point(lambda p: 255 if p < 100 else 0)
    b_mask = b.point(lambda p: 255 if p < 100 else 0)
    
    final_mask = Image.new("L", im.size, 0)
    pixels = final_mask.load()
    r_pix = mask.load()
    g_pix = g_mask.load()
    b_pix = b_mask.load()
    
    for y in range(im.height):
        for x in range(im.width):
            if r_pix[x,y] == 255 and g_pix[x,y] == 255 and b_pix[x,y] == 255:
                pixels[x,y] = 255
    
    bbox = final_mask.getbbox()
    if bbox:
        left, top, right, bottom = bbox
        return (left + right) // 2, (top + bottom) // 2
    return im.width // 2, im.height // 2

def convert_to_icon(input_path, output_path, png_output_path):
    img = Image.open(input_path).convert("RGBA")
    
    # Find the red circle center to center the crop
    center_x, center_y = get_red_circle_center(img)
    
    # Find content bbox
    gray = img.convert("L")
    inverted = ImageOps.invert(gray)
    mask = inverted.point(lambda p: 255 if p > 30 else 0)
    bbox = mask.getbbox()
    
    if bbox:
        l, t, r, b = bbox
        width = r - l
        height = b - t
        size = max(width, height) + 40
        
        half_size = size // 2
        crop_box = (center_x - half_size, center_y - half_size, center_x + half_size, center_y + half_size)
        
        # Ensure we don't go out of bounds
        crop_box = [
            max(0, crop_box[0]),
            max(0, crop_box[1]),
            min(img.width, crop_box[2]),
            min(img.height, crop_box[3])
        ]
        
        # Adjust crop_box to be square if it hit edges
        final_w = crop_box[2] - crop_box[0]
        final_h = crop_box[3] - crop_box[1]
        final_size = max(final_w, final_h)
        
        cropped_img = img.crop(crop_box)
        
        # If not square, pad it to square with transparency
        if final_w != final_h:
            new_img = Image.new("RGBA", (final_size, final_size), (255, 255, 255, 0))
            new_img.paste(cropped_img, ((final_size - final_w) // 2, (final_size - final_h) // 2))
            cropped_img = new_img

        # Optional: Transparency for the very corners if they are pure white
        # This helps with the rounded square look on different system themes
        # However, the shadow makes this tricky. We'll stick to the current clean crop.
        
        icon_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
        cropped_img.save(output_path, sizes=icon_sizes)
        cropped_img.save(png_output_path)
        print(f"Icon saved: {output_path} and {png_output_path}")
    else:
        print("Could not find content to crop.")

if __name__ == "__main__":
    input_img = "ichi_hinomaru_logo_1776067137884.png"
    convert_to_icon(input_img, "ichi.ico", "icon.png")
