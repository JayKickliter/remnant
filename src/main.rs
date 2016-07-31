extern crate remnant;
extern crate copperline;

use remnant::remnantdb;

fn help() {
    println!("exit - quit");
    println!("help - list this help information");
    println!("list - list all entries in the database");
}

fn list(db: &remnantdb::RemnantDB) {
    for entry in db.iter() {
        println!("{}", entry);
    }
}

fn main() {
    let mut r = remnantdb::RemnantDB::new();
    let root = r.create_str("console");
    let mut anchor = root;

    let mut cl = copperline::Copperline::new();
    while let Ok(line) = cl.read_line_default(format!("{} ", anchor).as_ref()) {
        match line.trim() {
            "help" => help(),
            "exit" => break,
            "list" => list(&r),
            "" => (),
            s => {
                anchor = r.append_str(&anchor, s);
                println!("! {}", r.len());
            }
        }
        cl.add_history(line);
    }
}
