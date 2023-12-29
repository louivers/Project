mod algorithms;
mod models;
mod examples_testing;
mod util;
#[allow(unused_imports)]
use examples_testing::database_loader_test::loader_test;
#[allow(unused_imports)]
use examples_testing::globally_consistent_db::globally_consistent_db;
#[allow(unused_imports)]
use examples_testing::naturaljoin_test::naturaljoin_test;
#[allow(unused_imports)]
use crate::examples_testing::order_test::order_test;
#[allow(unused_imports)]
use crate::examples_testing::join_tree_test::join_tree_test;
#[allow(unused_imports)]
use crate::examples_testing::full_reducer_test::full_reducer_test;
#[allow(unused_imports)]
use crate::examples_testing::yannakakis_test::yannakakis_test;
#[allow(unused_imports)]
use crate::examples_testing::acyclic_queries_test::test_queries;

fn main() {
    //join_tree_test();
    //order_test();
    //full_reducer_test();
    // globally_consistent_db();
    // naturaljoin_test();
    // loader_test();
    //yannakakis_test();
    match test_queries() {
        Ok(_) => println!("All tests passed!"),
        Err(e) => println!("Error: {:?}", e),
    }
}
