mod traits;

use geo::{Coordinate, MultiPoint, Point, Polygon, GeometryCollection};
use geo::concave_hull::ConcaveHull;
use geo::convex_hull::ConvexHull;
use geo::algorithm::is_convex::IsConvex;

pub struct GeointTool {

}

impl traits::GpTool<bool> for GeointTool {

    fn execute(&self) -> bool {
        true
    }
}

impl traits::GpTool<bool> for Polygon<f64> {

    fn execute(&self) -> bool {
        let concavity = 2.0;
        self.concave_hull(concavity);
        true
    }
}



/// Concave hull implementation
fn concave_hull_points(points: Vec<Point<f64>>) -> Polygon<f64> {
    let multi_point = MultiPoint(points);
    let concavity = 2.0;
    multi_point.concave_hull(concavity)
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::traits::*;

    #[test]
    fn it_works() {
        let tool = GeointTool {};
        let executed = tool.execute();
        assert_eq!(true, executed);
    }

    #[test]
    fn multipoint_test() {
        let points = vec![
            Point::new(-102.130629, 48.251542),
            Point::new(-102.07351, 48.3250820000001),
            Point::new(-101.20723, 46.253469),
            Point::new(-103.328183, 46.243274),
            Point::new(-101.20723, 46.253469)
        ];

        let hull = concave_hull_points(points);
        assert_eq!(true, hull.exterior().is_convex());
    }
}
