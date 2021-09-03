witx_bindgen_rust::export!("../spec/component.witx");

struct Component;

impl component::Component for Component {
    fn consume_add(lh: i32, rh: i32) -> i32 {
        lh + rh
    }

    fn moo(s : String) -> String {
        format!("{}{}", s, "moo")
    }
}