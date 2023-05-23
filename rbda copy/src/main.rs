use std::time::Instant;

use rbda::joints::Joint;
use rbda::mesh::{BoxMesh, Mesh};
use rbda::model::Model;
use rbda::sva::{Inertia, Matrix, Motion, Vector, Xform};

fn main() {
    let base = Joint::base(Motion::new([0., 0., 9.81], [0., 0., 0.]));

    // first pendulum link
    let inertia_0 = Inertia::new(
        1.,
        Vector::new(0., 0., -0.5),
        (1. / 12.) * Matrix::from_diagonal(&Vector::new(1., 1., 0.04)),
    );
    let xt_0 = Xform::identity();
    let mut rx_0 = Joint::rx(None, inertia_0, xt_0);
    rx_0.meshes = vec![Mesh::Box(BoxMesh::new(-0.1, 0.1, -0.1, 0.1, -1., 0.))];
    rx_0.q = 90.0_f32.to_radians();

    let inertia_1 = Inertia::new(
        1.,
        Vector::new(0., 0., -0.5),
        (1. / 12.) * Matrix::from_diagonal(&Vector::new(1., 1., 0.04)),
    );
    let xt_1 = Xform::new(Vector::new(-0., 0., -1.), Matrix::identity());
    let mut rx_1 = Joint::rx(Some(0), inertia_1, xt_1);
    rx_1.meshes = vec![Mesh::Box(BoxMesh::new(-0.1, 0.1, -0.1, 0.1, -1., 0.))];

    let joint_array = vec![rx_0, rx_1];

    let mut model = Model::new(base, joint_array);

    let start = Instant::now();
    let mut solution = Vec::new();

    let dt = 0.001;

    for _ in 0..100_000_00 {
        model.update();
        model.integrate_state(dt);
        solution.push(model.get_state());
    }
    let duration = start.elapsed();
    println!("Time elapsed simulating is: {:?}", duration);
}
