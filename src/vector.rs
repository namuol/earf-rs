// Quick port from *coffeescript* 😂
// class Vector
//   constructor: (@x, @y, @z) ->
//   add: (v) ->
//     return new Vector(@x+v.x, @y+v.y, @z+v.z)
//   sub: (v) ->
//     return new Vector(@x-v.x, @y-v.y, @z-v.z)
//   mul: (s) ->
//     return new Vector(@x*s,@y*s,@z*s)
//   normal: ->
//     mag = Math.sqrt(@x*@x + @y*@y + @z*@z)
//     @x /= mag
//     @y /= mag
//     @z /= mag
//     @
//   len: ->
//     return Math.sqrt @x*@x + @y*@y + @z*@z

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq)]
pub struct Vector {
  x: f64,
  y: f64,
  z: f64,
}

impl Add for Vector {
  type Output = Vector;

  fn add(self, other: Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vector {
  type Output = Vector;

  fn sub(self, other: Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Vector;
  fn mul(self, scalar: f64) -> Vector {
    Vector {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Div<f64> for Vector {
  type Output = Vector;
  fn div(self, scalar: f64) -> Vector {
    Vector {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
    }
  }
}

impl Add<Vector> for &Vector {
  type Output = Vector;

  fn add(self, other: Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub<Vector> for &Vector {
  type Output = Vector;

  fn sub(self, other: Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<f64> for &Vector {
  type Output = Vector;
  fn mul(self, scalar: f64) -> Vector {
    Vector {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Div<f64> for &Vector {
  type Output = Vector;
  fn div(self, scalar: f64) -> Vector {
    Vector {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
    }
  }
}

impl AddAssign<Vector> for Vector {
  fn add_assign(&mut self, other: Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl SubAssign<Vector> for Vector {
  fn sub_assign(&mut self, other: Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl MulAssign<f64> for Vector {
  fn mul_assign(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }
}

impl DivAssign<f64> for Vector {
  fn div_assign(&mut self, scalar: f64) {
    self.x /= scalar;
    self.y /= scalar;
    self.z /= scalar;
  }
}

impl AddAssign<Vector> for &mut Vector {
  fn add_assign(&mut self, other: Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl SubAssign<Vector> for &mut Vector {
  fn sub_assign(&mut self, other: Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl MulAssign<f64> for &mut Vector {
  fn mul_assign(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }
}

impl DivAssign<f64> for &mut Vector {
  fn div_assign(&mut self, scalar: f64) {
    self.x /= scalar;
    self.y /= scalar;
    self.z /= scalar;
  }
}

impl Vector {
  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&mut self) {
    // TODO: The borrow-checker doesn't like this:
    // self /= self.length();

    let length = self.length();
    self.x /= length;
    self.y /= length;
    self.z /= length;
  }

  pub fn normalized(&self) -> Self {
    let length = self.length();
    self / length
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn equal_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(a, b);
  }

  #[test]
  fn add_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(
      a + b,
      Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
      }
    );
  }

  #[test]
  fn sub_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let b = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert_eq!(
      a - b,
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      }
    );
  }

  #[test]
  fn mul_operator() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a * 3.0,
      Vector {
        x: 3.0,
        y: 6.0,
        z: 9.0,
      }
    );
  }

  #[test]
  fn add_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a += Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a,
      Vector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
      }
    )
  }

  #[test]
  fn sub_assign_operator() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a -= Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(
      a,
      Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      }
    )
  }

  #[test]
  fn length_squared() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.length_squared(), 14.0);
  }

  #[test]
  fn length() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.length(), (a.length_squared()).sqrt());
  }

  #[test]
  fn normalize() {
    let mut a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    a.normalize();

    assert_eq!(a.length(), 1.0);
  }

  #[test]
  fn normalized() {
    let a = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };

    assert_eq!(a.normalized().length(), 1.0);
  }
}
