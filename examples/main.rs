use ph_region::region::Region;

fn main() {
    // Display regions as code to name key-value pairs
    println!("{:?}", Region::list());

    // Display regions as name to code key-value pairs
    println!("{:?}", Region::list_by_full_name());
}