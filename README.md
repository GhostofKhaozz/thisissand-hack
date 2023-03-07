# Use This Instead
Most people should be using the user script version instead of this code.

You can find it [here on GreasyFork](https://greasyfork.org/en/scripts/461389-thisissand-image-hack).

# Image upload hack for ThisIsSand
This calls the same API that the ThisIsSand app does when you post your sand creations. It bypasses the *(terrible)* anticheat to let you post arbitrary JPEG-formatted images on the public gallery.

## Disclaimer
_PLEASE_ do not abuse this, or it will likely be patched quickly.

This is only a proof of concept to demonstrate the ineffectiveness of anticheat in web games that relies on obfuscation or "secrets". The code is released for educational purposes only. I am *NOT* responsible in *ANY* way for the misuse of this code, or for any resulting damages caused to the ThisIsSand servers.

# Prerequisites
You will probably need something like Gimp or ImageMagick to convert images to the right format. Needless to say, you will also need an Internet connection.

# How to Use
This program requires the following files in the same directory that you run it in:
- `username.txt`: Put any username here.
- `caption.txt`: Put a caption for your creation here.
- `image.jpg `: Copy the image you want to upload here. The image file can't exceed the uploaded file size limit (around 2mb I think), and it seems to get rejected by the server if it's not in JPEG format.

It will print out the MD5 hash of the file and the "secret" hash, and then attempt to submit the image to the public gallery before exiting.
