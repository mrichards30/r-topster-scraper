#[cfg(test)]
mod tests {
    use image::GenericImageView;
    use crate::{convolve, get_greyscale_value, image_to_matrix};

    #[test]
    fn get_greyscale_value_has_the_correct_coefficients() {
        assert_eq!(0.299, get_greyscale_value([1, 0, 0]));
        assert_eq!(0.587, get_greyscale_value([0, 1, 0]));
        assert_eq!(0.114, get_greyscale_value([0, 0, 1]));
    }

    #[test]
    fn the_image_matrix_keeps_the_same_dimensions() {
        let image = image::DynamicImage::new_luma8(16, 32);
        let matrix = image_to_matrix(&image);
        assert_eq!(image.dimensions(), (matrix.len() as u32, matrix[0].len() as u32));
    }

    #[test]
    fn convolving_a_matrix_with_a_kernel_retains_the_matrices_dimensions() {
        let matrix = vec![vec![1_f32, 2_f32, 3_f32]];
        let kernel = [
            [1_f32, 2_f32, 3_f32],
            [1_f32, 2_f32, 3_f32],
            [1_f32, 2_f32, 3_f32]
        ];
        let convolution_result = convolve(&matrix, kernel);
        assert_eq!((matrix.len(), matrix[0].len()), (convolution_result.len(), convolution_result[0].len()));
    }

}
