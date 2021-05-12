use itertools::multizip;
use nalgebra::Rotation3;
use serde_json::json;
use tool::{List, Vector, Vectors};

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EA_2026.tm")?;
    let reference_frame_name = "ECLIPJ2000";
    let origin_name = "SUN";
    let body_name = "DIMORPHOS";
    let body_frame_name = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 1.0 * tool::DAY;
    let duration = 192.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();
    let inertial_upward = Vector::new(0.0, 0.0, 1.0);
    // let reference_frame = nalgebra::Rotation3::<f64>::identity();

    let mut body_positions = Vectors::<f64>::zeros(size);
    let mut body_frames = vec![Rotation3::new(Vector::zeros()); size];
    let mut obliquities = List::<f64>::zeros(size);

    for (et, mut pos, frm, tilt) in multizip((
        ephemeris_times.iter().cloned(),
        body_positions.column_iter_mut(),
        body_frames.iter_mut(),
        obliquities.iter_mut(),
    )) {
        pos.copy_from(&spice::spkpos(body_name, et, reference_frame_name, abcorr, origin_name).0);
        *frm = spice::pxform(reference_frame_name, body_frame_name, et);
        let local_upward = *frm * inertial_upward;
        *tilt = local_upward.angle(&inertial_upward) * tool::RAD2DEG;
    }

    // Export.
    let json = json!({
        "context": {
            "reference_frame_name": reference_frame_name,
            "origin_name": origin_name,
            "body_name": body_name,
            "body_frame_name": body_frame_name,
            "abcorr": abcorr,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "ephemeris_times": ephemeris_times,
        "body_positions": body_positions,
        "body_frames": body_frames,
        "obliquities": obliquities,
    });
    tool::writejs!("rsc/data/tmp/asteroid.json", json);

    kernel.unload()?;
    Ok(())
}