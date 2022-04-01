use crate::uci_packets::PicaPosition;
use nalgebra::{Rotation3, Vector3};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::From;
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Position {
    position: Vector3<f64>,
    rotation: Rotation3<f64>,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Position", 5)?;
        state.serialize_field("x", &(self.position.x as i16))?;
        state.serialize_field("y", &(self.position.y as i16))?;
        state.serialize_field("z", &(self.position.z as i16))?;

        let vector = self.rotation * Vector3::new(0.0, 0.0, 1.0);

        state.serialize_field("azimuth", &(-azimuth(vector).to_degrees() as i16))?;
        state.serialize_field("elevation", &(elevation(vector).to_degrees() as i8))?;
        state.end()
    }
}

fn checked_div(num: f64, den: f64) -> Option<f64> {
    if den == 0. {
        None
    } else {
        Some(num / den)
    }
}

fn azimuth(delta: Vector3<f64>) -> f64 {
    checked_div(delta.x, delta.z).map_or(
        if delta.x == 0. {
            0.
        } else {
            delta.x.signum() * std::f64::consts::FRAC_PI_2
        },
        f64::atan,
    ) + if delta.z >= 0. {
        0.
    } else {
        delta.x.signum() * std::f64::consts::PI
    }
}

fn elevation(delta: Vector3<f64>) -> f64 {
    checked_div(delta.y, f64::sqrt(delta.x.powi(2) + delta.z.powi(2)))
        .map_or(delta.y.signum() * std::f64::consts::FRAC_PI_2, f64::atan)
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16, azimuth: i16, elevation: i8) -> Self {
        assert!(azimuth >= -180 && azimuth <= 180);
        assert!(elevation >= -90 && elevation <= 90);
        Position {
            position: Vector3::new(x as f64, y as f64, z as f64),
            rotation: Rotation3::from_axis_angle(
                &Vector3::y_axis(),
                (-azimuth as f64).to_radians(),
            ) * Rotation3::from_axis_angle(
                &Vector3::x_axis(),
                (elevation as f64).to_radians(),
            ),
        }
    }

    pub fn compute_range_azimuth_elevation(&self, other: &Position) -> (u16, i16, i8) {
        let delta = other.position - self.position;

        let distance = f64::sqrt(delta.x.powi(2) + delta.y.powi(2) + delta.z.powi(2)).round();
        let orientation = self.rotation * delta;

        let azimuth = azimuth(orientation).to_degrees().round();
        let elevation = elevation(orientation).to_degrees().round();

        assert!(azimuth >= -180. && azimuth <= 180.);
        assert!(elevation >= -90. && elevation <= 90.);

        (
            f64::min(distance, u16::MAX as f64) as u16,
            azimuth as i16,
            elevation as i8,
        )
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }
}

impl From<&PicaPosition> for Position {
    fn from(other: &PicaPosition) -> Self {
        Self::new(
            other.x as i16,
            other.y as i16,
            other.z as i16,
            other.azimuth as i16,
            other.elevation as i8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn azimuth_simple() {
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(10, 0, 10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 45);
            assert!(elevation == 0);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(-10, 0, 10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == -45);
            assert!(elevation == 0);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(10, 0, -10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 135);
            assert!(elevation == 0);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(-10, 0, -10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == -135);
            assert!(elevation == 0);
        }
    }

    #[test]
    fn elevation_simple() {
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(0, 10, 10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 0);
            assert!(elevation == 45);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(0, -10, 10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 0);
            assert!(elevation == -45);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(0, 10, -10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 180 || azimuth == -180);
            assert!(elevation == 45);
        }
        {
            let position_a = Position::new(0, 0, 0, 0, 0);
            let position_b = Position::new(0, -10, -10, 0, 0);

            let (_range, azimuth, elevation) =
                position_a.compute_range_azimuth_elevation(&position_b);
            assert!(azimuth == 180 || azimuth == -180);
            assert!(elevation == -45);
        }
    }

    #[test]
    fn azimuth_elevation_check_positive() {
        let position_a = Position::new(0, 0, 0, 42, 35);
        let position_b = Position::new(0, 0, 10, 0, 0);

        let (_range, azimuth, elevation) = position_a.compute_range_azimuth_elevation(&position_b);
        println!("{} {}", azimuth, elevation);
        assert!(azimuth == -42);
        assert!(elevation == -35);
    }

    #[test]
    fn azimuth_elevation_check_first_negative() {
        let position_a = Position::new(0, 0, 0, -42, 42);
        let position_b = Position::new(0, 0, 10, 0, 0);

        let (_, azimuth, elevation) = position_a.compute_range_azimuth_elevation(&position_b);

        assert!(azimuth == 42);
        assert!(elevation == -42);
    }

    #[test]
    fn azimuth_elevation_check_second_negative() {
        let position_a = Position::new(0, 0, 0, 42, -42);
        let position_b = Position::new(0, 0, 10, 0, 0);

        let (_, azimuth, elevation) = position_a.compute_range_azimuth_elevation(&position_b);

        assert!(azimuth == -42);
        assert!(elevation == 42);
    }

    #[test]
    fn azimuth_elevation_check_negative() {
        let position_a = Position::new(0, 0, 0, -42, -42);
        let position_b = Position::new(0, 0, 10, 0, 0);
        let (_, azimuth, elevation) = position_a.compute_range_azimuth_elevation(&position_b);

        assert!(azimuth == 42);
        assert!(elevation == 42);
    }

    #[test]
    fn azimuth_elevation_check_over_90() {
        let position_a = Position::new(0, 0, 0, 90 + 42, 42);
        let position_b = Position::new(0, 0, 10, 0, 0);
        let (_, azimuth, elevation) = position_a.compute_range_azimuth_elevation(&position_b);

        assert!(azimuth == -(90 + 42));
        assert!(elevation == -42);
    }

    #[test]
    fn azimuth_composition() {
        let position_a = Position::new(0, 0, 0, 45, 0);
        let position_b = Position::new(10, 0, 10, 0, 0); // 45+45deg

        let res = position_a.compute_range_azimuth_elevation(&position_b);
        println!("{:?}", res);
        assert!(res.1 == 0);
        assert!(res.2 == 0);
    }
}
