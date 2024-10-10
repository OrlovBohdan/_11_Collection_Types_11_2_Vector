#[test]

/*
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    // FILL in the blank
    let v: __= vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
*/

fn main() {
    // Заполняем вектор типом Box<dyn IpAddr>
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display(); // Вызов метода display для каждого IP адреса
    }
}

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}



/*
Box<dyn IpAddr>: Это позволяет хранить объекты разных структур, реализующих трейты IpAddr, в одном векторе. Использование Box позволяет обойтись с динамическими размерами, что в свою очередь необходимо для хранения различных типов, реализующих один и тот же трейт.

for ip in v: Перебираем все элементы вектора и вызываем метод display для каждого из них, что печатает IP-адреса в соответствующем формате.
*/