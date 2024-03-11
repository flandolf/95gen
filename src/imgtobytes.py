path = "../assets/icon.png"

# Open the image file
with open(path
            , "rb") as image_file:
        # Read the image file
        image_bytes = image_file.read()
        # print as a rust byte array
        print("pub const ICON: &[u8] = &[", end="")
        for byte in image_bytes:
            print(f"{byte}, ", end="")
        print("];")
