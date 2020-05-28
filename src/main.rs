use std::io;

struct Numbers {
    x: i64,
    y: i64
}

impl Numbers {

    fn soma (&self) -> i64{
        self.x + self.y
    }

    fn subtrac (&self) -> i64{
        self.x - self.y
    }

    fn multip (&self) -> i64{
        self.x * self.y
    }

    fn divi (&self) -> i64{
        self.x / self.y
    }

}

enum MathOptions {
    Soma,
    Subtrac,
    Multip,
    Divi,
    Quit,
    Invalid,
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    return input
}

fn numbers_create () -> Numbers {
    println!("Insira o primeiro numero: ");
    let num1 = user_input();
    println!("Insira o segundo numero: ");
    let num2 = user_input();
    let numbers = Numbers {x: num1.trim().parse().expect("Inteiro"), y: num2.trim().parse().expect("Inteiro")};

    return numbers
}


fn main() {
    

    loop {

        println!("Escolha a operação que deseja fazer: \n
        1 - soma\n
        2 - subtração\n
        3 - multiplicação\n
        4 - divisão\n
        5 - sair");

        let opt = user_input();

        let opt: i32 = opt.trim().parse().expect("Inteiro");

        let opt_name = match opt {
            1 => MathOptions::Soma,
            2 => MathOptions::Subtrac,
            3 => MathOptions::Multip,
            4 => MathOptions::Divi,
            5 => MathOptions::Quit,
            _ => MathOptions::Invalid,

        };

        match opt_name {
            MathOptions::Soma => {
                let numbers = numbers_create();
                println!("Resultado soma {}", numbers.soma());
            },
            MathOptions::Subtrac => {
                let numbers = numbers_create();
                println!("Resultado subtração {}", numbers.subtrac());
            },
            MathOptions::Multip => {
                let numbers = numbers_create();
                println!("Resultado multiplicação {}", numbers.multip());
            },
            MathOptions::Divi => {
                let numbers = numbers_create();
                println!("Resultado divisão {}", numbers.divi());
            },
            MathOptions::Quit => {
                println!("Encerrando...");
                break;
            },
            MathOptions::Invalid => {
                println!("Selecione uma operação valida");
            },
        };
        
    }

}
