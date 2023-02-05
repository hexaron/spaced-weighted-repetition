#[derive(Debug)]
pub struct Problem {
    from: String,
    to: String,
    history: Vec<bool>,
    spacing: u32,
}

impl Problem {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            history: Vec::new(),
            spacing: 0,
        }
    }

    pub fn all_new() -> Vec<Self> {
        vec![Self::new("a", "1")]
    }

    pub fn state(&mut self) {
        self.spacing = 0;

        println!("{}", self.from);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input.trim() == self.to {
            println!("Correct!");

            self.history.push(true);
        } else {
            println!("Not correct!");

            self.history.push(false);
        }
    }
}
