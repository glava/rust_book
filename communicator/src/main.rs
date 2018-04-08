extern crate communicator;

use communicator::outermost::inside;

fn main() {
    inside::inner_function();
    communicator::client::connect();
}