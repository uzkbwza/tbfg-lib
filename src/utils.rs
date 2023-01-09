use gdnative::prelude::*;
use fixed::prelude::*;
use fixed_trigonometry::*;
use super::point::*;

const PI: FixedNum = FixedNum::unwrapped_from_str("3.14159265358979323846264338");

pub type FixedNum = fixed::types::I48F16;

pub trait SimNum: ToFixed + Copy {}


impl SimNum for FixedNum {}
impl SimNum for i32 {}
impl SimNum for i64 {}

pub trait FixedExtra {
    fn powu(&self, pow: usize) -> FixedNum;
    fn sign(&self) -> FixedNum;
}

impl FixedExtra for FixedNum {
    fn powu(&self, pow: usize) -> FixedNum {
        let mut new = FixedNum::from_num(1);
        for _ in 0..pow {
            new *= *self;
        }
        new
    }

    fn sign(&self) -> FixedNum {
        if *self == FixedNum::from_num(0) {
            return FixedNum::from_num(0)
        }
        *self / self.abs()
    }
}

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct FixedMath {}

#[methods]
impl FixedMath {
    fn new(_base: &Reference) -> Self {
        Self {}
    }

    #[method]
    fn add(&self, x: String, y: String) -> String {
        let x_ = FixedNum::from_str(&x).unwrap();
        let y_ = FixedNum::from_str(&y).unwrap();
        (x_ + y_).to_string()
    }

    #[method]
    fn sub(&self, x: String, y: String) -> String {
        let x_ = FixedNum::from_str(&x).unwrap();
        let y_ = FixedNum::from_str(&y).unwrap();
        (x_ - y_).to_string()
    }

    #[method]
    fn div(&self, x: String, y: String) -> String {
        let x_ = FixedNum::from_str(&x).unwrap();
        let y_ = FixedNum::from_str(&y).unwrap();
        (x_ / y_).to_string()
    }

    #[method]
    fn mul(&self, x: String, y: String) -> String {
        let x_ = FixedNum::from_str(&x).unwrap();
        let y_ = FixedNum::from_str(&y).unwrap();
        (x_ * y_).to_string()
    }

    #[method]
    fn normalized_vec(&self, x: String, y: String) -> FixedVec2String {
        let vec = FixedVec2::from_string(x, y);
        vec.normalized().to_string()
    }
    
    #[method]
    fn normalized_vec_times(&self, x: String, y: String, magnitude: String) -> FixedVec2String {

        let vec = FixedVec2::from_string(x, y);
        let magnitude = FixedNum::unwrapped_from_str(&magnitude);

        (vec.normalized() * magnitude).to_string()
    }

    #[method]
    fn vec_mul(&self, x: String, y: String, rhs: String) -> FixedVec2String {
        let vec = FixedVec2::from_string(x, y);
        let rhs = FixedNum::from_str(&rhs).unwrap();
        (vec * rhs).to_string()
    }

    #[method]
    fn vec_div(&self, x: String, y: String, rhs: String) -> FixedVec2String {
        let vec = FixedVec2::from_string(x, y);
        let rhs = FixedNum::from_str(&rhs).unwrap();
        (vec / rhs).to_string()
    }

    #[method]
    fn vec_len(&self, x: String, y: String) -> String {
        let vec = FixedVec2::from_string(x, y);
        vec.length().to_string()
    }

    #[method]
    fn vec_to_angle(&self, x: String, y: String) -> String {
        let vec = FixedVec2::from_string(x, y);
        vec.angle().to_string()
    }

    #[method]
    fn angle_to_vec(&self, t: String) -> FixedVec2String {
        let t = FixedNum::unwrapped_from_str(&t);
        FixedVec2::coords(cos(t), sin(t)).to_string()
    }

    #[method]
    fn rotate_vec(&self, x: String, y: String, t: String) -> FixedVec2String {
        let x = FixedNum::unwrapped_from_str(&x);
        let y = FixedNum::unwrapped_from_str(&y);
        let t = FixedNum::unwrapped_from_str(&t);
        FixedVec2::coords(x * cos(t) - y * sin(t), x * sin(t) + y * cos(t)).to_string()
    }

    #[method]
    fn vec_dist(&self, x1: String, y1: String, x2: String, y2: String) -> String {
        (FixedVec2::from_string(x1, y1) - FixedVec2::from_string(x2, y2)).length().to_string()
    }

    #[method]
    fn deg2rad(&self, n: String) -> String {
        ((FixedNum::unwrapped_from_str(&n) * PI) / FixedNum::from_num(180)).to_string()
    }

    #[method]
    fn vec_add(&self, x1: String, y1: String, x2: String, y2: String) -> FixedVec2String {
        (FixedVec2::from_string(x1, y1) + FixedVec2::from_string(x2, y2)).to_string()
    }
    #[method]
    fn vec_sub(&self, x1: String, y1: String, x2: String, y2: String) -> FixedVec2String {
        (FixedVec2::from_string(x1, y1) - FixedVec2::from_string(x2, y2)).to_string()
    }


    #[method]
    fn vec_round(&self, x: String, y: String) -> FixedVec2Int {
        let vec = FixedVec2::from_string(x, y);
        vec.to_int()
    }

    #[method]
    fn lerp_int(&self, a: String, b: String, max: i32, f: i32) -> i32 {
        self.lerp_simnum(FixedNum::unwrapped_from_str(&a), FixedNum::unwrapped_from_str(&b), FixedNum::from_num(f) / FixedNum::from_num(max)).to_num()
    }



