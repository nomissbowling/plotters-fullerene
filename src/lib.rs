#![doc(html_root_url = "https://docs.rs/plotters-fullerene/0.1.4")]
//! plotters fullerene and polyhedron for Rust
//!

use plotters::prelude::*;
use plotters::coord::Shift;

use num::Float;

use ph_faces::{PHF, TUV, f_to_f64};
use ph_faces::tetra::*;
use ph_faces::cube::*;
use ph_faces::octa::*;
use ph_faces::sphere::*;
use ph_faces::cylinder::*;
use ph_faces::capsule::*;
use ph_faces::cone::*;
use ph_faces::torus::*;
use ph_faces::pipe::*;
use ph_faces::polyhedron; // polyhedron::pin::Pin
use ph_faces::revolution::*;
use fullerene::Icosahedron;
use fullerene::{Dodecahedron, DodecahedronCenter};
use fullerene::{C60, C60Center};

/// mk chart
#[macro_export]
macro_rules! mk_chart {
  ($rt: ident, $name: expr) => {{
    let mut chart = ChartBuilder::on($rt)
      .margin(20)
      .caption($name, ("sans-serif", 40))
      .build_cartesian_3d(-3.0..3.0, -3.0..3.0, -3.0..3.0)
      .unwrap();
    chart.with_projection(|mut pb| {
      pb.pitch = 0.5; // 0.7; // 1.2;
      pb.yaw = 0.2; // 0.7; // 0.5;
      pb.scale = 0.7; // 0.7;
      pb.into_matrix()
    });
    chart.configure_axes().draw().unwrap();
    chart
  }}
}

/// lines
pub fn lines<DB: DrawingBackend>(rt: &DrawingArea<DB, Shift>, name: &str) {
  let mut chart = mk_chart!(rt, name);
  chart.draw_series(
    LineSeries::new(
      (-100..100).map(|y| y as f64 / 100.0).map(|y|
        ((y * 10.0).sin(), y, (y * 10.0).cos())
      ),
      &RED
    )
  ).unwrap();
}

/// waves mesh
pub fn waves<DB: DrawingBackend>(rt: &DrawingArea<DB, Shift>, name: &str) {
  let mut chart = mk_chart!(rt, name);
  chart.draw_series(
    SurfaceSeries::xoz(
      (-25..25).map(|v| v as f64 / 10.0),
      (-25..25).map(|v| v as f64 / 10.0),
      |x: f64, z: f64| (x * x + z * z).cos()
    ).style(&BLUE.mix(0.2))
  ).unwrap();
}

/// waves3d quad polygons
pub fn waves3d<DB: DrawingBackend>(rt: &DrawingArea<DB, Shift>, name: &str) {
  let mut chart = mk_chart!(rt, name);
  let mut d = vec![];
  for x in (-25..25).map(|v| v as f64 / 10.0) {
    let mut row = vec![];
    for z in (-25..25).map(|v| v as f64 / 10.0) {
      row.push((x, (x * x + z * z).cos(), z));
    }
    d.push(row);
  }
  chart.draw_series(
    (0..49).map(|x| std::iter::repeat(x).zip(0..49))
    .flatten()
    .map(|(x, z)| {
      Polygon::new(vec![
        d[x][z],
        d[x+1][z],
        d[x+1][z+1],
        d[x][z+1]
      ], &BLUE.mix(0.3))
    })
  ).unwrap();
}

/// triangles
pub fn triangles<DB: DrawingBackend>(rt: &DrawingArea<DB, Shift>, name: &str) {
  let mut chart = mk_chart!(rt, name);
  chart.draw_series(
    (0..3).into_iter().flat_map(|k|
      (0..4).into_iter().flat_map(move |j| // move for borrow k
        (0..5).into_iter().map(|i|
          Polygon::new(vec![
            (-1.0 + k as f64, -2.0 + i as f64, -1.5 + j as f64),
            (-1.0 + k as f64, -2.0 + i as f64, -0.5 + j as f64),
            (k as f64, -2.0 + i as f64, -1.5 + j as f64)
          ], &BLUE.mix(0.3)) // must be vec of tuple
        ).collect::<Vec<_>>()
      )
    )
  ).unwrap();
}

/// surface3d
pub fn surface3d<F: Float, DB: DrawingBackend>(rt: &DrawingArea<DB, Shift>,
  name: &str, phf: PHF<F>) {
  let mut chart = mk_chart!(rt, name);
  chart.draw_series(
    phf.iter().flat_map(|f|
      f.iter().map(|t| {
        let vtx = t.iter().map(|v| {
          let (p, _uv) = v.puv();
          let fp = f_to_f64(p); // expected f64
          (fp[0], fp[1], fp[2]) // must be vec of tuple
        }).collect::<Vec<_>>();
        Polygon::new(vtx, &BLUE.mix(0.3))
      }).collect::<Vec<_>>()
    )
  ).unwrap();
}

