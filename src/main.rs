fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    // Render
    (0..image_height).for_each(|y| {
        (0..image_width).for_each(|x| {
            let r = x as f32 / (image_width - 1) as f32;
            let g = y as f32 / (image_height - 1) as f32;
            let b = 0f32;

            let ir = (256. * r) as u32;
            let ig = (256. * g) as u32;
            let ib = (256. * b) as u32;

            println!("{} {} {}", ir, ig, ib)
        })
    })
}