    #[method]
    fn lerp_string(&self, a: String, b: String, f: String) -> String {
        self.lerp_simnum(FixedNum::unwrapped_from_str(&a), FixedNum::unwrapped_from_str(&b), FixedNum::unwrapped_from_str(&f)).to_string()
    }

    #[method]
    fn lerp_angle(&self, from: String, to: String, weight: String) -> String {
        (FixedNum::unwrapped_from_str(&from) + self.short_angle_dist(FixedNum::unwrapped_from_str(&from), FixedNum::unwrapped_from_str(&to)) * FixedNum::unwrapped_from_str(&weight)).to_string()
    }


    #[method]
    fn angle_dist(&self, angle1: String, angle2: String) -> String {
        short_angle_dist(FixedNum::unwrapped_from_str(&angle1), FixedNum::unwrapped_from_str(&angle2))
    }

    fn short_angle_dist(&self, from: FixedNum, to: FixedNum) -> FixedNum {
        let max_angle = PI * 2;
        let difference = (to - from) % max_angle;
        ((2 * difference) % (max_angle)) - difference
    }

    #[method]
    fn sign(&self, a: String) -> i32 {
        FixedNum::unwrapped_from_str(&a).sign().to_num()
    }

    fn lerp_simnum<T: SimNum>(&self, a: T, b: T, f: FixedNum) -> FixedNum {
        FixedNum::from_num(a) * (FixedNum::from_num(1) - FixedNum::from_num(f)) + (FixedNum::from_num(b) *  FixedNum::from_num(f))
    }
    
    #[method]
    fn get_x_intercept(&self, x1: String, y1: String, x2: String, y2: String) -> String {
        let x1 = FixedNum::unwrapped_from_str(&x1);
        let y1 = FixedNum::unwrapped_from_str(&y1);
        let x2 = FixedNum::unwrapped_from_str(&x2);
        let y2 = FixedNum::unwrapped_from_str(&y2);
        if y2 == y1 || x2 == x1{
            return (FixedNum::from_num(i32::MAX) * (x2 - x1).sign()).to_string()
        }

        let m = (y2 - y1) / (x2 - x1);
        let y = FixedNum::from_num(y1);
        let x = FixedNum::from_num(x1);
        let b = x - (m * y);

        ((y - b) / m).to_string()
    }
    
    #[method]
    fn lt(&self, x: String, y: String) -> bool {
        FixedNum::unwrapped_from_str(&x) < FixedNum::unwrapped_from_str(&y)
    }

    #[method]
    fn gt(&self, x: String, y: String) -> bool {
        FixedNum::unwrapped_from_str(&x) > FixedNum::unwrapped_from_str(&y)
    }

    #[method]
    fn ge(&self, x: String, y: String) -> bool {
        FixedNum::unwrapped_from_str(&x) >= FixedNum::unwrapped_from_str(&y)
    }

    #[method]
    fn le(&self, x: String, y: String) -> bool {
        FixedNum::unwrapped_from_str(&x) <= FixedNum::unwrapped_from_str(&y)
    }

    #[method]
    fn eq(&self, x: String, y: String) -> bool {
        FixedNum::unwrapped_from_str(&x) == FixedNum::unwrapped_from_str(&y)
    }


    #[method]
    fn abs(&self, n: String) -> String {
        FixedNum::unwrapped_from_str(&n).abs().to_string()
    }

    // fn get_x_intercept_to_int(

    // #[method]
    // fn vec_scale_int(&self, x: String, y: String, max: i32, f: i32) -> FixedVec2String {
    //     (FixedVec2::from_string(x, y).normalized() 
    //         * self.lerp_simnum(
    //             FixedNum::from_num(0), 
    //             FixedNum::from_num(max), 
    //             FixedNum::from_num(f) / FixedNum::from_num(max)
    //         )
    //     ).to_string()
    // }

    #[method]
    fn powu(&self, n: String, e: i32) -> String {
        FixedNum::unwrapped_from_str(&n).powu(e as usize).to_string()
    }

    #[method]
    fn vec_scale(&self, x: String, y: String, f: String) -> FixedVec2Int {
        (FixedVec2::from_string(x, y)
            * self.lerp_simnum(
                0,
                1, 
                FixedNum::unwrapped_from_str(&f),
            )
        ).to_int()
    }
    #[method]
    fn int_map(&self, value: i32, istart: i32, istop: i32, ostart: i32, ostop: i32) -> i32 {
        let value = FixedNum::from_num(value);
        let istart = FixedNum::from_num(istart);
        let istop = FixedNum::from_num(istop);
        let ostart = FixedNum::from_num(ostart);
        let ostop = FixedNum::from_num(ostop);
        (ostart + (ostop - ostart) * ((value - istart) / (istop - istart))).to_num()
    }

    #[method]
    fn floor(&self, value: String) -> i32 {
        FixedNum::unwrapped_from_str(&value).floor().to_num()
    }

    #[method]
    fn round(&self, value: String) -> i32 {
        FixedNum::unwrapped_from_str(&value).round().to_num()
    }

    #[method]
    fn ceil(&self, value: String) -> i32 {
        FixedNum::unwrapped_from_str(&value).ceil().to_num()
    }
}

// #[derive(NativeClass)]
// #[inherit(Reference)]
// pub struct Box {

// }

// #[methods]
// impl Box {
//     fn new(_base: &Reference) -> Self {
//         Self {}
//     }
// }