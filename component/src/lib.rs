witx_bindgen_rust::export!("witx/component.witx");
witx_bindgen_rust::import!("witx/host.witx");

struct Component;

impl component::Component for Component {
    fn run() {
        let r = host::next();
        println!("{:?}", r);
        host::emit(&r);
    }

    fn process(r: component::Input) -> Vec<component::Output> {
        let mut out = Vec::new();
        out.push(component::Output {
            tag: "english".to_string(),
            translated: r.body,
        });
        out
    }
}
