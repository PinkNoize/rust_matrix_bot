use matrix_api::matrix_api;

pub enum Net {
    Matrix(matrix_api),
}

impl Net {
    fn send_message(&self,message: String) {
        match *self {
            Net::Matrix(ref api) => {
                api.send(message);
            }
        }
    }

    fn receive_message(&mut self) -> String {
        let mut ret=String::new();
        match *self {
            Net::Matrix(ref mut api) => {
                while 
                    match api.receive() {
                        Some(cmd)=> {
                            ret+=&cmd; 
                            false
                        },
                        None => true 
                    }
                {
                    //nop
                }
            }
        };
        ret
    }
}

