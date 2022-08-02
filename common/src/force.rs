use serde::{Deserialize, Serialize};
/* GravityMethod encodes different ways of creating gravity in Hyperbolic space

PlanarConstant has gravity oriented in the "down" direction, and the force of gravity is
constant with repect to height.

PlanarConstant has gravity oriented in the "down" direction, and the force of gravity respects
the divergence theorem, which makes it proportial to the ratio of the area of the ground-plane
to the area of the surface which is equidistant to the ground-plane at the indicated height.

Zero disables gravity.
*/
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum GravityMethod {
    PlanarConstant,
    PlanarDivergent,
    Zero,
}

pub struct ForceParams {
    pub gravity_intensity: f64,
    pub air_drag_factor: f64, // lower number means more drag
    pub gravity_type: GravityMethod,
    pub float_speed: f64,
}

impl ForceParams {
    pub fn apply_forces(
        &self,
        inital_velocity: &na::Vector4<f64>,
        down: &na::Vector4<f64>,
        height: f64,
        is_colliding: bool,
        time: f64,
    ) -> na::Vector4<f64> {
        let mut velocity = *inital_velocity;

        // otherwise we apply air resistance and gravity
        if is_colliding {
            velocity = self.collsion_drag(&velocity, down, time);
        } else {
            velocity = self.air_drag(&velocity, time);
            velocity += self.gravity(down, height, time);
        }

        velocity
    }

    /// returns the change in velocity that occurs by pulled by gravity at "height" for "time"
    fn gravity(&self, down: &na::Vector4<f64>, height: f64, time: f64) -> na::Vector4<f64> {
        match self.gravity_type {
            GravityMethod::PlanarConstant => down * (self.gravity_intensity * time),
            GravityMethod::PlanarDivergent => {
                down * (self.gravity_intensity * time / height.cosh().powi(2))
            }
            GravityMethod::Zero => na::zero(),
        }
    }

    /// rewrites velocity to account for time spent under drag.
    fn air_drag(&self, inital_velocity: &na::Vector4<f64>, time: f64) -> na::Vector4<f64> {
        inital_velocity * self.air_drag_factor.powf(time)
    }

    fn collsion_drag(
        &self,
        inital_velocity: &na::Vector4<f64>,
        down: &na::Vector4<f64>,
        time: f64,
    ) -> na::Vector4<f64> {
        let ratio = 0.1_f64.powf(time);

        inital_velocity * ratio + down * -self.float_speed * (1_f64 - ratio)
    }
}
