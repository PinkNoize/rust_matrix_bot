use command::Command;
use std::collections::HashMap;
    
pub struct Shell<'a>{
    env: HashMap<&'a str, String>,
    commands: HashMap<&'a str,Command<'a>>,
}

impl<'a> Shell<'a>{
    pub fn new() -> Shell<'a> {
    let mut cmds = HashMap::new();
    cmds.insert("help",Command {keyword:"help",
                                help:"Usage: help [command]\nDisplays information about commands.\n",
                                func:help});
    cmds.insert("exit",Command {keyword:"exit",
                                help:"Usage: exit\nKills the bot.\n",
                                func:exit});
    Shell {
        env: HashMap::new(),
        commands: cmds
        }
    }

    pub fn process_command(&self,message: String) -> String {
        let mut args=Vec::new();
        args.extend(message.split_whitespace());
        match self.commands.get(&args[0]) {
            Some(cmd) => (cmd.func)(args,&self.env,&self.commands),
            None => args[0].to_owned()+" not found."
        }
    }

    pub fn load_commands(&mut self,commands: Vec<Command<'a>>) {
        for cmd in commands {
            self.commands.insert(&cmd.keyword,cmd);
        }
    }

}
fn help(args: Vec<&str>, env: &HashMap<&str, String>, commands: &HashMap<&str,Command>) -> String {
    let mut result=String::new();
    if args.len() > 1 {
        for index in 1..args.len() {
            let length=result.len();
            match commands.get(&args[index]) {
                Some(cmd) => result.insert_str(length,&cmd.help),
                None => result.insert_str(length,&(args[index].to_owned()+" not found\n"))
            }
        }
    }
    else {
        for (cmdkey,_) in commands.iter() {
            result+=cmdkey;
            result+="\n";
        }
    }
    result
}
fn exit(args: Vec<&str>, env: &HashMap<&str, String>, commands: &HashMap<&str,Command>) -> String{
    String::new()
}
