fn serve_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }

    fn cook_order() {}
}

fn main() {
    crate::back_of_house::fix_incorrect_order();
}
