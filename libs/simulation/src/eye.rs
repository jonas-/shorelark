use crate::*;
use std::f64::consts::*;

const FOV_RANGE: f64 = 0.25;
const FOV_ANGLE: f64 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f64,
    fov_angle: f64,
    cells: usize,
}

impl Eye {
    // FOV_RANGE, FOV_ANGLE & CELLS are the values we'll use during
    // simulation - but being able to create an arbitrary eye will
    // come handy during the testing:
    fn new(fov_range: f64, fov_angle: f64, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f64>,
        rotation: na::Rotation2<f64>,
        foods: &[Food],
    ) -> Vec<f64> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            // check if food is inside the view range
            let vec = food.position - position;
            let dist = vec.norm();
            if dist >= self.fov_range {
                continue;
            }

            // check if inside the FoV
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle(); // correct own angle
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // check the heading, which eye cell sees food
            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f64);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f64,
        fov_angle: f64,
        x: f64,
        y: f64,
        rot: f64,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();

            let actual_vision = actual_vision.join("");

            // The finish line!
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    // A helper-function that allows to create food easily
    fn food(x: f64, y: f64) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        /// During tests in this module, we're using a world that looks
        /// like this:
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>   %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Each test gradually reduces our birdie's field of view and
        /// compares what the birdie sees:
        ///
        /// ------------
        /// |        /.|
        /// |      /...|
        /// |    @>...%|
        /// |      \\...|
        /// |        \\.|
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |      /.| |
        /// |    @>..|%|
        /// |      \\.| |
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>.| %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Over time, what we see is the food gradually disappearing
        /// into an emptiness:
        ///
        /// (well, technically the food and bird remain stationary - it's
        /// only the birdie's own field of view that gets reduced.)
        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]

        fn test(fov_range: f64, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "            +")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f64, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking the X axis:
        // (you can see the bird is "flying away" from the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f64, y: f64, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI as f64, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI as f64, ".  +     +  .")]
        #[test_case(0.75 * PI as f64, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI as f64, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI as f64, "   . + + .   ")]
        #[test_case(1.50 * PI as f64, ".   .+ +.   .")]
        #[test_case(1.75 * PI as f64, ".   .+ +.   .")]
        #[test_case(2.00 * PI as f64, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f64, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }
            .run()
        }
    }
}
