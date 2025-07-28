///-------------------------------------------------------------------------------
///
/// This is your first task to get warmed up and see how useful traits can be.
/// 
/// Complete the implementation of methods in the Rectangle and Circle structs, 
/// then implement the Shape trait for both structs.
/// 
/// Tasks:
/// 1. Implement Rectangle struct methods (constructor, setters, getters)
/// 2. Implement Circle struct methods (constructor, setter, getter)  
/// 3. Implement the Shape trait for both Rectangle and Circle
/// 4. Handle validation errors properly using the Error enum
/// 
///-------------------------------------------------------------------------------



use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Circle {
    radius: f64,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidWidth,
    InvalidHeight,
    InvalidRadius,
}

// Implementation for Rectangle
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Result<Self, Error> {
        if width < 0.0 {
            return Err(Error::InvalidWidth);
        }
        if height < 0.0 {
            return Err(Error::InvalidHeight);
        }
        Ok(Rectangle { width, height })
    }

    pub fn set_width(&mut self, width: f64) -> Result<(), Error> {
        if width < 0.0 {
            Err(Error::InvalidWidth)
        } else {
            self.width = width;
            Ok(())
        }
    }

    pub fn set_height(&mut self, height: f64) -> Result<(), Error> {
        if height < 0.0 {
            Err(Error::InvalidHeight)
        } else {
            self.height = height;
            Ok(())
        }
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }
}

// Implementation for Circle
impl Circle {
    pub fn new(radius: f64) -> Result<Self, Error> {
        if radius < 0.0 {
            Err(Error::InvalidRadius)
        } else {
            Ok(Circle { radius })
        }
    }

    pub fn set_radius(&mut self, radius: f64) -> Result<(), Error> {
        if radius < 0.0 {
            Err(Error::InvalidRadius)
        } else {
            self.radius = radius;
            Ok(())
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

// Implement Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Implement Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
