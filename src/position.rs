use nalgebra::{Rotation3, Vector3};

pub struct Position {
    position: Vector3<f64>,
    rotation: Rotation3<f64>,
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
        .map_or(delta.y.signum() * std::f64::consts::PI, f64::atan)
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16, azimuth: i16, elevation: i8) -> Self {
        assert!(azimuth >= -180 && azimuth <= 180);
        assert!(elevation >= -90 && elevation <= 90);
        Position {
            position: Vector3::new(x as f64, y as f64, z as f64),
            rotation: Rotation3::from_axis_angle(
                &Vector3::x_axis(),
                (elevation as f64).to_radians(),
            ) * Rotation3::from_axis_angle(
                &Vector3::y_axis(),
                (-azimuth as f64).to_radians(),
            ),
        }
    }

    pub fn compute_range_azimuth_elevation(&self, other: Position) -> (u16, i16, i8) {
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
