use std::cmp;
use image::{DynamicImage, GenericImageView, GrayImage, Pixel};
use num_traits::{Pow};
pub mod tests;

const IMAGE_NAME: &str = "img/topster.png";

const SOBEL_X: [[f32; 3]; 3] = [
    [1_f32, 2_f32, 1_f32],
    [0_f32, 0_f32, 0_f32],
    [-1_f32, -2_f32, -1_f32]
];

const SOBEL_Y: [[f32; 3]; 3] = [
    [1_f32, 0_f32, -1_f32],
    [2_f32, 0_f32, -2_f32],
    [1_f32, 0_f32, -1_f32]
];

const GAUSSIAN: [[f32; 3]; 3] = [
    [0.0625, 0.125, 0.0625],
    [0.125, 0.25, 0.125],
    [0.0625, 0.125, 0.0625]
];

fn main() {
    // Image preprocessing
    let image = image::open(IMAGE_NAME).unwrap();
    let matrix = rotate_90_degrees(&image_to_matrix(&image));
    let _ = matrix_to_grayimage(&matrix).save("test.png");

    let blurred_matrix = convolve(&matrix, GAUSSIAN);
    let sobel_x_matrix = convolve(&blurred_matrix, SOBEL_X);
    let sobel_y_matrix = convolve(&matrix, SOBEL_Y);

    // Split the image into its columns
    let derivative_averages: Vec<f32> = sobel_x_matrix.iter().map(|v| v.iter().copied().sum::<f32>() / (v.len() as f32)).collect();
    let mut col_sections = vec![];
    let mut start: i32 = -1;
    for i in 1_i32..derivative_averages.len() as i32 {
        if start != -1 && derivative_averages[i as usize] == 0_f32 {
            col_sections.push(Vec::from(&matrix[(start as usize)..(i as usize)]));
            start = -1;
        } else if start == -1 && derivative_averages[i as usize] != 0_f32 {
            start = i;
        }
    }

    // Work out length of the albums on each row
    let rotated_matrix = convolve(&rotate_90_degrees(&matrix), SOBEL_X);
    let derivative_averages: Vec<f32> = rotated_matrix.iter().map(|v| v.iter().copied().sum::<f32>() / (v.len() as f32)).collect();
    let mut row_lengths = vec![];
    let mut start: i32 = -1;
    for i in 1_i32..derivative_averages.len() as i32 {
        if start != -1 && derivative_averages[i as usize] == 0_f32 {
            row_lengths.push((start, i));
            start = -1;
        } else if start == -1 && derivative_averages[i as usize] != 0_f32 {
            start = i;
        }
    }
    
    // Split each column into its individual albums -- assuming square album covers
    for (i, section) in col_sections.iter().enumerate() {
        let rotated_section = rotate_90_degrees(&section);
        for (start, end) in &row_lengths {
            let album_matrix = rotate_90_degrees(&rotate_90_degrees(&Vec::from(&rotated_section[*start as usize..*end as usize])));
            let _ = matrix_to_grayimage(&album_matrix).save(format!("out/topster{}-{}-{}.png", start, end, i));
        }
    }

}

#[allow(dead_code)] //TODO remove this function once something is working...
fn matrix_to_grayimage(matrix: &Vec<Vec<f32>>) -> GrayImage {
    let mut image = GrayImage::new(matrix.len() as u32, matrix[0].len() as u32);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Luma([matrix[x as usize][y as usize] as u8]);
    }
    return image;
}

fn rotate_90_degrees(matrix: &Vec<Vec<f32>>, n: usize) -> Vec<Vec<f32>> {
    let mut rotated_matrix: Vec<Vec<f32>> = vec![vec![0_f32; matrix.len()]; matrix[0].len()];
    for i in 0..rotated_matrix.len() {
        for j in 0..rotated_matrix[i].len() {
            rotated_matrix[i][j] = matrix[j][i];
        }
    }
    return rotated_matrix;
}

fn get_greyscale_value(rgb: [u8; 3]) -> f32 {
    let [r, g, b] = rgb;
    return r as f32 * 0.299_f32 + g as f32 * 0.587_f32 + b as f32 * 0.114_f32;
}

fn image_to_matrix(image: &DynamicImage) -> Vec<Vec<f32>> {
    let mut matrix: Vec<Vec<f32>> = vec![vec![0_f32; image.dimensions().1 as usize]; image.dimensions().0 as usize];
    for i in 0..image.dimensions().0 {
        for j in 0..image.dimensions().1 {
            matrix[i as usize][j as usize] = get_greyscale_value(image.get_pixel(i, j).to_rgb().0);
        }
    }
    return matrix;
}

fn convolve(matrix: &Vec<Vec<f32>>, kernel: [[f32; 3]; 3]) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![vec![0_f32; matrix[0].len()]; matrix.len()];
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            for u in 0..kernel.len() {
                for v in 0..kernel[0].len() {
                    let mut image_val: f32 = 0_f32;
                    let x_minus_u: i32 = x as i32 - u as i32;
                    let y_minus_v: i32 = y as i32 - v as i32;
                    if matrix.len() as i32 > x_minus_u &&
                            x_minus_u >= 0 &&
                            matrix[x_minus_u as usize].len() as i32 > y_minus_v &&
                            0 <= y_minus_v {
                        image_val = matrix[x_minus_u as usize][y_minus_v as usize];
                    }
                    new_matrix[x][y] = new_matrix[x][y] + kernel[u][v] * image_val;
                }
            }
        }
    }
    return new_matrix;
}
