use nannou_osc::{Message, Type};
use visgen_graph::{
    ParameterEnd, ParameterEndpoint, ParameterFactory, ParameterHandle, ParameterStore,
};

#[test]
fn test_integration_parameters() {
    let mut store = ParameterStore::new();
    let mut endpoints: Vec<ParameterEndpoint<f32>> = Vec::new();

    {
        // lifetime of the factory (it keeps a &mut of the store)
        let mut factory = ParameterFactory::new("group_a".to_string(), &mut store);
        endpoints.push(factory.build("a".to_string()));
        endpoints.push(factory.build("b".to_string()));
        endpoints.push(factory.build("c".to_string()));
        endpoints.push(factory.build("d".to_string()));
        factory.path("group_b".to_string());
        endpoints.push(factory.build("a".to_string()));
        endpoints.push(factory.build("b".to_string()));
        endpoints.push(factory.build("c".to_string()));
        endpoints.push(factory.build("d".to_string()));
    }

    // all parameters == default
    assert!(endpoints
        .iter()
        .all(|f| { f.get(&store) == f32::default() }));

    //construct some OSC-Message
    let end_i: usize = 1;
    let value = 0.12345;
    let addr_a = "group_a/b".to_string();
    let mut msg = Message {
        addr: addr_a.clone(),
        args: Some(vec![Type::Float(value)]),
    };

    // all parameters == default
    assert!(endpoints
        .iter()
        .all(|f| { f.get(&store) == f32::default() }));
    store.update(&msg);
    assert_eq!(
        store.get_path_value(&addr_a).unwrap()[0]
            .clone()
            .float()
            .unwrap(),
        value
    );
    assert_eq!(endpoints[end_i].get(&store), value);

    msg.addr = "group_b/c".to_string();
    store.update(&msg);

    {
        //Use handles to make access to the store transparent
        let handles: Vec<ParameterHandle<f32>> = endpoints.iter().map(|f| f.bind(&store)).collect();

        for hand in handles {
            let val: f32 = hand.into();
            println!("{}", val)
        }
    } //Drop handles to recover borrowed refs

    msg.addr = "group_b/c".to_string();
    store.update(&msg);
}
