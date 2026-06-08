use glam::Vec2;

use super::tween::{Easing, Tween};

pub struct AnimationSequence {
    steps: Vec<AnimationStep>,
    current_step: usize,
    elapsed_in_step: f32,
    running: bool,
}

#[allow(dead_code)]
enum AnimationStep {
    MoveCamera { target: Vec2, duration: f32, easing: Easing },
    ZoomCamera { target_zoom: f32, duration: f32, easing: Easing },
    Wait { duration: f32 },
}

impl AnimationStep {
    fn duration(&self) -> f32 {
        match self {
            AnimationStep::MoveCamera { duration, .. }
            | AnimationStep::ZoomCamera { duration, .. }
            | AnimationStep::Wait { duration } => *duration,
        }
    }
}

impl AnimationSequence {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            elapsed_in_step: 0.0,
            running: false,
        }
    }

    pub fn move_camera(mut self, target: Vec2, duration: f32, easing: Easing) -> Self {
        self.steps.push(AnimationStep::MoveCamera { target, duration, easing });
        self
    }

    pub fn zoom_camera(mut self, target_zoom: f32, duration: f32, easing: Easing) -> Self {
        self.steps.push(AnimationStep::ZoomCamera { target_zoom, duration, easing });
        self
    }

    pub fn wait(mut self, duration: f32) -> Self {
        self.steps.push(AnimationStep::Wait { duration });
        self
    }

    pub fn start(&mut self) {
        self.current_step = 0;
        self.elapsed_in_step = 0.0;
        self.running = true;
    }

    pub fn update(&mut self, dt: f32) -> Option<AnimationEvent> {
        if !self.running || self.current_step >= self.steps.len() {
            self.running = false;
            return None;
        }

        self.elapsed_in_step += dt;

        let step_duration = self.steps[self.current_step].duration();
        if self.elapsed_in_step >= step_duration {
            let completed = self.current_step;
            self.current_step += 1;
            self.elapsed_in_step = 0.0;

            if self.current_step >= self.steps.len() {
                self.running = false;
            }

            return Some(AnimationEvent::StepCompleted(completed));
        }

        Some(AnimationEvent::StepInProgress(self.current_step))
    }

    pub fn is_finished(&self) -> bool {
        !self.running && self.current_step >= self.steps.len()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[derive(Debug)]
pub enum AnimationEvent {
    StepCompleted(usize),
    StepInProgress(usize),
}

pub struct StaggeredAnimation {
    animations: Vec<Tween<Vec2>>,
    delay_per_item: f32,
}

impl StaggeredAnimation {
    pub fn new(delay_per_item: f32) -> Self {
        Self {
            animations: Vec::new(),
            delay_per_item,
        }
    }

    pub fn add(&mut self, from: Vec2, to: Vec2, duration: f32, easing: Easing) {
        let delay = self.animations.len() as f32 * self.delay_per_item;
        let mut tween = Tween::new(from, to, duration, easing);
        tween.elapsed = -delay;
        self.animations.push(tween);
    }

    pub fn update(&mut self, dt: f32) -> Vec<(usize, Vec2)> {
        let mut results = Vec::new();

        for (i, anim) in self.animations.iter_mut().enumerate() {
            if !anim.is_finished() {
                let pos = anim.update(dt);
                results.push((i, pos));
            }
        }

        results
    }

    pub fn is_finished(&self) -> bool {
        self.animations.iter().all(|a| a.is_finished())
    }
}

impl Default for AnimationSequence {
    fn default() -> Self {
        Self::new()
    }
}
