use std::io::stdin;

#[derive(Debug)]
enum VisitorAction{
    Accept,
    AcceptWithNote {note:String},
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor{
    name:String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    //Constructor
    // fn new(name: &str, greeting: &str) -> Self{
    //     Self { name: name.to_lowercase(), greeting: greeting.to_string() }
    // }

    fn new(name: &str, action: VisitorAction, age: i8) -> Self{
        Self { name: name.to_string(), action, age}
    }
    
    //Associated function = method
    // fn greet_visitor(&self){
    //     println!("{}", self.greeting)
    // }

    fn greet_visitor(&self){
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("Note: , {}", note);
                if self.age < 21{
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a
            probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
    let mut visitor_list = vec![
        Visitor::new("Eduardo", VisitorAction::Accept, 26),
        Visitor::new("Leonardo",VisitorAction::AcceptWithNote 
            {
                note: String::from("Pouco alcool")
            }, 25),
        Visitor::new("Giselma", VisitorAction::Refuse, 50),
        Visitor::new("Damaris", VisitorAction::AcceptWithNote 
            { 
                note: String::from("coloca vinho")
            }, 25),
    ];

    loop {
        println!("Qual e o seu nome?");
        let name = qual_seu_nome();
        
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty(){
                    break;
                }else{
                    println!("The name {} isn't on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }

        break;
    }
    
    println!("The final list of visitors:");
    println!("{:?}", visitor_list);
        
}

//http://media.pragprog.com/titles/hwrust/code/FirstStepsWithRust/treehouse_guestlist_enum/src/main.rs