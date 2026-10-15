use crate::render::Scene;
use mirui::anim::{PlayMode, Tween, ease};
use mirui::app::RendererFactory;
use mirui::prelude::*;
use mirui::surface::Surface;

pub struct Feedback {
    press: Tween,
    activity: [Tween; 3],
    last_ms: u64,
}

impl Default for Feedback {
    fn default() -> Self {
        Self {
            press: Tween::ease_to(Fixed::ZERO, Fixed::ZERO, 180),
            activity: std::array::from_fn(|index| {
                let mut tween = Tween::new(
                    Fixed::ZERO,
                    Fixed::ONE,
                    450,
                    &ease::LINEAR,
                    PlayMode::PingPong,
                );
                tween.elapsed_ms = index as u16 * 105;
                tween
            }),
            last_ms: 0,
        }
    }
}

impl Feedback {
    pub fn press(&mut self, now: u64) {
        self.last_ms = now;
        self.press = Tween::ease_to(Fixed::ONE, Fixed::ZERO, 180);
    }

    pub fn apply(&mut self, scene: &Scene, now: u64) {
        let delta = now.saturating_sub(self.last_ms).min(u64::from(u16::MAX)) as u16;
        self.last_ms = now;
        self.press.tick(delta);
        let press = self.press.value().to_f32();
        let stamp = Color::rgb(
            (234.0 - 16.0 * press) as u8,
            (207.0 - 37.0 * press) as u8,
            (192.0 - 60.0 * press) as u8,
        );
        let activity = std::array::from_fn(|index| {
            self.activity[index].tick(delta);
            if scene.loading {
                Color::rgba(
                    166,
                    61,
                    50,
                    (48.0 + self.activity[index].value().to_f32() * 207.0) as u8,
                )
            } else {
                Color::rgb(242, 235, 221)
            }
        });
        scene.feedback(stamp, activity);
    }

    pub fn paint<B: Surface, F: RendererFactory<B>>(
        &mut self,
        app: &mut App<B, F>,
        scene: &Scene,
        now: u64,
    ) -> Result<(), String> {
        self.apply(scene, now);
        if app.tick() {
            return Err("The display requested shutdown".into());
        }
        Ok(())
    }
}
