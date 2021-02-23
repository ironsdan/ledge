use std::time;

pub struct TimerState {
    initial_instant: time::Instant,
    last_instant: time::Instant,
    frame_times: Vec<time::Duration>,
    pub dt_left: time::Duration,
}

impl TimerState {
    pub fn new() -> Self {
        Self{
            initial_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_times: Vec::new(),
            dt_left: time::Duration::from_secs(0),
        }
    }

    pub fn tick(&mut self) {
        let now = time::Instant::now();
        let mut frame_time = now - self.last_instant;
        // println!("time since last: {:?}", frame_time);
        if frame_time > time::Duration::from_millis(25) {
            frame_time = time::Duration::from_millis(25);
        }
        self.frame_times.push(frame_time);
        self.last_instant = now;
        self.dt_left += frame_time;
        // println!("dt_left: {:?}", self.dt_left);
    }

    pub fn delta(&self) -> time::Duration {
        *self.frame_times.last().unwrap()
    }

    pub fn alpha(&self) -> f32 {
        let target_dt = self.fps_as_duration(60);
        // println!("ALPHA: {:?}", self.dt_left.as_secs_f32() / target_dt.as_secs_f32());
        self.dt_left.as_secs_f32() / target_dt.as_secs_f32()
    }

    pub fn check_update_time(&mut self, target_fps: u32) -> bool {
        let target_dt = self.fps_as_duration(target_fps);
        // println!("targetdt: {:?}", target_dt);
        if self.dt_left > target_dt {
            self.dt_left -= target_dt;
            true
        } else {
            false
        }
    }

    fn fps_as_duration(&self, fps: u32) -> time::Duration {
        let target_dt_seconds = 1.0 / f64::from(fps);
        self.f64_to_duration(target_dt_seconds)
    }

    pub fn f64_to_duration(&self, t: f64) -> time::Duration {
        debug_assert!(t > 0.0, "f64_to_duration passed a negative number!");
        let seconds = t.trunc();
        let nanos = t.fract() * 1e9;
        time::Duration::new(seconds as u64, nanos as u32)
    }
}