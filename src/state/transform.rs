use std::ops::Add;

use itertools::Itertools;
use serde::*;

#[derive(PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct TransformTranslate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TransformComponent{
    Translate(TransformTranslate),
    Rotate(TransformRotate)
}




impl TransformComponent{

    pub fn get_transform_string(ts: &Vec<TransformComponent>)->String{
        if ts.is_empty(){
            "".to_string()
        }else {
           "transform: ".to_string() + &ts.iter().map(|x|x.to_string()).join(" ") + ";"
        }
    }

    pub fn combine_transforms<T: Iterator<Item =TransformComponent>>(ts: T) -> Vec<TransformComponent>{
        let mut v = Vec::<TransformComponent>::new();
     
         for t in ts{
             if let Some(last) = v.last(){
                 match last.try_add(t) {
                     Some(sum) => {
                         v.pop();
                         if !sum.is_empty(){
                             v.push(sum);
                         }
                     },
                     None => v.push(t),
                 }
             }
             else{
                 v.push(t);
             }
         }
     
        v
     }

    pub fn try_add(self, rhs: Self)->  Option<Self>{
        match self{
            TransformComponent::Translate(t1) => match rhs{
                TransformComponent::Translate(t2) => Self::Translate(t1 + t2).into(),
                _ => None,
            },
            TransformComponent::Rotate(r1) => match rhs{
                TransformComponent::Rotate(r2) => Self::Rotate(r1 + r2).into(),
                _ => None,
                
            },
        }
    }

    pub fn is_empty(self)-> bool{
        match self{
            TransformComponent::Translate(t) =>  t == Default::default(),
            TransformComponent::Rotate(r) => r == Default::default(),
        }
    }

}

impl ToString for TransformComponent{
    fn to_string(&self) -> String {
        match self {
            TransformComponent::Translate(t) => t.to_string(),
            TransformComponent::Rotate(r) => r.to_string(),
        }
    }
}

impl From<TransformTranslate> for TransformComponent {
    fn from(x: TransformTranslate) -> Self {
        Self::Translate(x)
    }
}

impl From<TransformRotate> for TransformComponent {
    fn from(x: TransformRotate) -> Self {
        Self::Rotate(x)
    }
}

impl ToString for TransformTranslate{
    fn to_string(&self) -> String {

        if self == &Default::default(){
            return "".to_string();
        }

        format!("translate3d({:.2}vw,{:.2}vw,{:.2}vw)", self.x, self.y, self.z)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct TransformRotate {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Add for TransformTranslate{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for TransformRotate{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ToString for TransformRotate{
    fn to_string(&self) -> String {
        if self.x != 0 {
            if self.y != 0 {
                format!("rotateX({:.2}deg) rotateY({:.2}deg)", self.x, self.y)
            } else {
                format!("rotateX({:.2}deg)", self.x)
            }
        } else if self.y != 0 {
            format!("rotateY({:.2}deg)", self.y)
        } else {
            "".to_string()
        }
    }
}
