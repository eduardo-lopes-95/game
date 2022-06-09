use std::io::stdin;

struct Visitor{
    name:String,
    greeting: String,
}

impl Visitor {
    //Constructor
    fn new(name: &str, greeting: &str) -> Self{
        Self { name: name.to_lowercase(), greeting: greeting.to_string() }
    }
    
    //Associated function = method
    fn greet_visitor(&self){
        println!("{}", self.greeting)
    }
}

fn friend_on_the_list(name: &str) -> bool{
    let friends = ["eduardo", "leonardo", "gil", "damaris"];
    let result:bool = friends.contains(&name);
    result
}

fn qual_seu_nome() -> String{
    let mut nome = String::new();
    stdin()
    .read_line(&mut nome)
    .expect("Failed to read line");
    nome
        .trim()
        .to_lowercase()
}

fn main() {
    println!("Qual e o seu nome?");
    
    let name = qual_seu_nome();
    
    let visitor_list = [
        Visitor::new("Eduardo", "Fala pica"),
        Visitor::new("Leonardo", "Fala irmão"),
        Visitor::new("Giselma", "Tudo bem, mamãe?"),
        Visitor::new("Damaris", "Sr. Sabino"),
    ];
    
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);
    
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry pal, your name isn't on the lis. Get out of here!")
    };
}
