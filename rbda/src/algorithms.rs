use crate::model::Model;
use crate::sva::{Motion, Xform};

pub fn articulated_body(model: &mut Model) {
    // Loop 1: calculates joint velocities, transforms and inertial properties.
    for joint_index in 0..model.joints.len() {
        let (bottom, top) = model.joints.split_at_mut(joint_index);
        let jnt = top.get_mut(0).unwrap();

        // should be done in joint update?
        jnt.xj = Xform::rotx(jnt.q);
        jnt.vj = jnt.qd * jnt.s;
        jnt.xl = jnt.xj * jnt.xt;

        match jnt.parent {
            None => {
                jnt.x = jnt.xl;
                jnt.v = jnt.vj;
            }
            Some(parent_index) => {
                let prnt = bottom.get_mut(parent_index).unwrap();
                jnt.x = jnt.xl * prnt.x;
                jnt.v = (jnt.xl * prnt.v) + jnt.vj;
            }
        }
        jnt.c = jnt.v.cross_v(jnt.vj);
        jnt.iaa = jnt.i.into();
        jnt.paa = jnt.v.cross_f(jnt.i * jnt.v);
    }
    // Calculate external and joint forces
    // todo

    // Apply External Force
    // for joint_index in (0..model.joints.len()).rev() {
    //     let joint = model.joints.get(joint_index).unwrap();
    //     js.paa = js.paa - joint.x.inverse() * &joint.f_ext;
    // }

    // Loop 2
    for joint_index in (0..model.joints.len()).rev() {
        let (bottom, top) = model.joints.split_at_mut(joint_index);
        let jnt = top.get_mut(0).unwrap();

        jnt.uu = jnt.iaa * jnt.s; // (6x6) * (6xn) =  (6xn)
        jnt.dd = jnt.s.w.dot(&jnt.uu.m) + jnt.s.v.dot(&jnt.uu.f); // (nx6) * (6xn) =  (nxn)
        jnt.u = jnt.tau - (jnt.s.w.dot(&jnt.paa.m) + jnt.s.v.dot(&jnt.paa.f));

        match jnt.parent {
            None => {}
            Some(parent_index) => {
                let prnt = bottom.get_mut(parent_index).unwrap();
                // let ia = &jnt.iaa - u * d^-1 * u.T; (6xn) * (nxn) * (n*6) = 6x6
                let dd_inv = 1. / jnt.dd;
                let ia = jnt.iaa - (dd_inv * jnt.uu.self_outer_product());
                let pa = jnt.paa + (ia * jnt.c) + ((dd_inv * jnt.u) * jnt.uu);
                let xli = jnt.xl.inverse();
                prnt.iaa += xli * ia;
                prnt.paa += xli * pa;
            }
        }
    }

    // Loop 3:
    for joint_index in 0..model.joints.len() {
        let (bottom, top) = model.joints.split_at_mut(joint_index);
        let jnt = top.get_mut(0).unwrap();

        let ap: Motion;
        match jnt.parent {
            None => {
                ap = jnt.xl * model.base.a + jnt.c;
            }
            Some(parent_index) => {
                let prnt = bottom.get_mut(parent_index).unwrap();
                ap = jnt.xl * prnt.a + jnt.c;
            }
        };

        let dd_inv = 1. / jnt.dd;
        let te = jnt.u - (jnt.uu.m.dot(&ap.w) + jnt.uu.f.dot(&ap.v));
        jnt.qdd = dd_inv * te;
        jnt.a = ap + (jnt.qdd * jnt.s)
    }
}
