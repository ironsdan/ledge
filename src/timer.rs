use std::time;

pub struct TimerState {
    _initial_instant: time::Instant,
    last_instant: time::Instant,
    frame_times: Vec<time::Duration>,
    pub accumulator: time::Duration,
}

impl TimerState {
    pub fn new() -> Self {
        Self{
            _initial_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_times: Vec::new(),
            accumulator: time::Duration::from_secs(0),
        }
    }

    pub fn tick(&mut self) {
        let now = time::Instant::now();
        let frame_time = now - self.last_instant;
        // println!("Frame time: {:?}", frame_time);
        self.frame_times.push(frame_time);
        self.last_instant = now;
        self.accumulator += frame_time;
    }

    pub fn alpha(&self) -> f32 {
        let target_dt = fps_as_duration(60);
        self.accumulator.as_secs_f32() / target_dt.as_secs_f32()
    }

    pub fn check_update_time(&mut self, target_fps: u32) -> bool {
        let target_dt = fps_as_duration(target_fps);
        // println!("{:?} {:?}", self.accumulator, target_dt);
        if self.accumulator >= target_dt {
            self.accumulator -= target_dt;
            true
        } else {
            false
        }
    }
}

pub fn fps_as_duration(fps: u32) -> time::Duration {
    let target_dt_seconds = 1.0 / f64::from(fps);
    f64_to_duration(target_dt_seconds)
}

pub fn f64_to_duration(t: f64) -> time::Duration {
    debug_assert!(t > 0.0, "f64_to_duration passed a negative number!");
    let seconds = t.trunc();
    let nanos = t.fract() * 1e9;
    time::Duration::new(seconds as u64, nanos as u32)
}