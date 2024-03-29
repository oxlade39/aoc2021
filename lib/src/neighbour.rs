use crate::cartesian::{Plane, Point, Transform, Vector};

pub trait Neighbours<T> {
    fn neighbours(&self, p: &T) -> Vec<Point>;
}

pub struct DirectNeighbours<'a>(pub &'a Plane);
pub struct TouchingNeighbours<'a>(pub &'a Plane);

impl<'a> Neighbours<Point> for TouchingNeighbours<'_> {
    fn neighbours(&self, p: &Point) -> Vec<Point> {
        [
            (-1, 0),
            (1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ]
        .map(|t| {
            let into: Transform = t.into();
            into
        })
        .into_iter()
        .filter_map(|t| {
            let transformed = p.transform(&t);
            if transformed.within(self.0) {
                Some(transformed)
            } else {
                None
            }
        })
        .collect()
    }
}

impl Neighbours<Vector> for TouchingNeighbours<'_> {
    fn neighbours(&self, v: &Vector) -> Vec<Point> {
        if v.start.x != v.end.x && v.start.y != v.end.y {
            panic!("non deterministic neighbours, vector must share an axis");
        }

        let mut n = Vec::new();

        if v.start.y == v.end.y {
            let min_x = v.start.x.min(v.end.x);
            let max_x = v.start.x.max(v.end.x);
            let y = v.start.y;

            for x in min_x..=max_x {
                if x == min_x {
                    let prospective_points = [
                        Point { x: x - 1, y: y + 1 },
                        Point { x: x - 1, y },
                        Point { x: x - 1, y: y - 1 },
                    ];
                    for p in prospective_points {
                        if p.within(self.0) {
                            n.push(p);
                        } else {
                            println!("{:?} not within {:?}", p, self.0);
                        }
                    }
                }

                if x == max_x {
                    let prospective_points = [
                        Point { x: x + 1, y: y + 1 },
                        Point { x: x + 1, y },
                        Point { x: x + 1, y: y - 1 },
                    ];
                    for p in prospective_points {
                        if p.within(self.0) {
                            n.push(p);
                        } else {
                            println!("{:?} not within {:?}", p, self.0);
                        }
                    }
                }

                let prospective_points = [Point { x, y: y + 1 }, Point { x, y: y - 1 }];
                for p in prospective_points {
                    if p.within(self.0) {
                        n.push(p);
                    } else {
                        println!("{:?} not within {:?}", p, self.0);
                    }
                }
            }
        } else {
            let min_y = v.start.y.min(v.end.y);
            let max_y = v.start.y.max(v.end.y);
            let x = v.start.x;
            for y in min_y..=max_y {
                if y == min_y {
                    let prospective_points = [
                        Point { x: x - 1, y: y - 1 },
                        Point { x, y: y - 1 },
                        Point { x: x + 1, y: y - 1 },
                    ];
                    for p in prospective_points {
                        if p.within(self.0) {
                            n.push(p);
                        }
                    }
                }

                if y == max_y {
                    let prospective_points = [
                        Point { x: x - 1, y: y + 1 },
                        Point { x: x, y: y + 1 },
                        Point { x: x + 1, y: y + 1 },
                    ];
                    for p in prospective_points {
                        if p.within(self.0) {
                            n.push(p);
                        }
                    }
                }

                let prospective_points = [Point { x: x - 1, y }, Point { x: x + 1, y }];
                for p in prospective_points {
                    if p.within(self.0) {
                        n.push(p);
                    }
                }
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        cartesian::{Plane, Point, Vector},
        neighbour::{Neighbours, TouchingNeighbours},
    };

    #[test]
    fn test_vector_touching_neighbours_right_along_y() {
        // *..
        // *..
        // *..
        let plane = Plane {
            top_left: (0, 2).into(),
            bottom_right: (2, 0).into(),
        };
        let vector = Vector {
            start: (0, 0).into(),
            end: (0, 2).into(),
        };
        let neighbours = TouchingNeighbours(&plane);

        let expected: HashSet<Point> = [(1, 0).into(), (1, 1).into(), (1, 2).into()]
            .into_iter()
            .collect();
        let actual: HashSet<Point> = neighbours.neighbours(&vector).into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vector_touching_neighbours_centre_along_y() {
        // .*.
        // .*.
        // .*.
        let plane = Plane {
            top_left: (0, 2).into(),
            bottom_right: (2, 0).into(),
        };
        let vector = Vector {
            start: (1, 0).into(),
            end: (1, 2).into(),
        };
        let neighbours = TouchingNeighbours(&plane);

        let expected: HashSet<Point> = [
            (0, 0).into(),
            (0, 1).into(),
            (0, 2).into(),
            (2, 0).into(),
            (2, 1).into(),
            (2, 2).into(),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<Point> = neighbours.neighbours(&vector).into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vector_touching_neighbours_diagonal_below_along_y() {
        // .*.
        // .*.
        // ...
        let plane = Plane {
            top_left: (0, 2).into(),
            bottom_right: (2, 0).into(),
        };
        let vector = Vector {
            start: (1, 1).into(),
            end: (1, 2).into(),
        };
        let neighbours = TouchingNeighbours(&plane);

        let expected: HashSet<Point> = [
            (0, 0).into(),
            (0, 1).into(),
            (0, 2).into(),
            (2, 0).into(),
            (2, 1).into(),
            (2, 2).into(),
            (1, 0).into(),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<Point> = neighbours.neighbours(&vector).into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vector_touching_neighbours_diagonal_above_along_y() {
        // ...
        // .*.
        // .*.
        let plane = Plane {
            top_left: (0, 2).into(),
            bottom_right: (2, 0).into(),
        };
        let vector = Vector {
            start: (1, 0).into(),
            end: (1, 1).into(),
        };
        let neighbours = TouchingNeighbours(&plane);

        let expected: HashSet<Point> = [
            (0, 0).into(),
            (0, 1).into(),
            (0, 2).into(),
            (2, 0).into(),
            (2, 1).into(),
            (2, 2).into(),
            (1, 2).into(),
        ]
        .into_iter()
        .collect();
        let actual: HashSet<Point> = neighbours.neighbours(&vector).into_iter().collect();

        assert_eq!(expected, actual);
    }
}