/// create_png
pub fn create_png() {
  let fname = "./images/polyhedron.png";
  let rt = BitMapBackend::new(fname, (1920, 1280)).into_drawing_area();
  rt.fill(&WHITE).unwrap();
  let da = rt.split_evenly((4, 6));

  lines(&da[0], "spiral"); // dummy
  waves(&da[6], "waves"); // dummy
  waves3d(&da[12], "waves3d"); // dummy
  triangles(&da[18], "triangles"); // dummy

  let tetra = Tetra::<f64>::new(1.0);
  surface3d(&da[1], "Tetra", tetra.ph.with_uv(false));
  let cube = Cube::<f64>::new(1.0);
  surface3d(&da[2], "Cube", cube.ph.with_uv(false));
  let cubec = CubeCenter::<f64>::new(1.0);
  surface3d(&da[3], "CubeCenter", cubec.ph.with_uv(false));
  let octa = Octa::<f64>::new(1.0);
  surface3d(&da[4], "Octa", octa.ph.with_uv(false));

  let revo = Revolution::<f64>::new(1.0, 9, 6, (true, true),
    |n: u16, m: u16| -> (f64, f64) {
    (-2.25 + 4.5 * n as f64 / m as f64, 1.0) // -2.25 to 2.125 (q = 9, m = 36)
  });
  surface3d(&da[5], "Revolution", revo.ph.with_uv(false));

  let rsphere = RSphere::<f64>::new(1.0, 6);
  surface3d(&da[7], "RSphere", rsphere.ph.with_uv(false));
  let cylinder = Cylinder::<f64>::new(1.0, 4.0, 6);
  surface3d(&da[8], "Cylinder", cylinder.ph.with_uv(false));
  let capsule = Capsule::<f64>::new(1.0, 3.0, 6);
  surface3d(&da[9], "Capsule", capsule.ph.with_uv(false));
  let cone = Cone::<f64>::new(1.0, 4.0, 6);
  surface3d(&da[10], "Cone", cone.ph.with_uv(false));

  let q = 9;
  let s = q * 2 + 1;
  let tbl = (0..s).into_iter().map(|sn| { // -2.25 to 2.25 (q = 9)
    (-2.25 + 4.5 * sn as f64 / (s - 1) as f64, 1.0)
  }).collect::<Vec<_>>();
  let revo_tbl = Revolution::<f64>::from_tbl(1.0, q, 6, (true, true), &tbl);
  surface3d(&da[11], "Revo Table", revo_tbl.ph.with_uv(false));

  let torus24 = Torus::<f64>::new(2.0, 0.8, 6, 6);
  surface3d(&da[13], "Torus24", torus24.ph.with_uv(false));
/*
  let rtorus24 = RTorus::<f64>::new(2.0, 0.8, 12, 6);
  surface3d(&da[14], "RTorus24", rtorus24.ph.with_uv(false));
*/
  let ring24 = Ring::<f64>::new(2.0, 0.1, 0.8, 12, 6);
  surface3d(&da[14], "Ring24", ring24.ph.with_uv(false));
  let tube = Tube::<f64>::new(2.0, 1.6, 4.0, 6);
  surface3d(&da[15], "Tube", tube.ph.with_uv(false));
  let halfpipe = HalfPipe::<f64>::new(4.712388980, 2.4, 2.0, 4.0, 6); // 3pi/2
  surface3d(&da[16], "HalfPipe", halfpipe.ph.with_uv(false));
  let pin = polyhedron::pin::Pin::<f64>::new(0.4, 8, 6);
  surface3d(&da[17], "Pin", pin.ph.with_uv(false));

  let icosa = Icosahedron::<f64>::new(1.0);
  surface3d(&da[19], "Icosahedron", icosa.ph.with_uv(false));
  let dodeca = Dodecahedron::<f64>::new(1.0);
  surface3d(&da[20], "Dodecahedron", dodeca.ph.with_uv(false));
  let dodecac = DodecahedronCenter::<f64>::new(1.0);
  surface3d(&da[21], "DodecahedronCenter", dodecac.ph.with_uv(false));
  let c60 = C60::<f64>::new(1.0);
  surface3d(&da[22], "C60", c60.ph.with_uv(false));
  let c60c = C60Center::<f64>::new(1.0);
  surface3d(&da[23], "C60Center", c60c.ph.with_uv(false));
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_plot() {
    assert_eq!(create_png(), ());
  }
}
