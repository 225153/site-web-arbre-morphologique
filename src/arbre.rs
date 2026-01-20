
pub struct Derive {
    pub mot: String,
    pub schema: String,
}

pub struct RacineNode {
    pub racine: [char; 3],              
    pub left: Option<Box<RacineNode>>,  
    pub right: Option<Box<RacineNode>>, 
}
pub struct Tree{Option<Box<RacineNode>>};

impl Tree {
    pub fn new(racine : char)-> Self {


    }
}
