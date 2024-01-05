use std::io;

#[derive(Debug)]
struct Currency {
    taxa_conversao: f64,
    nome: String,
}

impl Currency {
    fn new(taxa_conversao: f64, nome: String) -> Currency {
        Currency { taxa_conversao, nome }
    }

    fn converter(&self, valor: f64) -> f64 {
        let result = self.taxa_conversao * valor;
        println!("{} {} = {} BRL", valor, self.nome, result);
        result
    }
}

fn get_input(label: &str) -> String {
    let mut input = String::new();
    println!("{}", label);
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    input.trim().to_string()
}

fn add_currency(moedas: &mut Vec<Currency>) {
    let name = get_input("Enter the currency name: ");
    let mut conversion_rate = get_input("Enter the currency conversion rate: ");
    if let Some(rate) = conversion_rate.parse::<f64>().ok() {
        moedas.push(Currency::new(rate, name));
    } else {
        println!("Invalid conversion rate");
    }
}

fn main() {
    let mut moedas: Vec<Currency> = Vec::new();

    // Menu
    let mut menu_input = String::new();
    while menu_input != "stop" {
        menu_input.clear();

        match menu_input.as_str() {
            "add" => add_currency(&mut moedas),
            "stop" => println!("Saindo..."),
            _ => println!("Opção inválida"),
        }
        menu_input = get_input("Bem-vindo ao conversor de moedas!\n Digite 'add' para adicionar uma moeda\n \
        'stop' para sair\n 'converter' para converter uma moeda de sua escolha para o real:");
    }

    add_currency(&mut moedas);
    println!("Moedas: {:?}", moedas);
    moedas.iter().find(|&x| x.nome == "Dollar".to_lowercase()).unwrap().converter(10.0);
}
