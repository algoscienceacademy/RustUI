use std::time::Duration;

pub struct Animation {
    duration: Duration,
    easing: EasingFunction,
    start_value: f32,
    end_value: f32,
    current_value: f32,
    progress: f32,
}

#[derive(Clone, Copy)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Animation {
    pub fn new(start: f32, end: f32, duration: Duration) -> Self {
        Self {
            duration,
            easing: EasingFunction::Linear,
            start_value: start,
            end_value: end,
            current_value: start,
            progress: 0.0,
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn update(&mut self, delta_time: Duration) -> f32 {
        self.progress += delta_time.as_secs_f32() / self.duration.as_secs_f32();
        self.progress = self.progress.min(1.0);
        
        self.current_value = match self.easing {
            EasingFunction::Linear => self.linear_interpolate(),
            EasingFunction::EaseIn => self.ease_in(),
            EasingFunction::EaseOut => self.ease_out(),
            EasingFunction::EaseInOut => self.ease_in_out(),
        };
        
        self.current_value
    }

    fn linear_interpolate(&self) -> f32 {
        self.start_value + (self.end_value - self.start_value) * self.progress
    }

    fn ease_in(&self) -> f32 {
        let t = self.progress;
        self.start_value + (self.end_value - self.start_value) * (t * t)
    }

    fn ease_out(&self) -> f32 {
        let t = self.progress;
        self.start_value + (self.end_value - self.start_value) * (1.0 - (1.0 - t) * (1.0 - t))
    }

    fn ease_in_out(&self) -> f32 {
        let t = self.progress;
        self.start_value + (self.end_value - self.start_value) * 
            (if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 })
    }
}
