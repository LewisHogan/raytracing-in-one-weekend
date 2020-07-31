fn main() {
    let (width, height) = (256, 256);

    // We write the output to the stdout so a terminal user can redirect into a file or another
    // process.

    // Be careful doing this on powershell (non core versions) as they can mess with the data
    // instead of just outputting in ASCII or UTF-8

    // Write the file format header
    println!("P3\n{} {}\n255", width, height);

    // Generates a test image with top green on the left, yellow on the top right,
    // dark blue on the bottom left and red on the bottom right
    for row in (0..height).rev() {
        // Print the progress to stderr, which means redirect operators won't capture it.
        // Note: Since we're using \r all of these eprints will appear on the same line
        eprint!("\rScanlines remaining: {}", row);
        for column in 0..width {
            // As we go along in the x axis, the amount of red being contributed increases
            // from 0 to 1.
            // The same change goes for green along the y axis, although as the loop for row is
            // reversed this means we write
            let r = (column as f64) / ((width - 1) as f64);
            let g = (row as f64) / ((height - 1) as f64);
            let b = 0.25;

            // Convert the pixels from 0-1 to 0-255
            let (ir, ig, ib) = ((r * 255.99) as u8, (g * 255.99) as u8, (b * 255.99) as u8);

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
