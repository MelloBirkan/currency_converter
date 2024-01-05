use std::io;

#[derive(Debug)]
struct Currency {
    taxa_conversao: f64,
    nome: String,
}

impl Currency {
    fn new(taxa_conversao: f64, nome: String) -> Currency {
        Currency {
            taxa_conversao,
            nome,
        }
    }

    fn converter(&self, valor: f64) {
        let result = self.taxa_conversao * valor;
        println!("{} {} = {:.3} BRL", valor, self.nome, result);
    }
}

fn get_input(label: &str) -> String {
    let mut input = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");
    input.trim().to_string()
}

fn add_currency(moedas: &mut Vec<Currency>) {
    let name = get_input("Entre o nome da moeda: ");
    let mut conversion_rate = get_input("Entre com a taixa de conversão da moeda: ");
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
    while menu_input != "sair" {
        menu_input.clear();
        menu_input = get_input(
            "Bem-vindo ao conversor de moedas!\nDigite:\n'adicionar' para adicionar uma moeda\n\
        'converter' para converter uma moeda de sua escolha para o real\n'sair' para sair",
        );
        match menu_input.as_str() {
            "add" => add_currency(&mut moedas),
            "converter" => {
                let moeda = get_input("Digite a moeda que deseja converter: ");
                let quantidade = get_input("Digite a quantidade de moeda que deseja converter: ");
                moedas
                    .iter()
                    .find(|&x| x.nome == moeda.to_lowercase())
                    .unwrap()
                    .converter(quantidade.parse::<f64>().unwrap());
            }
            "sair" => println!("Saindo..."),
            _ => println!("Opção inválida"),
        }
    }
}
