use core::num;
use std::array;
use std::io;
use std::io::Read;
use std::ptr::null;
use std::string;
use std::vec;

fn main() {
    let mut lista: Vec<String> = Vec::new();
    let mut listaposicao: Vec<String> = Vec::new();

    loop {
        let mut _index: usize = 0;

        let mut opc = String::new();

        println!("\nOpções:\n1. Nova tarefa: \n2. Completar tarefa: \n3. Excluir tarefa: \n4. Listar Tarefas: \n\n");

        io::stdin()
            .read_line(&mut opc)
            .expect("Erro ao selecionar opção!!!");

        let mut tarefa = String::new();

        let opc: u32 = opc.trim().parse().expect("Erro ao coverter 'opc' ");

        if opc == 1 {
            let mut numm = 0;

            println!("\nAdicione uma nova tarefa: \n");

            io::stdin()
                .read_line(&mut tarefa)
                .expect("Erro ao inserir nova tarefa!!");

            if tarefa == "" {
                break;
            }

            let tarefa: String = tarefa.trim().parse().expect("");
            let not = "Não concluida";

            let notstr = not.to_string();

            lista.push(tarefa);
            listaposicao.push(notstr);

            println!("_____________________________________________________");
        }

        if opc == 2 {
            let mut id_tarefa = String::new();

            let mut num = 1;

            let mut verifica = String::new();

            println!("\nSelecione uma tarefa para completar: \n");

            for z in &lista {
                println!("{}. {}", num, z);

                num += 1;
            }

            io::stdin()
                .read_line(&mut id_tarefa)
                .expect("Erro ao selecionar tarefa para a alteração!!!");

            let id_tarefa: usize = id_tarefa.trim().parse().expect("msg");

            let _index: usize = id_tarefa - 1;

            println!("\nSelecione uma ação: Concluir (1), Voltar(2)\n");

            io::stdin().read_line(&mut verifica).expect("msg");

            if listaposicao[_index] == lista[_index] {
                println!("\nTarefa escolhida: {tarefa}");
            }
            let verifica: usize = verifica.trim().parse().expect("msg");

            if verifica == 1 {
                println!("Tarefa : '{}' concluida!", lista[_index]);

                if let Some(elem) = listaposicao.get_mut(_index) {
                    let concluido = "Concluido";

                    *elem = concluido.to_string();
                }
            } else {
                continue;
            };

            println!("_____________________________________________________");
        }

        if opc == 3 {
            let mut id_tarefa = String::new();

            let mut num = 1;

            println!("\nSelecione um Item: \n");

            for e in &lista {
                println!("{}. {}", num, e);

                num += 1;
            }

            io::stdin()
                .read_line(&mut id_tarefa)
                .expect("Erro ao selecionar tarefa para a exclusao!!!");

            let id_tarefa: usize = id_tarefa.trim().parse().expect("msg");

            let _index: usize = id_tarefa - 1;

            lista.remove(_index);
            listaposicao.remove(_index);
            println!("_____________________________________________________");
        }

        if opc == 4 {
            let mut _index = 0;

            let mut num = 1;

            for ex in &lista {
                println!("{}. {}", num, ex);

                num += 1;
            }

            for e in &listaposicao {
                println!("{},{}", &lista[_index], e);

                _index += 1;
            }
            println!("_____________________________________________________");
        }
    }
}
