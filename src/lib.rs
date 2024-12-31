pub mod render_math {
    pub mod vector {
        use super::matrix::Mat4;
        use std::ops::{Mul, Add, Sub};

        pub trait Magnitude {
            fn magnitude(&self) -> f64;
        }

        #[derive(Debug, Copy, Clone)]
        pub struct Vector2i {
            x: i32,
            y: i32,
        }

        #[derive(Debug, Copy, Clone)]
        pub struct Vector3 {
            x: f64,
            y: f64,
            z: f64,
        }

        #[derive(Debug, Copy, Clone)]
        pub struct Vector4 {
            x: f64,
            y: f64,
            z: f64,
            w: f64,
        }

        impl Vector2i {
            pub fn new(x: i32, y: i32) -> Self {
                Vector2i {
                    x,
                    y,
                }
            }

            pub fn x(&self) -> i32 {
                self.x
            }
            pub fn y(&self) -> i32 {
                self.y
            }

            pub fn mut_x(&mut self) -> &mut i32 {
                &mut self.x
            }
            pub fn mut_y(&mut self) -> &mut i32 {
                &mut self.y
            }
        }

        impl Add for Vector2i {
            type Output = Vector2i;

            fn add(self, rhs: Self) -> Self::Output {
                Vector2i {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y
                }
            }
        }

        impl Sub for Vector2i {
            type Output = Vector2i;

            fn sub(self, rhs: Self) -> Self::Output {
                Vector2i {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y
                }
            }
        }

        impl Mul for Vector2i {
            type Output = Vector2i;

            fn mul(self, rhs: Self) -> Self::Output {
                Vector2i {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y
                }
            }
        }

        impl Vector3 {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Vector3 {
                    x,
                    y,
                    z,
                }
            }

            pub fn from(vec: &Vector4) -> Self {
                Vector3 {
                    x: vec.x,
                    y: vec.y,
                    z: vec.z
                }
            }

            pub fn x(&self) -> f64 {
                self.x
            }
            pub fn y(&self) -> f64 {
                self.y
            }
            pub fn z(&self) -> f64 {
                self.z
            }

            pub fn mut_x(&mut self) -> &mut f64 {
                &mut self.x
            }
            pub fn mut_y(&mut self) -> &mut f64 {
                &mut self.y
            }
            pub fn mut_z(&mut self) -> &mut f64 {
                &mut self.z
            }
        }

        impl Add for Vector3 {
            type Output = Vector3;
            
            fn add(self, rhs: Self) -> Self::Output {
                Vector3 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl Sub for Vector3 {
            type Output = Vector3;

            fn sub(self, rhs: Self) -> Self::Output {
                Vector3 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z
                }
            }
        }

        impl Mul for Vector3 {
            type Output = Vector3;

            fn mul(self, rhs: Self) -> Self::Output {
                Vector3 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.y * rhs.y
                }
            }
        }

        impl Mul<f64> for Vector3 {
            type Output = Vector3;

            fn mul(self, rhs: f64) -> Self::Output {
                Vector3 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs
                }
            }
        }
        
        impl Vector4 {
            pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
                Vector4 {
                    x,
                    y,
                    z,
                    w,
                }
            }

            pub fn from(vec: &Vector3) -> Self {
                Vector4 {
                    x: vec.x,
                    y: vec.y,
                    z: vec.z,
                    w: 1.0_f64
                }
            }

            pub fn x(&self) -> f64 {
                self.x
            }
            pub fn y(&self) -> f64 {
                self.y
            }
            pub fn z(&self) -> f64 {
                self.z
            }
            pub fn w(&self) -> f64 {
                self.w
            }
        }

        impl Mul<&Mat4> for Vector4 {
            type Output = Vector4;

            fn mul(self, rhs: &Mat4) -> Self::Output {
                Vector4::new(
                    self.x * rhs.values()[0][0] + self.y * rhs.values()[0][1] + self.z * rhs.values()[0][2] + self.w * rhs.values()[0][3],
                    self.x * rhs.values()[1][0] + self.y * rhs.values()[1][1] + self.z * rhs.values()[1][2] + self.w * rhs.values()[1][3],
                    self.x * rhs.values()[2][0] + self.y * rhs.values()[2][1] + self.z * rhs.values()[2][2] + self.w * rhs.values()[2][3],
                    self.x * rhs.values()[3][0] + self.y * rhs.values()[3][1] + self.z * rhs.values()[3][2] + self.w * rhs.values()[3][3],
                )
            }
        }

        impl Add for Vector4 {
            type Output = Vector4;

            fn add(self, rhs: Self) -> Self::Output {
                Vector4 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w
                }
            }
        }

        impl Sub for Vector4 {
            type Output = Vector4;

            fn sub(self, rhs: Self) -> Self::Output {
                Vector4 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w
                }
            }
        }

        impl Mul for Vector4 {
            type Output = Vector4;

            fn mul(self, rhs: Self) -> Self::Output {
                Vector4 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                    w: self.w * rhs.w
                }
            }
        }

        impl Magnitude for Vector2i {
            fn magnitude(&self) -> f64 {
                f64::from(self.x * self.x + self.y * self.y).sqrt()
            }
        }

        impl Magnitude for Vector3 {
            fn magnitude(&self) -> f64 {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }
        }

        impl Magnitude for Vector4 {
            fn magnitude(&self) -> f64 {
                (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
            }
        }

        pub struct Mesh {
            verts: Vec<Vector3>,
    
            translation_matrix: Mat4,
            scale_matrix: Mat4,
            rotation_matrix: Mat4,
        }
    
        impl Mesh {
            pub fn new() -> Self {
                Mesh {
                    verts: Vec::new(),
    
                    translation_matrix: Mat4::identity(),
                    scale_matrix: Mat4::identity(),
                    rotation_matrix: Mat4::identity() 
                }
            }

            pub fn set_translation_matrix(&mut self, mat: Mat4) {
                self.translation_matrix = mat;
            }

            pub fn set_rotation_matrix(&mut self, mat: Mat4) {
                self.rotation_matrix = mat;
            }

            pub fn set_scale_matrix(&mut self, mat: Mat4) {
                self.scale_matrix = mat;
            }
            
            pub fn add_vertex(&mut self, point: Vector3) {
                self.verts.push(point);
            }
    
            pub fn get_transformed_verts(&self) -> Vec<Vector3> {
                let mut v = Vec::new();
    
                for vertice in self.verts.iter() {
                    v.push(
                        Vector3::from(
                            &(Vector4::from(vertice) * &self.scale_matrix * &self.rotation_matrix * &self.translation_matrix)
                        )
                    );
                }
    
                v
            }
        }
    }

    pub mod matrix {
        use crate::render_math::vector::Vector3;
        use std::ops::Mul;

        #[derive(Debug)]
        pub struct Mat4 {
            values: [[f64;4]; 4],
        }

        impl Mat4 {
            pub fn values(&self) -> [[f64;4]; 4] {
                self.values
            }

            pub fn mutable_values(&mut self) -> &mut [[f64;4]; 4] {
                &mut self.values
            }

            pub fn new(values: [[f64;4]; 4]) -> Self {
                Mat4 {
                    values
                }
            }

            pub fn empty() -> Self {
                Mat4 {
                    values: [[0.0;4];4]
                }
            }

            pub fn identity() -> Self {
                Mat4 {
                    values: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ]
                }
            }

            pub fn scale(s: Vector3) -> Self {
                Mat4 {
                    values: [
                        [s.x(), 0.0, 0.0, 0.0],
                        [0.0, s.y(), 0.0, 0.0],
                        [0.0, 0.0, s.z(), 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ]
                }
            }

            pub fn translation(t: Vector3) -> Self {
                Mat4 {
                    values: [
                        [1.0, 0.0, 0.0, t.x()],
                        [0.0, 1.0, 0.0, t.y()],
                        [0.0, 0.0, 1.0, t.z()],
                        [0.0, 0.0, 0.0, 1.0]
                    ]
                }
            }

            pub fn euler_rotation(euler: Vector3) -> Self {
                let x_mat = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, euler.x().cos(), -euler.x().sin(), 0.0],
                    [0.0, euler.x().sin(), euler.x().cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];

                let y_mat = [
                    [euler.y().cos(), 0.0, euler.y().sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-euler.y().sin(), 0.0, euler.y().cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];

                let z_mat = [
                    [euler.z().cos(), -euler.z().sin(), 0.0, 0.0],
                    [euler.z().sin(), euler.z().cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ];

                let x_mat = Mat4::new(x_mat);
                let y_mat = Mat4::new(y_mat);
                let z_mat = Mat4::new(z_mat);

                x_mat * y_mat * z_mat
            }
        }

        impl Mul for Mat4 {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut result: [[f64;4];4] = [[0.0;4];4];

                for row in 0..4 {
                    for column in 0..4 {
                        result[row][column] = 
                            self.values[row][0] * rhs.values[0][column] +
                            self.values[row][1] * rhs.values[1][column] +
                            self.values[row][2] * rhs.values[2][column] +
                            self.values[row][3] * rhs.values[3][column];
                    }
                }

                Mat4::new(result)
            }
        }
    }
}


