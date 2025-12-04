use std::f32::consts::PI;

use crate::core::Vector3;

pub fn exercise2() {
    // Ex 2.1
    // [2 -2 -2]
    // Decompose the vector into its magnitude and direction
    let vec2_1: Vector3 = Vector3::new(2.0, -2.0, -2.0);
    let magnitude2_1: f32 = vec2_1.magnitude();
    println!("Magnitude2_1: {}", magnitude2_1);
    // let direction1_x = ();

    // 2.2
    // (a) calculate the scalar product vec(3 1 2) * vec(0 2 -1)
    let scalar_product2: f32 = 3.0 * 0.0 + 1.0 * 2.0 + 2.0 * -1.0;
    println!("Scalar Product 2: {}", scalar_product2); // 0
    // (b) what does the result of (a) tell about the angle?
    // since it is 0, that means the vectors are ether 90 degress or 270 degrees, right angles

    // 2.3 Calcuate the scalar product of a vector with itself
    let scalar_product_of_2_1_with_self: f32 = 2.0 * 2.0 + -2.0 * -2.0 + -2.0 * -2.0;
    println!("Scalar Product with Self: {}", scalar_product_of_2_1_with_self); // 12
    // (a) Which other method corresponds to this value?
    // This corrosponds to the sqaure_magnitude method
    println!("Sqaure Magnitude of vec2_1: {}", vec2_1.square_magnitude()); // 12
    // (b) is it more or less efficient to calculate this value using the scalar product?
    // It would be more efficient to use the square_magnitude method because it avoids the redundant step of
    // calculating the scalar product

    // 2.4 Assume following vector
    // [ 1 2 3 ]
    // and another containting unknown, x, [ 7 -2 x ]
    // If we know they are perpendicular, what is x
    // Use the dot product
    // 1 * 7 + 2 * -2 + 3 * x
    // -> 7 - 4 + 3x -> 3 + 3x -> 3 = -3x -> x = -1

    // 2.5
    // (a) use scalar product to find angle between [ 0 1 1 ] [ 0 -1 0 ]
    // |a X b| = |a||b|sin(theta)
    // => |a||b|sqrt( 1 - (a * b)^2 )
    let a5: Vector3 = Vector3::new(0.0, 1.0, 1.0);
    let b5: Vector3 = Vector3::new(0.0, -1.0, 0.0);
    let scalar_product5: f32 = a5 * &b5; // -1
    let magnitude5a: f32 = a5.magnitude();
    let magnitude5b: f32 = b5.magnitude();
    // degrees = radians * 180 / PI
    let angle_5_a: f32 = (scalar_product5 / (magnitude5a * magnitude5b)).acos() * 180.0 / PI;
    println!("5a Angle: {}", angle_5_a);
    // (b) calculate the angle using the vector product
    // x: 1 * 0 - 1 * -1 -> 1
    // y: 1 * 0 - 0 * 0 -> 0
    // z: 0 * -1 - 1 * 0 -> 0
    let angle_5: f32 = ( a5.vector_product(&b5).magnitude() / ( a5.magnitude() * b5.magnitude() )).asin() * 180.0 / PI;
    println!("Vector Product Angle: {}", angle_5);

    // 2.6 Assume following vectors
    // a = 1 / sqrt(2) [ 0 1 1 ]
    // b = [ 1 2 3 ]
    // (a) calculate the scalar product c = ^a . b
    let a6: Vector3 = Vector3::new( (1.0 / (f32::sqrt(2.0))) * 0.0, (1.0 / (f32::sqrt(2.0))) * 1.0, (1.0 / (f32::sqrt(2.0)) * 1.0) );
    let b6: Vector3 = Vector3::new(1.0, 2.0, 3.0);
    let scalar_product2_6: f32 = a6 * &b6;
    println!("2.6: a. Scalar Product: {}", scalar_product2_6);
    // (b) calculate the value of vector d where d = b - c(^a)
    let ca6: Vector3 = a6 * scalar_product2_6;
    let d_2_6: Vector3 = b6 - &ca6;
    println!("d_2_6: {:?}", d_2_6);
    // (c) what is the angle between vectors ^a and d? Gemoetrically, what have we done to get the results?
    let angle_6: f32 = (a6 * &d_2_6) / (a6.magnitude() * d_2_6.magnitude()).acos() * 180.0 / PI;
    println!("Angle between a6 and d_2_6: {}", angle_6);
    
    // 2.7 If vector starts at [1 2 3] and changes with velocity [1 -1 2] per second what will it be after 10 seconds
    let mut start_2_7: Vector3 = Vector3::new(1.0, 2.0, 3.0);
    let velocity_2_7: Vector3 = Vector3::new(1.0, -1.0, 2.0);
    let delta_time: u32 = 10;
    
    for _i in 0..delta_time {
        start_2_7.x = start_2_7.x * velocity_2_7.x;
        start_2_7.y = start_2_7.y * velocity_2_7.y;
        start_2_7.z = start_2_7.z * velocity_2_7.z;
        println!("Steps: {:?}", start_2_7);
    }
    println!("End 2_7: {:?}", start_2_7);
}