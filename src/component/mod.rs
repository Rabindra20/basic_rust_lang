mod component {
    // Items in modules default to private visibility.
    // fn private_function() {
    //     println!("called `my_mod::private_function()`");
    // }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `component::function()`");
    }
}