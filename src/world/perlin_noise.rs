// Modified from https://mrl.cs.nyu.edu/~perlin/noise and https://adrianb.io/2014/08/09/perlinnoise.html.

use crate::utils::resolve_optional_val;

/// Generates Perlin noise for a 3D point, using octaves (fractal generation) to add more detail.
/// The results are approximately(?) evenly distributed around 0. (During testing, I observed a
/// difference of only ~0.5% more results below 0, which is close enough for my needs regardless.)
///
/// # Arguments
///
/// * `x` - The x-coordinate of the point.
/// * `y` - The y-coordinate of the point.
/// * `z` - The z-coordinate of the point.
/// * `scale` - How much the noise map is stretched out. Larger scale is more stretching.
///             Be cautious with low scale values, as the noise value for all-integer
///             coordinate sets is always 0. Defaults to 100.
/// * `octaves` - The number of detail levels used. More octaves is more detail. Defaults to 4.
///
/// # Returns
///
/// A `f64` value representing the Perlin noise at the given point, approximately in the range of [-1, 1].
pub fn octaved_noise(x: f64, y: f64, z: f64, scale: Option<f64>, octaves: Option<i32>) -> f64 {
    let scale = resolve_optional_val(scale, 100.0);
    let octaves = resolve_optional_val(octaves, 4);

    let mut total = 0.0;
    let mut octave_frequency = 1.0;
    let mut octave_amplitude = 1.0;
    let mut amplitudes_sum = 0.0;
    for _ in 0..octaves {
        total += noise(
            x / scale * octave_frequency,
            y / scale * octave_frequency,
            z / scale * octave_frequency,
        ) * octave_amplitude;

        amplitudes_sum += octave_amplitude;

        // Decrease the amplitude and increase the frequency for the next octave.
        // This results in the next octave being more detailed and less influential.
        octave_amplitude *= 0.5;
        octave_frequency *= 2.0;
    }

    // Normalize the result to approximately [-1, 1].
    total / amplitudes_sum
}

#[allow(clippy::many_single_char_names)]
fn noise(x: f64, y: f64, z: f64) -> f64 {
    let x1 = x.floor() as i32 & 255; // FIND UNIT CUBE THAT CONTAINS POINT.
    let y1 = y.floor() as i32 & 255;
    let z1 = z.floor() as i32 & 255;
    let rx = x - x.floor(); // FIND RELATIVE X,Y,Z OF POINT IN CUBE.
    let ry = y - y.floor();
    let rz = z - z.floor();
    let u = fade(rx); // COMPUTE FADE CURVES FOR EACH OF X,Y,Z.
    let v = fade(ry);
    let w = fade(rz);
    let a = P[x1 as usize] + y1; // HASH COORDINATES OF THE 8 CUBE CORNERS...
    let aa = P[a as usize] + z1;
    let ab = P[(a + 1) as usize] + z1;
    let b = P[(x1 + 1) as usize] + y1;
    let ba = P[b as usize] + z1;
    let bb = P[(b + 1) as usize] + z1;

    // AND ADD BLENDED RESULTS FROM 8 CORNERS OF CUBE
    lerp(
        w,
        lerp(
            v,
            lerp(
                u,
                grad(P[aa as usize], rx, ry, rz),
                grad(P[ba as usize], rx - 1.0, ry, rz),
            ),
            lerp(
                u,
                grad(P[ab as usize], rx, ry - 1.0, rz),
                grad(P[bb as usize], rx - 1.0, ry - 1.0, rz),
            ),
        ),
        lerp(
            v,
            lerp(
                u,
                grad(P[(aa + 1) as usize], rx, ry, rz - 1.0),
                grad(P[(ba + 1) as usize], rx - 1.0, ry, rz - 1.0),
            ),
            lerp(
                u,
                grad(P[(ab + 1) as usize], rx, ry - 1.0, rz - 1.0),
                grad(P[(bb + 1) as usize], rx - 1.0, ry - 1.0, rz - 1.0),
            ),
        ),
    )
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

#[allow(clippy::many_single_char_names)]
fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15; // CONVERT LOW 4 BITS OF HASH CODE INTO 12 GRADIENT DIRECTIONS.
    let u = if h < 8 { x } else { y };
    let v = match h {
        0..=3 => y,
        12 | 14 => x,
        _ => z,
    };
    let signed_u = if (h & 1) == 0 { u } else { -u };
    let signed_v = if (h & 2) == 0 { v } else { -v };
    signed_u + signed_v
}

