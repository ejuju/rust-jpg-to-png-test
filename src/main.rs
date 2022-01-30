use image::GenericImageView;

fn main() {
    let filename = "src/input_img.jpg";
    let img = image::open(filename).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("output.png").unwrap();
}
