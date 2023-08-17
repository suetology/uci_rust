use rust_uci::Uci;

fn main() {
    let mut uci = Uci::new()?;
    
    let server = uci.get("openvpn.myserver")?;
    println!("{}", server);
}