pub mod render {
    pub struct Screen<T: Default> {
        pixels: Vec<T>,
        width: u32,
        height: u32,
    }

    impl<T: Default> Screen<T> {
        pub fn new(x: u32, y: u32) -> Self {
            let mut screen = Screen {
                pixels: Vec::new(),
                width: x,
                height: y,
            };
            screen.clear();
            screen
        }

        pub fn resize(&mut self, x: u32, y: u32) {
            self.width = x;
            self.height = y;
            self.clear();
        }

        pub fn clear(&mut self) {
            let total = self.height * self.width;
            self.pixels.clear();
            for _ in 0..total {
                self.pixels.push(T::default());
            }
        }
        
        pub fn set_pixel(&mut self, x: u32, y: u32, pixel: T) -> Result<(), &'static str> {
            let i: usize = match (y * self.width + x).try_into() {
                Ok(v) => v,
                Err(_) => return Err("Desired pixel index does not fit into usize.")
            };
            if let Some(px) = self.pixels.get_mut(i) {
                *px = pixel;
            }
            else {
                return Err("Index out of bounds.");
            }
            Ok(())
        }

        pub fn push_pixels(&mut self, x: u32, y: u32, pixels: Vec<T>) -> Result<(), &'static str> {
            let i: usize = match (y * self.width + x).try_into() {
                Ok(v) => v,
                Err(_) => return Err("Desired pixel index does not fit into usize.")
            };
            
