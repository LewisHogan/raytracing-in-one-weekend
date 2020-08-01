use raytracing_in_one_weekend::ray::Ray;
use raytracing_in_one_weekend::vec3::Vec3;

type Color = Vec3;

fn main() {
    // We want to create a camera
    let aspect_ratio = 16.0 / 9.0;

    // Ensures that the final image height matches the aspect ratio to prevent
    // a stretched image.
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;

    // The viewport will be between -1 and 1 on the vertical axis,
    // with a normalized coordinate system scaled to the aspect ratio
    // on the horizontal axis
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);

    // Since the origin is the center, subtracting half the horziontal and vertical from
    // the origin will get us the bottom left, then we shift by the focal length.
    let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0, 0, focal_length);

    // We write the output to the stdout so a terminal user can redirect into a file or another
    // process.

    // Be careful doing this on powershell (non core versions) as they can mess with the data
    // instead of just outputting in ASCII or UTF-8

    // Write the file format header
    println!("P3\n{} {}\n255", width, height);

    for row in (0..height).rev() {
        // Print the progress to stderr, which means redirect operators won't capture it.
        // Note: Since we're using \r all of these eprints will appear on the same line
        eprint!("\rScanlines remaining: {}", row);
        for column in 0..width {
            // As we go along through the rendered image, we increment the u and v coordinates
            // to correspond to the location we would be in the final texture (if we were to
            // render to a texture or frame buffer directly)
            let u = (column as f64) / ((width - 1) as f64);

            // Since the row iterator starts reversed, it means that we'll start at the top
            // of the image instead of the bottom when rendering.
            let v = (row as f64) / ((height - 1) as f64);

            // Shoot a ray from the location of the camera (0, 0, 0)
            // moving from the top left to the bottom right of the image as time goes on
            let color: Color = ray_color(Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            ));

            write_color(color);
        }
    }

    eprintln!("\nDone.");
}

/// Given the parameters of a sphere, and a ray, returns
/// if the ray has collides with the sphere
///
/// Given the parameters of a sphere, works out how close the ray will come to the sphere
/// and if the ray ends up intersecting at any point, returns true.
///
/// * `center` - The center of the sphere
/// * `radius` - The radius of the sphere
/// * `ray` - The ray to check against the sphere
fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> bool {
    // We can tell if a point is in the sphere due to being able to know that
    // x*x + y*y + z*z <= R*R for all points inside or on a sphere
    //
    // The equation of a if a point is in or on the sphere
    // can be defined as (P-C)*(P-C) <= R * R, where C is the
    // center of the sphere and P is the point being checked
    //
    // The ray itself can be defined as a function P(t) = A + tb where A is the origin,
    // b is the direction and t is an unknown. By putting different t values into the function
    // we can get all the points the ray will ever hit
    //
    // As we're looking for a case where we are hitting the sphere, the equation we need to
    // solve is (P(t)-C) * (P(t)-C) = R*R
    //
    // Through complicated math I don't 100% understand you can expand the equation and move
    // all terms on the side to get a quadratic equation that can provide values for t.

    // if the sphere is not at the origin then we need to shift the point we compare against
    // by the sphere's offset
    let oc: Vec3 = ray.origin - center;
    // length squared is the same as x*x + y*y + z*z or doing the dot product with itself
    let a = ray.direction.length_squared();

    let b = 2.0 * oc.dot(ray.direction);
    //
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

/// Given a ray calculates a color to represent either the background
fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Vec3::new(0, 0, -1), 0.5, ray) {
        return Color::new(1, 0, 0);
    }

    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    // Lerp between white and blue based on the y component of the normalized vector
    (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1)
}

fn write_color(color: Color) {
    // Convert the color from 0-1 to 0-255
    let (ir, ig, ib) = (
        (color[0] * 255.99) as u8,
        (color[1] * 255.99) as u8,
        (color[2] * 255.99) as u8,
    );
    println!("{} {} {}", ir, ig, ib);
}
