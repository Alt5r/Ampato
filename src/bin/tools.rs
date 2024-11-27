use std::{intrinsics::sqrtf64, ops::*};

struct Vector<T> {
    x:T,
    y:T
}

// overloading * for any numebr type (hopefully)
impl<T> Mul<T> for Vector<T> 
where 
    T: Mul<Output = T> + Copy, // T must implement Mul and Copy for multiplication
{
    type Output = Vector<T>;

    fn mul(self, a: T) -> Vector<T> {
        Vector {
            x: self.x * a,
            y: self.y * a,
        }
    }
}

impl<T> Add<Vector<T>> for Vector<T> where T: Add<Output = T> + Copy {
    type Output = Vector<T>;

    fn add(self, a: Vector<T>) -> Vector<T> {
        Vector {
            x : self.x + a.x,
            y : self.y + a.y
        }
    }
}

impl<T> Sub<Vector<T>> for Vector<T> where T:Sub<Output = T> + Copy {
    type Output = Vector<T>;

    fn sub(self, a: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x - a.x,
            y: self.y - a.y
        }
    }
}

impl<T> Div<Vector<T>> for Vector<T> where T:Div<Output = T> + Copy {
    type Output = Vector<T>;

    fn div(self, a: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x / a,
            y: self.y / a
        }
    }
}


impl Vector<T> {

    fn add(&self, a:f64) {
        self.x = self.x + a;
        self.y = self.y + a;
    }

    fn scale(&self, a:f64) {
        self.x *= a;
        self.y *= a;
    }

    fn parse_to_int(&self) -> (f64, f64){
        (self.x, self.y)
    }

    fn magnitude(&self) -> f64 {
        let x = unsafe{self.x * self.x + self.y * self.y};
        sqrtf64(x)
    }

}