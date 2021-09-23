witx_bindgen_rust::export!("witx/component.witx");
witx_bindgen_rust::import!("witx/host.witx");

struct Component;

impl component::Component for Component {
    fn run() {
        let r = host::next();
        println!("{:?}", r);
        host::emit(&r);
    }
}
