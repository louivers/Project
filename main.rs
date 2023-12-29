mod algorithms;
mod models;
mod examples_testing;
#[allow(unused_imports)]
use examples_testing::globally_consistent_db::globally_consistent_db;
use examples_testing::naturaljoin_test::naturaljoin_test;
#[allow(unused_imports)]
use crate::examples_testing::order_test::order_test;
#[allow(unused_imports)]
use crate::examples_testing::join_tree_test::join_tree_test;
#[allow(unused_imports)]
use crate::examples_testing::full_reducer_test::full_reducer_test;

fn main() {
    //join_tree_test();
    //order_test();
    //full_reducer_test();
    //globally_consistent_db();
    naturaljoin_test();
}
