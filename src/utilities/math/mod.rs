use nalgebra::{Matrix3, Matrix4, UnitQuaternion, Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

pub type Mat3 = Matrix3<f32>;
pub type Mat4 = Matrix4<f32>;

pub type Quat = UnitQuaternion<f32>;

pub const M_PI: f32 = std::f32::consts::PI;
pub const M_2PI: f32 = std::f32::consts::TAU;
pub const M_HPI: f32 = std::f32::consts::FRAC_PI_2;
pub const M_QPI: f32 = std::f32::consts::FRAC_PI_4;

pub const M_GPI: f32 = 1.618_034;
pub const M_RADPI: f32 = 57.295_78;

pub const EPSILON: f32 = 1e-6;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg / M_RADPI
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * M_RADPI
}

pub fn normalize_angle(mut angle: f32) -> f32 {
    while angle > 180.0 {
        angle -= 360.0;
    }
    while angle < -180.0 {
        angle += 360.0;
    }
    angle
}

pub fn normalize_angles(angles: Vec2) -> Vec2 {
    Vec2::new(normalize_angle(angles.x), normalize_angle(angles.y))
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.dot(&b)
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    a.cross(&b)
}

pub fn normalize(v: Vec3) -> Vec3 {
    if v.norm() < EPSILON {
        Vec3::zeros()
    } else {
        v.normalize()
    }
}

pub fn distance(a: Vec3, b: Vec3) -> f32 {
    (b - a).norm()
}

pub fn calc_angle(src: Vec3, dst: Vec3) -> Vec2 {
    let delta = dst - src;

    let hyp = (delta.x * delta.x + delta.y * delta.y).sqrt();

    let pitch = rad_to_deg((-delta.z).atan2(hyp));
    let yaw = rad_to_deg(delta.y.atan2(delta.x));

    normalize_angles(Vec2::new(pitch, yaw))
}

pub fn calc_fov(view_angles: Vec2, aim_angles: Vec2) -> f32 {
    let delta = normalize_angles(aim_angles - view_angles);
    (delta.x * delta.x + delta.y * delta.y).sqrt()
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

pub fn clamp_angles(mut angles: Vec2) -> Vec2 {
    angles.x = clamp(angles.x, -89.0, 89.0);
    angles.y = normalize_angle(angles.y);
    angles
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_vec(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

pub fn vector_to_angles(forward: Vec3) -> Vec2 {
    let hyp = (forward.x * forward.x + forward.y * forward.y).sqrt();

    let pitch = rad_to_deg((-forward.z).atan2(hyp));
    let yaw = rad_to_deg(forward.y.atan2(forward.x));

    normalize_angles(Vec2::new(pitch, yaw))
}

pub fn angles_to_vector(angles: Vec2) -> Vec3 {
    let pitch = deg_to_rad(angles.x);
    let yaw = deg_to_rad(angles.y);

    Vec3::new(
        pitch.cos() * yaw.cos(),
        pitch.cos() * yaw.sin(),
        -pitch.sin(),
    )
}

pub fn smooth_angle(current: Vec2, target: Vec2, factor: f32) -> Vec2 {
    let delta = normalize_angles(target - current);
    current + delta / factor.max(1.0)
}

pub fn world_to_screen(pos: Vec3, matrix: Mat4, width: f32, height: f32) -> Option<Vec2> {
    let clip = matrix * Vec4::new(pos.x, pos.y, pos.z, 1.0);

    if clip.w < 0.1 {
        return None;
    }

    let ndc = clip.xyz() / clip.w;

    let screen = Vec2::new(
        (width / 2.0) * (1.0 + ndc.x),
        (height / 2.0) * (1.0 - ndc.y),
    );

    Some(screen)
}