const PERMUTATION: [i32; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

// Double the permutation table to avoid overflow
const P: [i32; 512] = {
    let mut p = [0; 512];
    let mut i = 0;

    // We have to use a while loop instead of a for loop here since this is within a const block
    while i < PERMUTATION.len() {
        p[i] = PERMUTATION[i];
        p[i + 256] = PERMUTATION[i];

        i += 1;
    }

    p
};

// using System;
// using System.Linq;
//
// // Modified from https://mrl.cs.nyu.edu/~perlin/noise and https://adrianb.io/2014/08/09/perlinnoise.html.
// public static class PerlinNoise
// {
// /*
//  * Generates Perlin noise for a 3D point, using octaves (fractal generation) to add more detail. The results are
//  * approximately(?) evenly distributed around 0. (During testing, I observed a difference of only ~0.5% more
//  * results below 0, which is close enough for my needs regardless.)
//  * Scale is how much the noise map is stretched out. Larger scale is more stretching. Be cautious with low scale
//  * values, as the noise value for all-integer coordinate sets is always 0.
//  * Octaves is the number of detail levels used. More octaves is more detail.
//  */
// public static double OctavedNoise(double x, double y, double z, int scale = 100, int octaves = 4)
// {
// double total = 0;
// double octaveFrequency = 1;
// double octaveAmplitude = 1;
// double amplitudesSum = 0;
// for (int i = 0; i < octaves; i++)
// {
// total += Noise(x / scale * octaveFrequency, y / scale * octaveFrequency, z / scale * octaveFrequency) *
// octaveAmplitude;
//
// amplitudesSum += octaveAmplitude;
//
// // Decrease the amplitude and increase the frequency for the next octave.
// // This results in the next octave being more detailed and less influential.
// octaveAmplitude *= 0.5;
// octaveFrequency *= 2;
// }
//
// // Normalize result to approximately [-1, 1]
// return total / amplitudesSum;
// }
//
// private static double Noise(double x, double y, double z)
// {
// int x1 = (int)Math.Floor(x) & 255; // FIND UNIT CUBE THAT CONTAINS POINT.
// int y1 = (int)Math.Floor(y) & 255;
// int z1 = (int)Math.Floor(z) & 255;
// x -= Math.Floor(x); // FIND RELATIVE X,Y,Z OF POINT IN CUBE.
// y -= Math.Floor(y);
// z -= Math.Floor(z);
// var u = Fade(x); // COMPUTE FADE CURVES FOR EACH OF X,Y,Z.
// var v = Fade(y);
// var w = Fade(z);
// var a = P[x1] + y1; // HASH COORDINATES OF THE 8 CUBE CORNERS...
// var aa = P[a] + z1;
// var ab = P[a + 1] + z1;
// var b = P[x1 + 1] + y1;
// var ba = P[b] + z1;
// var bb = P[b + 1] + z1;
//
// // AND ADD BLENDED RESULTS FROM 8 CORNERS OF CUBE
// return Lerp(w,
// Lerp(v, Lerp(u, Grad(P[aa], x, y, z), Grad(P[ba], x - 1, y, z)),
// Lerp(u, Grad(P[ab], x, y - 1, z), Grad(P[bb], x - 1, y - 1, z))),
// Lerp(v, Lerp(u, Grad(P[aa + 1], x, y, z - 1), Grad(P[ba + 1], x - 1, y, z - 1)),
// Lerp(u, Grad(P[ab + 1], x, y - 1, z - 1), Grad(P[bb + 1], x - 1, y - 1, z - 1))));
// }
//
// private static double Fade(double t)
// {
// return t * t * t * (t * (t * 6 - 15) + 10);
// }
//
// private static double Lerp(double t, double a, double b)
// {
// return a + t * (b - a);
// }
//
// private static double Grad(int hash, double x, double y, double z)
// {
// var h = hash & 15; // CONVERT LOW 4 BITS OF HASH CODE INTO 12 GRADIENT DIRECTIONS.
// double u = h < 8 ? x : y, v = h < 4 ? y : h == 12 || h == 14 ? x : z;
// return ((h & 1) == 0 ? u : -u) + ((h & 2) == 0 ? v : -v);
// }
//
// private static readonly int[] Permutation =
// {
// 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37,
// 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177,
// 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146,
// 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
// 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100,
// 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
// 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153,
// 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246,
// 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192,
// 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114,
// 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180
// };
//
// private static readonly int[] P = Permutation.Concat(Permutation).ToArray();
// }