            for pixel in pixels {
                if let Some(px) = self.pixels.get_mut(i) {
                    *px = pixel;
                }
            }

            Ok(())
        }

        pub fn get_pixel(&self, x: u32, y: u32) -> Option<&T> { 
            let i: usize = match (y * self.width + x).try_into() {
                Ok(v) => v,
                Err(_) => return None,
            };
            self.pixels.get(i) 
        }

        pub fn get_buffer(&self) -> &Vec<T> {
            &self.pixels
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

    }

}

// TODO: Try to make renderers use single Renderer trait for ease of use as interchangable renderers.
// TODO: Reduce code repitition between renderers.

pub mod pixel {
    use crate::{render::Screen, render_math::vector::*};
    use image_helper::image::{ImageData, PixelData};
    use std::path::Path;

    pub struct PixelRenderer {
        screen: Screen<PixelData>,

        z_buffer: Vec<((u32, u32), f64)>,
        frame_num: u32,

        z_min: f64,
        z_max: f64,
    }

    impl PixelRenderer {
        pub fn new(w: u32, h: u32) -> Self {
            PixelRenderer {
                screen: Screen::<PixelData>::new(w, h),

                z_buffer: Vec::new(),
                
                frame_num: 0,
                z_min: 0.0,
                z_max: 0.0,
            }
        }

        pub fn draw_at(&mut self, x: u32, y: u32, pixel: PixelData, z: f64) -> Result<(), &'static str> {
            // Ignore pixel if something above it already exists in z buffer
            for v in self.z_buffer.iter() {
                if v.0.0 == x && v.0.1 == y && v.1 < z {
                    return Ok(());
                }
            }

            self.screen.set_pixel(x, y, pixel)?;

            self.z_buffer.push(((x, y), z));

