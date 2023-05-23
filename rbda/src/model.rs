use crate::{algorithms::articulated_body, joints::Joint};
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug)]
pub struct Model {
    pub base: Joint,
    pub joints: Vec<Joint>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub q: Vec<f32>,
    pub qd: Vec<f32>,
}

impl Model {
    pub fn new(base: Joint, joints: Vec<Joint>) -> Self {
        Self { base, joints }
    }

    pub fn get_state(&self) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for joint in &self.joints {
            q.push(joint.q);
            qd.push(joint.qd);
        }
        State { q, qd }
    }

    pub fn set_state(&mut self, state: State) {
        for (joint, q) in self.joints.iter_mut().zip(state.q.iter()) {
            joint.q = *q;
        }
        for (joint, qd) in self.joints.iter_mut().zip(state.qd.iter()) {
            joint.qd = *qd;
        }
    }

    pub fn get_dstate(&self) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for joint in &self.joints {
            q.push(joint.qd);
            qd.push(joint.qdd);
        }
        State { q, qd }
    }

    pub fn integrate_state(&mut self, dt: f32) {
        for joint in &mut self.joints {
            joint.q = joint.q + dt * joint.qd;
            joint.qd = joint.qd + dt * joint.qdd;
        }
    }

    pub fn update(&mut self) {
        articulated_body(self)
    }
}

// implement ops::Add for State
impl Add for State {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for (q1, q2) in self.q.iter().zip(rhs.q.iter()) {
            q.push(q1 + q2);
        }
        for (qd1, qd2) in self.qd.iter().zip(rhs.qd.iter()) {
            qd.push(qd1 + qd2);
        }

        State { q, qd }
    }
}

// implement ops::Add for borrowed State
impl Add<State> for &State {
    type Output = State;

    fn add(self, rhs: State) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for (q1, q2) in self.q.iter().zip(rhs.q.iter()) {
            q.push(q1 + q2);
        }
        for (qd1, qd2) in self.qd.iter().zip(rhs.qd.iter()) {
            qd.push(qd1 + qd2);
        }

        State { q, qd }
    }
}

// implement ops::Mul for borrowed State and f32
impl Mul<f32> for &State {
    type Output = State;

    fn mul(self, rhs: f32) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for q1 in self.q.iter() {
            q.push(q1 * rhs);
        }
        for qd1 in self.qd.iter() {
            qd.push(qd1 * rhs);
        }
        State { q, qd }
    }
}

// implement ops::AddAssign for State
impl AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        for (q1, q2) in self.q.iter_mut().zip(rhs.q.iter()) {
            *q1 += q2;
        }
        for (qd1, qd2) in self.qd.iter_mut().zip(rhs.qd.iter()) {
            *qd1 += qd2;
        }
    }
}

// implement ops::Mul for State for and f32
impl Mul<f32> for State {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for q1 in self.q.iter() {
            q.push(q1 * rhs);
        }
        for qd1 in self.qd.iter() {
            qd.push(qd1 * rhs);
        }
        State { q, qd }
    }
}

// implement ops::Mul for f32 for and state
impl Mul<State> for f32 {
    type Output = State;

    fn mul(self, rhs: State) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for q1 in rhs.q.iter() {
            q.push(q1 * self);
        }
        for qd1 in rhs.qd.iter() {
            qd.push(qd1 * self);
        }
        State { q, qd }
    }
}

// implement ops::Mul for f32 and borrowed State
impl Mul<&State> for f32 {
    type Output = State;

    fn mul(self, rhs: &State) -> State {
        let mut q = Vec::new();
        let mut qd = Vec::new();
        for q1 in rhs.q.iter() {
            q.push(q1 * self);
        }
        for qd1 in rhs.qd.iter() {
            qd.push(qd1 * self);
        }
        State { q, qd }
    }
}
