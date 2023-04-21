use std::collections::LinkedList;
use accrete::{Accrete, Planetesimal, System};
use chrono::{DateTime, Utc};
use egui::{LayerId, Painter, Response, Rgba, Sense, Shape, Stroke, Ui, Widget};
use egui::Shape::LineSegment;
use num_traits::{Float, FloatConst};
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::math::camera;
use crate::math::camera::Camera2D;
use crate::math::ellipsis::Ellipsis;
use crate::math::rect::Rect;
use crate::math::scalar::Vec2;

const AU_MILLIONKM: f64 = 149_597.870_700;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpaceMap {
    system: System,
    created: DateTime<Utc>,
}

impl SpaceMap {
    pub fn new(seed: u64) -> Self {
        let mut accrete = Accrete::new(seed);
        accrete.stellar_mass = 2.0;
        // accrete.dust_density_coeff = 1e-10;
        let system = accrete.planetary_system();
        Self {
            system,
            created: Utc::now(),
        }
    }

    fn planets_recursive(&self, shapes: &mut impl Extend<Shape>, camera: &Camera2D<f64>, pos: Vec2<f64>, planets: &[Planetesimal]) {
        for planet in planets {
            let a = planet.a * AU_MILLIONKM;
            let b = planet.b * AU_MILLIONKM;
            let orbit = Ellipsis::new_at_focal_pos(pos, [a, b].into(), planet.axial_tilt * std::f64::consts::PI / 180.0);
            orbit.draw_line(shapes, camera, Stroke::new(0.75, Rgba::from_gray(1.0).multiply(0.5)));
            let planet_pos = orbit.position_at((Utc::now().timestamp_millis() - self.created.timestamp_millis()) as f64 / 1000.0 / planet.orbital_period_days * 2.0 * std::f64::consts::PI * 365.0);
            let x = Stroke::new(1.0, Rgba::from_rgb(1.0, 0.0, 0.0).multiply(0.5));
            let transformed_pos = camera.transform_point(planet_pos);
            shapes.extend([
                LineSegment {
                    points: [
                        (transformed_pos + Vec2::new([-4.0, -4.0])).into(),
                        (transformed_pos + Vec2::new([4.0, 4.0])).into()
                    ],
                    stroke: x,
                }, LineSegment {
                    points: [
                        (transformed_pos + Vec2::new([4.0, -4.0])).into(),
                        (transformed_pos + Vec2::new([-4.0, 4.0])).into()
                    ],
                    stroke: x,
                }]);
            let r = planet.radius * 1e-6;
            Ellipsis::new(planet_pos, [r, r].into(), 0.0).draw(shapes, camera, Rgba::from_luminance_alpha(0.5, 1.0), Stroke::none());
            self.planets_recursive(shapes, camera, planet_pos, &planet.moons)
        }
    }

    #[inline]
    pub fn paint(&self, camera: &Camera2D<f64>) -> Vec<Shape> {
        let mut vec = Vec::new();
        let [r, g, b] = self.system.primary_star.color;
        let color = Rgba::from_rgb(r as _, g as _, b as _);
        let radius = self.system.primary_star.stellar_radius_au * AU_MILLIONKM;
        Ellipsis::new_at_focal_pos(Vec2::default(), [radius, radius].into(), 0.0).draw(&mut vec, camera, color, Stroke::none());
        self.planets_recursive(&mut vec, camera, Vec2::default(), &self.system.planets);
        vec
    }
}