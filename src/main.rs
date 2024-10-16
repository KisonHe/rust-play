pub struct VehicleParam {
    #[arg(default_value="hexcan0".to_string())]
    pub can_device: String,
    pub calc_odom_from_speed: bool,
    pub max_speed: Option<i64>,
}

fn foo() -> HashMap<String, HexValue>{
    Default::default()
}

fn main(){
    // Can we auto gen this?
    let param_config = vec![
        "can_device".to_string(),
        "calc_odom_from_speed".to_string(),
        "max_speed".to_string(),
    ];

    // Magic foo function to get the hashmap containing the param info
    let hmap = foo();

    let vehicle_param = VehicleParam {
        can_device: if let Some(HexValue::String(s)) = hmap.get("can_device") {
            println!("{}", t!("param-set-to", param="can_device", value=s.clone()).color("green"));
            s.clone()
        } else {
            // Args says this value has default value, so we can set it
            let default_can_device = "hexcan0".to_string();
            eprintln!("{}", t!("param-warning-using-default", param="can_device", value=default_can_device.clone()).color("yellow"));
            default_can_device
        },
        calc_odom_from_speed: if let Some(HexValue::Bool(i)) = hmap.get("calc_odom_from_speed") {
            println!("{}", t!("param-set-to", param="calc_odom_from_speed", value=i).color("green"));
            i
        } else {
            panic!("{}", format!("{:?}", t!("required-param-missing", param="calc_odom_from_speed")));
        },
        // This is a optional value, so normal if gettting it failed
        max_speed: if let Some(HexValue::Int64(i)) = hmap.get("max_speed") {
            Some(i)
        } else {
            None
        },
    };
}
