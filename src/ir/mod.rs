mod to_rules;

#[derive(Debug)]
/// IR error information
pub struct Error(pub(crate) String);

#[derive(Debug)]
/// IR error information
pub struct IR {
    pos: usize,
    commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Command(String);

impl IR {
    pub(crate) fn new(txt: &str) -> Self {
        Self {
            pos: 0,
            commands: txt
                .lines()
                .map(|l| Command(l.to_string()))
                .collect::<Vec<_>>(),
        }
    }

    fn get(mut self) -> Result<(IR, Command), Error> {
        // dbg!(self.commands[self.pos].clone());
        if self.pos >= self.commands.len() {
            Err(Error("next over finished program".to_string()))
        } else {
            let cmd = self.commands[self.pos].clone();
            self.pos += 1;
            Ok((self, cmd))
        }
    }

    fn peek(&self) -> Option<Command> {
        self.commands.get(self.pos).map(|c| c.clone())
    }

    fn consume(self, val: &str) -> Result<IR, Error> {
        let (ir, cmd) = self.get()?;
        if cmd.0 == val {
            Ok(ir)
        } else {
            Err(Error(format!("expected {}, received {}", val, cmd.0)))
        }
    }
}
