use crate::position::{Position, PositionMode};
use std::collections::HashSet;

#[derive(Default)]
pub struct Camera {
    pub pos: Position,
    /// Note that the zoom level can be a decimal value, but only the value before the decimal place
    /// is used when determining zoom multiplier.
    zoom_level: f32,
}

impl Camera {
    #[must_use]
    pub fn new() -> Self {
        Camera {
            ..Default::default()
        }
    }

    pub fn pan(&mut self, direction: &MoveDirection) {
        let speed = CAMERA_SPEED_PX / f64::from(self.zoom_multiplier());

        let primary_directions = direction.to_primary_directions();
        let pos = &mut self.pos;

        if primary_directions.contains(&&PrimaryDirection::North) {
            pos.set_y(pos.y(PositionMode::Pixels) + speed, PositionMode::Pixels);
        }
        if primary_directions.contains(&&PrimaryDirection::East) {
            pos.set_x(pos.x(PositionMode::Pixels) + speed, PositionMode::Pixels);
        }
        if primary_directions.contains(&&PrimaryDirection::South) {
            pos.set_y(pos.y(PositionMode::Pixels) - speed, PositionMode::Pixels);
        }
        if primary_directions.contains(&&PrimaryDirection::West) {
            pos.set_x(pos.x(PositionMode::Pixels) - speed, PositionMode::Pixels);
        }
    }

    /// The amount that the camera is currently zoomed by. Positive values denote zooming in and vice versa.
    ///
    /// For example, a value of 2 would indicate that everything looks twice as big as normal.
    pub fn zoom_multiplier(&self) -> f32 {
        // Round the zoom level so that zooming happens in increments instead of continuously.
        let clipped_zoom_level = self.zoom_level as i32;
        CAMERA_ZOOM_LEVEL_MULTIPLIER.powf(clipped_zoom_level as f32)
    }

    pub fn zoom(&mut self, lines: f32) {
        self.zoom_level += lines;
        self.zoom_level = self
            .zoom_level
            .clamp(CAMERA_MIN_ZOOM_LEVEL, CAMERA_MAX_ZOOM_LEVEL);
    }
}

#[derive(PartialEq, Eq)]
pub enum MoveDirection {
    Stationary,
    Primary(PrimaryDirection),
    Secondary((PrimaryDirection, PrimaryDirection)),
}

impl MoveDirection {
    pub fn from_primary_directions(primary_directions: HashSet<PrimaryDirection>) -> Self {
        let directions: Vec<PrimaryDirection> = primary_directions.into_iter().collect();

        let vertical_direction = if directions.contains(&PrimaryDirection::North) {
            Some(PrimaryDirection::North)
        } else if directions.contains(&PrimaryDirection::South) {
            Some(PrimaryDirection::South)
        } else {
            None
        };

        let horizontal_direction = if directions.contains(&PrimaryDirection::East) {
            Some(PrimaryDirection::East)
        } else if directions.contains(&PrimaryDirection::West) {
            Some(PrimaryDirection::West)
        } else {
            None
        };

        #[expect(clippy::unwrap_used, clippy::unnecessary_unwrap)]
        if vertical_direction.is_none() && horizontal_direction.is_none() {
            MoveDirection::Stationary
        } else if vertical_direction.is_some() && horizontal_direction.is_none() {
            MoveDirection::Primary(vertical_direction.unwrap())
        } else if horizontal_direction.is_some() && vertical_direction.is_none() {
            MoveDirection::Primary(horizontal_direction.unwrap())
        } else {
            MoveDirection::Secondary((vertical_direction.unwrap(), horizontal_direction.unwrap()))
        }
    }

    fn to_primary_directions(&self) -> Vec<&PrimaryDirection> {
        match self {
            MoveDirection::Stationary => {
                vec![]
            }
            MoveDirection::Primary(direction) => {
                vec![direction]
            }
            MoveDirection::Secondary((direction1, direction2)) => {
                vec![direction1, direction2]
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum PrimaryDirection {
    North,
    East,
    South,
    West,
}

/// The single-axis speed of the camera when moving, in pixels of the window per tick.
const CAMERA_SPEED_PX: f64 = 10.0;
/// The amount each zoom increment zooms in/out.
const CAMERA_ZOOM_LEVEL_MULTIPLIER: f32 = 1.15;
const CAMERA_MAX_ZOOM_LEVEL: f32 = 5.0;
const CAMERA_MIN_ZOOM_LEVEL: f32 = -20.0;
