use std::io::stdin;

const ROOM_DESCRIPTIONS: [&str; 10]=["Scary place... You see a door.","Here is a table and a small lamp on it. Also you can see four doors.","There is more scary than in the place where you woke up.","You see a small trails of water on walls, some puddles around","At every corner of this room you see lamps. Strange.","Scarlet walls, scarlet floor...","Yellow symbolizes betrayals","This green room fills you with nostalgia.","This corridor is too narrow. You feel not comfortable.","You are not able to read it LOL"];
const KEY_DESCRIPTIONS: [&str; 3] = ["This key is made with bones!.","Just a black key.","This key is made with pure gold, you surely can become rich if take it out."];

struct Room{
    id: u8,
    room_name: String,
    connections: [Option<u8>; 4],
    is_locked: bool,
    description: String,

    items: Vec<Key>
}
#[derive(PartialEq)]
struct Key{
    id: u8,
    name: String,
    description: String
}

struct Inventory{
    keys: Vec<Key>
}
impl Inventory{
    fn new(keys: Vec<Key>) -> Self{
        Self{
            keys
        }
    }
}


struct State{
    position: u8
}

impl Room{
    fn unlock(&mut self, key: &Key){
        if self.id == key.id {
            self.is_locked = false
        } else {
            println!("Doors are closed")
        }

    }
    fn check_room(&self){
        println!("You decided to look around. {}", self.description)
    }
    fn search(&mut self) -> Vec<Key>{
        println!("You tried to find something here.");
        if self.items.is_empty(){
            println!("Here is nothing");
            Vec::new()
        } else {
            for item in &self.items{
                println!("You found a {}", item.name);
                println!("{}", item.description);
            }
            self.items.drain(..).collect()
        }
    }
    fn new(id: u8, room_name: &str, north: Option<u8>, west: Option<u8>, east: Option<u8>, south: Option<u8>, is_locked: bool, description: &str, items:  Vec<Key>) -> Self {
        Self{
            id,
            room_name: room_name.to_string(),
            connections: [north, west, east, south],
            is_locked,
            description: description.to_string(),
            items
        }
    }
}

impl Key {
    fn new(id: u8, name: &str, description: &str) -> Self {
        Self{
            id,
            name: name.to_string(),
            description: description.to_string()
        }
    }
}
impl State{
    fn change_room(&mut self, current_position: usize, side: usize, rooms: &mut [Room], inventory: &Inventory){
        let current_room = &rooms[current_position];
        let num = current_room.connections[side].unwrap_or(0);
        if current_room.connections[side].is_none() {
            println!("Here is no way")
        } else if rooms[num as usize].is_locked {
                println!("The doors are locked.");
            let pos_id = current_room.connections[side].unwrap_or(0);
            let index = inventory.keys.iter().find(|u| u.id == pos_id);
                    match index {
                        Some(key) => {
                            println!("The {} opened the doors", key.name);
                            rooms[num as usize].unlock(key)  ;
                            self.change_room(current_position, side, rooms, inventory)
                        },
                        None => println!("Find a key for these doors")
                    }

        } else { self.position = num }
    }
    fn new(position: u8)-> Self{
        Self{
            position
        }
    }
    fn process_commands(commands: Commands, rooms: &mut [Room], state: &mut State, inventory: &mut Inventory) {
        let current_room = state.position as usize;

        match commands {
            Commands::North => state.change_room(current_room, 0, rooms, &inventory),
            Commands::West  => state.change_room(current_room, 1, rooms, &inventory),
            Commands::East  => state.change_room(current_room, 2, rooms, &inventory),
            Commands::South => state.change_room(current_room, 3, rooms, &inventory),
            Commands::Search => {
                let found_items = rooms[current_room].search();
                for item in found_items {inventory.keys.push(item);}
            }
            Commands::LookAround => rooms[current_room].check_room(),
            Commands::Help => println!("Commands: North, West, East, South, Search, Look, Help"),
        }
    }
}

enum Commands{
    North,
    West,
    East,
    South,
    Search,
    LookAround,
    Help
}
impl Commands {
    fn from_input(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "north" | "n" => Some(Commands::North),
            "west"  | "w" => Some(Commands::West),
            "east"  | "e" => Some(Commands::East),
            "south" | "s" => Some(Commands::South),
            "search"      => Some(Commands::Search),
            "look"        => Some(Commands::LookAround),
            "help"  | "h" => Some(Commands::Help),
            _ => None,
        }
    }
}


fn main() {
    let mut rooms: [Room; 10] = [
        Room::new(0, "Start room", Some(1), None, None, None, false, ROOM_DESCRIPTIONS[0], Vec::new()),
        Room::new(1, "First room", Some(2), Some(5), Some(6), Some(0), false, ROOM_DESCRIPTIONS[1], Vec::new()),
        Room::new(2, "Gloomy room", Some(3), None, None, Some(1), true, ROOM_DESCRIPTIONS[2], Vec::new()),
        Room::new(3, "Wet room", Some(4), Some(7), Some(8), Some(2), false, ROOM_DESCRIPTIONS[3], Vec::new()),
        Room::new(4, "Bright room", None, None, None, Some(3), false, ROOM_DESCRIPTIONS[4], Vec::new()),
        Room::new(5, "Red room", None, None, Some(1), None, true, ROOM_DESCRIPTIONS[5], Vec::new()),
        Room::new(6, "Yellow room", None, Some(1), None, None, false, ROOM_DESCRIPTIONS[6], Vec::new()),
        Room::new(7, "Green room", None, None, Some(3), None, false, ROOM_DESCRIPTIONS[7], Vec::new()),
        Room::new(8, "Corridor", None, Some(3), None, Some(9), false, ROOM_DESCRIPTIONS[8], Vec::new()),
        Room::new(9, "Final room", None, Some(8), None, None, true, ROOM_DESCRIPTIONS[9], Vec::new()),
    ];
    let mut players_inventory = Inventory::new(Vec::new());


       let key1 = Key::new(5, "White key", KEY_DESCRIPTIONS[0]);
       let key2 = Key::new(2, "Black key", KEY_DESCRIPTIONS[1]);
       let key3 = Key::new(9, "Golden key", KEY_DESCRIPTIONS[2]);

    rooms[6].items.push(key1);
    rooms[5].items.push(key2);
    rooms[7].items.push(key3);


    let mut state= State::new(0);

    println!("You woke up at the strange room. There is a small light from above. Definitely it is a cave.\nAfter little rest you decided to leave this cave.");
    println!("Use commands like North, West, East, South to move. Type 'Search' to find any objects and 'Look' to look around the room");

    loop {

        println!("Your current position is: {}", rooms[state.position as usize].room_name);
        println!("possible directions:");
            if !rooms[state.position as usize].connections[0].is_none() { println!("North") }
            if !rooms[state.position as usize].connections[1].is_none() { println!("West") }
            if !rooms[state.position as usize].connections[2].is_none() { println!("East") }
            if !rooms[state.position as usize].connections[3].is_none() { println!("South") }
        let input = get_input("Type the command. If you forget any, type 'Help'");
        if let Some(command) = Commands::from_input(&input) {
            State::process_commands(command, &mut rooms, &mut state, &mut players_inventory);
        } else {
            println!("Invalid command");
        }
        if state.position == 9{
            println!("You found the exit from this cave, you win");
            break
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
