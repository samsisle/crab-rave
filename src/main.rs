#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

impl URL {
    fn toString(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }
    fn from(url: &str) -> URL {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]);
        let pathname = String::from(vec2[1]);
        URL {
            protocol,
            hostname,
            pathname,
        }
    }
}

fn main() {
    let app = URL::from("https://app.rust-for-js.dev/posts/07-structs-and-methods/");
    println!("{:?}", app);
}
