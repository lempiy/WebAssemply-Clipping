#![feature(proc_macro)]

extern crate poly;

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;

use poly::clip;
use stdweb::js_export;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

js_deserializable!(Point);
js_serializable!(Point);

#[js_export]
fn get_clip(poly: Vec<Point>, clipper: Vec<Point>) -> Vec<Vec<Point>> {
    let pol = clip::Polygon {
        points: poly.into_iter().map(|p| {
            clip::Point{x: p.x, y: p.y}
        }).collect(),
    };
    let cli = clip::Polygon {
        points: clipper.into_iter().map(|p| {
            clip::Point{x: p.x, y: p.y}
        }).collect(),
    };
    if let Some(plg) = pol.clip(&cli) {
        plg
            .into_iter()
            .map(|vct| {
                vct
                    .into_iter()
                    .map(|p| {
                        Point{x: p.x, y: p.y}
                    }).collect()
            }).collect()
    } else {
        vec![]
    }
}