            if (self.z_min > z) {
                self.z_min = z;
            }
            if (self.z_max < z) {
                self.z_max = z;
            }

            Ok(())
        }

        pub fn get_screen_buffer(&self) -> &Vec<PixelData> {
            self.screen.get_buffer()
        }

        pub fn flush(&mut self) {
            let _ = std::fs::create_dir("outputs/");
            let file = std::fs::File::create(Path::new(format!("outputs/output{}.png", self.frame_num).as_str())).unwrap();
            let ref mut w = std::io::BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, self.screen.width(), self.screen.height());
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            
            let veclen: usize = (self.screen.width() * self.screen.height() * 4).try_into().unwrap();
            let mut data: Vec<u8> = Vec::with_capacity(veclen);
            for (i, pixel) in self.screen.get_buffer().iter().enumerate() {
                /*data[i * 4] = pixel.r() as u8;
                data[i * 4 + 1] = pixel.g() as u8;
                data[i * 4 + 2] = pixel.b() as u8;
                data[i * 4 + 3] = pixel.a() as u8;*/

                data.push(pixel.r() as u8);
                data.push(pixel.g() as u8);
                data.push(pixel.b() as u8);
                data.push(pixel.a() as u8);
            }

            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();

            println!("Outputted {} frame", self.frame_num);

            // Cleanup
            self.screen.clear();
            self.z_buffer.clear();
            self.z_min = 0.0;
            self.z_max = 0.0;
            self.frame_num += 1;
        }

        pub fn draw_image_2d(&mut self, image: &ImageData, position: Vector2i, z: f64) -> Result<(), &'static str> {
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let pixel = image.get_pixel_at(x, y);

                    if pixel.r() != 0 {
                        let new_pos = position + Vector2i::new(x as i32, y as i32);
                        self.draw_at(new_pos.x() as u32, new_pos.y() as u32, PixelData::new(png::BitDepth::Eight, 255, 255, 0, 255), z)?;
                    }
                }
            }

            Ok(())
        }

        pub fn rasterize_vertices(&mut self, mesh: &Mesh, max_distance: f64) -> Result<(), &'static str> {
            let verts = mesh.get_transformed_verts();

            for x in 0..self.screen.width() {
                for y in 0..self.screen.height() {
                    for vert in verts.iter() {
                        let pixel_pos_3d = Vector3::new(f64::from(x), f64::from(y), 0.0);
                        let orthographic_projection_vector = Vector3::new(vert.x(), vert.y(), 0.0);
                        if (orthographic_projection_vector - pixel_pos_3d).magnitude() <= max_distance {
                            let multiplier = (1.0 / (self.z_min - self.z_max)) * (vert.z() - self.z_max);
                            let c = PixelData::new(png::BitDepth::Eight, (239.0 * multiplier) as usize, (100.0 * multiplier) as usize, (232.0 * multiplier) as usize, 255);

                            self.draw_at(x, y, c, vert.z())?;
                        }
                    }
                }
            }

            Ok(())
        }
    }
}

pub mod ansi {
    use std::{collections::HashMap, error::Error, string::FromUtf8Error};

    use crate::{render::Screen, render_math::vector::*};
    use image_helper::image::*;

    pub struct ANSIRenderer {
        helper: ANSIHelper,

        currently_set_pixels: Vec<(u32, u32)>,
        previously_set_pixels: Vec<(u32, u32)>,

        z_buffer: Vec<((u32, u32), f64)>,
    }

    impl ANSIRenderer {
        pub fn new() -> Self {
            ANSIRenderer {
                helper: ANSIHelper::new(),

                currently_set_pixels: Vec::new(),
                previously_set_pixels: Vec::new(),

                z_buffer: Vec::new()
            }
        }

        pub fn helper(&mut self) -> &mut ANSIHelper {
            &mut self.helper
        }

        pub fn draw_at(&mut self, x: u32, y: u32, s: &str, z: f64, style: Option<ANSIStyle>) -> Result<(), &'static str> {
            // Ignore pixel if something above it already exists in z buffer
            for v in self.z_buffer.iter() {
                if v.0.0 == x && v.0.1 == y && v.1 < z {
                    return Ok(());
                }
            }

