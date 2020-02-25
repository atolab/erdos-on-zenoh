#[macro_use]
extern crate erdos;

use erdos::dataflow::operators::{JoinOperator, SourceOperator};
use erdos::node::Node;
use erdos::{Configuration, OperatorConfig};

fn main() {
    let args = erdos::new_app("ERDOS").get_matches();
    let mut node = Node::new(Configuration::from_args(&args));
    let s1 = connect_1_write!(
        SourceOperator,
        OperatorConfig::new().name("SourceOperator1")
    );
    let s2 = connect_1_write!(
        SourceOperator,
        OperatorConfig::new().name("SourceOperator2")
    );
    let _s3 = connect_1_write!(JoinOperator<usize, usize, usize>, OperatorConfig::new().name("JoinOperator"), s1, s2);

    node.run();
}