use itertools::Itertools;
use serde::*;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Transform {
    Translate { x: f32, y: f32 },
    RotateX(f32),
    RotateY(f32),
    RotateZ(f32),
}

impl Transform {
    pub fn get_transform_string(ts: &Vec<Transform>, unit: &String) -> String {
        if ts.is_empty() {
            "".to_string()
        } else {
            "transform: ".to_string() + &ts.iter().map(|x| x.make_string(unit)).join(" ") + ";"
        }
    }

    pub fn combine_transforms<T: Iterator<Item = Transform>>(ts: T) -> Vec<Transform> {
        let mut v = Vec::<Transform>::new();

        for t in ts {
            if let Some(last) = v.last() {
                match last.try_add(t) {
                    Some(sum) => {
                        v.pop();
                        if !sum.is_empty() {
                            v.push(sum);
                        }
                    }
                    None => v.push(t),
                }
            } else {
                v.push(t);
            }
        }

        v
    }

    pub fn try_add(self, rhs: Self) -> Option<Self> {
        match self {
            Transform::Translate { x: x1, y: y1 } => match rhs {
                Transform::Translate { x: x2, y: y2 } => Self::Translate {
                    x: x1 + x2,
                    y: y1 + y2,
                }
                .into(),
                _ => None,
            },
            Transform::RotateX(a1) => match rhs {
                Transform::RotateX(a2) => Self::RotateX(a1 + a2).into(),
                _ => None,
            },
            Transform::RotateY(a1) => match rhs {
                Transform::RotateY(a2) => Self::RotateY(a1 + a2).into(),
                _ => None,
            },
            Transform::RotateZ(a1) => match rhs {
                Transform::RotateZ(a2) => Self::RotateZ(a1 + a2).into(),
                _ => None,
            },
        }
    }

    pub fn is_empty(self) -> bool {
        match self {
            Transform::Translate { x, y } => x == 0.0 && y == 0.0,
            Transform::RotateX(a) => a == 0.0,
            Transform::RotateY(a) => a == 0.0,
            Transform::RotateZ(a) => a == 0.0,
        }
    }

    pub fn make_string(&self, unit: &String) -> String {
        if self.is_empty() {
            return "".to_string();
        }

        match self {
            Transform::Translate { x, y } => {
                if x == &0.0 {
                    return format!("translateY({:.2}{})", y, unit);
                } else if y == &0.0 {
                    return format!("translateX({:.2}{})", x, unit);
                }
                format!(
                    "translate({x:.2}{unit}, {y:.2}{unit})",
                    x = x,
                    y = y,
                    unit = unit
                );
            }
            Transform::RotateX(a) => format!("rotateX({:.2}deg)", a),
            Transform::RotateY(a) => format!("rotateY({:.2}deg)", a),
            Transform::RotateZ(a) => format!("rotateZ({:.2}deg)", a),
        }
    }
}