            // WARN
            // This if statement does not appear to have significant performance improvement
            //if self.helper.current_pos.x() != pos.x() || self.helper.current_pos.y() != pos.y() {
            self.helper.go_to(x, y);
            //}
            if let Some(st) = style {
                self.helper.set_style(st);
            }
            self.helper.write(s)?;
            self.currently_set_pixels.push((x, y));

            self.z_buffer.push(((x, y), z));

            Ok(())
        }

        pub fn set_style(&mut self, style: ANSIStyle) {
            self.helper.set_style(style);
        }

        pub fn clear_at(&mut self, x: u32, y: u32) {
            self.helper.go_to(x, y);
            _ = self.helper.write(" "); // Error doesn't matter. We are clearing the pixel.
        }

        pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
            let previous_pixels = std::mem::take(&mut self.previously_set_pixels);
            
            // Clear pixels that are now unset. Does not produce flicker like full clear.
            for previous in previous_pixels {
                let mut found = false;
                for current in self.currently_set_pixels.iter() {
                    if previous.0 == current.0 && previous.1 == current.1 {
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.clear_at(previous.0, previous.1);
                }
            }
            // Flush
            self.helper.flush()?;
            self.helper.go_to_immediate(0, 0);
            self.previously_set_pixels = std::mem::take(&mut self.currently_set_pixels);
            self.z_buffer.clear();

            Ok(())
        }

        pub fn go_to_immediate(&mut self, x: u32, y: u32) {
            self.helper.go_to_immediate(x, y);
        }

        pub fn go_to(&mut self, x: u32, y: u32) {
            self.helper.go_to(x, y);
        }

        pub fn clear(&mut self) {
            self.helper.full_clear();
        }

        pub fn rasterize_line(&mut self) {

        }

        pub fn draw_bitmap(&mut self) {

        }

        pub fn draw_image_2d(&mut self, image: &ImageData, position: Vector2i, z: f64) -> Result<(), &'static str> {
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let pixel = image.get_pixel_at(x, y);

