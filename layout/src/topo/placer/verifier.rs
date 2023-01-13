use crate::core::geometry::do_boxes_intersect;
use crate::topo::layout::VisualGraph;

pub fn do_it(vg: &mut VisualGraph) {
    #[cfg(not(feature = "no_verify"))]
    verify_order_in_rank(vg);
}

#[cfg(not(feature = "no_verify"))]
fn verify_order_in_rank(vg: &mut VisualGraph) {
    for row in 0..vg.dag.num_levels() {
        let current_row = vg.dag.row(row);
        let num_elements = current_row.len();
        if num_elements == 0 {
            continue;
        }

        let mut node_iter = current_row.iter().copied();
        let first_node = node_iter.next().unwrap();

        for curr_node in node_iter {
            let bb0 = vg.pos(first_node).bbox(true);
            let bb1 = vg.pos(curr_node).bbox(true);
            assert!(!do_boxes_intersect(bb0, bb1), "Boxes must not intersect");
            assert!(
                bb0.0.x < bb1.0.x,
                "The order of the boxes must be sequential on the x axis"
            );
        }
    }
}