                    if pixel.r() != 0 {
                        let new_pos = position + Vector2i::new(x as i32, y as i32);
                        self.draw_at(new_pos.x() as u32, new_pos.y() as u32, "&", z, None)?;
                    }
                }
            }

            Ok(())
        }

        pub fn rasterize_vertices(&mut self, mesh: &Mesh, max_distance: f64) -> Result<(), &'static str> {
            let verts = mesh.get_transformed_verts();

            for x in 0..self.helper.width {
                for y in 0..self.helper.height {
                    for vert in verts.iter() {
                        let pixel_pos_3d = Vector3::new(f64::from(x), f64::from(y), 0.0);
                        let orthographic_projection_vector = Vector3::new(vert.x(), vert.y(), 0.0);
                        if (orthographic_projection_vector - pixel_pos_3d).magnitude() <= max_distance {
                            let mut c = "@";

                            if vert.z() > 16.0 {
                                c = ".";
                            }
                            else if vert.z() > 8.0 {
                                c = "u";
                            }
                            else if vert.z() > 4.0 {
                                c = "w";
                            }
                            else if vert.z() > 2.0 {
                                c = "W";
                            }

                            self.draw_at(x, y, c, vert.z(), None)?;
                        }
                    }
                }
            }

            Ok(())
        }
    }

    pub struct ANSIHelper {
        x: u32,
        y: u32,

        width: u32,
        height: u32,

        screen: Screen<char>,
        aux_buffer: HashMap<(u32, u32), String>
    }

    impl ANSIHelper {
        pub fn new() -> Self {
            let w = 120;
            let h = 50;

            ANSIHelper {
                x: 0,
                y: 0,

                width: w,
                height: h,

                screen: Screen::new(w, h),
                aux_buffer: HashMap::new(),
            }
        }

        pub fn write(&mut self, text: &str) -> Result<(), &'static str> {
            self.screen.push_pixels(self.x, self.y, Vec::from_iter(text.chars()))?;
            self.advance(text.len() as u32);
            
            Ok(())
        }

        fn advance(&mut self, length: u32) {
            self.x += length;
            while self.x > self.width {
                self.x -= self.width;
                self.y += 1;
            }
        }

        pub fn write_aux(&mut self, x: u32, y: u32, text: &str) {
            if self.aux_buffer.contains_key(&(x, y)) {
                let s =  self.aux_buffer.get_mut(&(x, y)).expect("This key already been established to exist");
                s.push_str(text);
                return;
            }
            self.aux_buffer.insert((x, y), String::from(text));
        }

        pub fn flush(&mut self) -> Result<(), FromUtf8Error> {
            self.go_to_immediate(0, 0);
            //print!("{}", self.screen.get_buffer().as_slice()); // Maybe there's a way to make it work?
            self.x = 0;
            self.y = 0;

            let mut goto = false;

            for c in self.screen.get_buffer() {
                if let Some(aux) = self.aux_buffer.get(&(self.x, self.y)) {
                    print!("{aux}");
                }
                if *c == '\0' {
                    goto = true;
                }
                else {
                    if goto {
                        print!("\u{001B}[{};{}H", self.y, self.x);
                        goto = false;
                    }
                    print!("{}", c);
                }
                
                // Rust borrow checker moment. Can't use self functions
                self.x += 1;
                while self.x > self.width {
                    self.x -= self.width;
                    self.y += 1;

                    print!("\u{001B}[{};{}H", self.y, self.x);
                }
            }
            self.screen.clear();
            Ok(())
        }

        pub fn go_to(&mut self, x: u32, y: u32) {
            self.x = x;
            self.y = y;
        }

        pub fn go_to_immediate(&mut self, x: u32, y: u32) {
            self.x = x;
            self.y = y;
            self.csi_start_immediate();
            print!("{y};{x}H");
        }
        
        pub fn set_style(&mut self, style: ANSIStyle) {
            self.csi_start();
            self.write_aux(self.x, self.y, &((style as i32).to_string() + "m"));
        }

        pub fn carriage_return(&mut self) {
            self.write_aux(self.x, self.y,"\u{000D}");
        }

        pub fn beep(&mut self) {
            self.write_aux(self.x, self.y, "\u{0007}");
        }

        pub fn full_clear(&mut self) {
            self.csi_start_immediate();
            print!("0J");
        }

        pub fn csi_start_immediate(&mut self) {
            self.escape_start_immediate();
            print!("[");
        }

        pub fn csi_start(&mut self) {
            self.escape_start();
            self.write_aux(self.x, self.y, "["); // TODO: potentially improve error handling
        }
        
        pub fn escape_start_immediate(&mut self) {
            print!("\u{001B}");
        }

        pub fn escape_start(&mut self) {
            self.write_aux(self.x, self.y, "\u{001B}"); // TODO: potentially improve error handling
        }
    }

    pub enum ANSIStyle {
        None = 0,
        Bold,
        Faint,
        Italic,
        Underline,
        SlowBlink,
        FastBlink,
        Invert,
        Hide,
        CrossedOut,
        PriamryFont,
        AlternativeFont,
        Fraktur = 20,
        DoublyUnderlinedOrNotBold,
        NormalIntensity,
        NeitherItalicNorBlackletter,
        NotUnderlined,
        NotBlinking,
        ProportionalSpacing,
        NotReversed,
        Reveal,
        NotCrossedOut,
        SetForegroundColor,
        SetForegroundColor8bit = 38,
        DefaultForegroundColor,
        SetBackgroundColor,
        SetBackgroundColor8bit = 48,
        DefaultBackgroundColor,
        DisableProportionalSpacing,
        Framed,
        Encircled,
        Overlined,
        NeitherFramedNorEncircled,
        NotOverlined,
        SetUnderlineColor,
        DefaultUnderlineColor,
        IdeogramUnderlineOrRightSideLine,
        IdeogramDoubleUnderlineOrDoubleRightSideLine,
        IdeogramOverlineOrLeftSideLine,
        IdeogramDoubleOverlineOrDoubleLeftSideLine,
        IdeogramStressMaking,
        NoIdeogramAttributes,
        Superscript,
        Subscript,
        NeitherSubscriptNorSuperscirpt,
        SetBrightForegroundColor,
        SetBrightBackgroundColor = 100,
    }
}