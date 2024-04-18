use std::io;

fn main() {
    let mut matrix: Vec<char> = vec!['0', '0', '0', '0', '0', '0', '0', '0', '0'];
    let mut op;
    let mut iterations = 0;
    let mut marc;
    let mut marcado = 0;

    loop{
        if marcado==9{
            println!("Deu velha");
            break;
        }
        if iterations%2==0 {
            println!("Vez do JOGADOR X!!");
            marc = 'X';
        }else{
            println!("Vez do JOGADOR O!!");
            marc = 'O';
        }

        plot_matrix(&matrix);
        op = read_keyboard();
        if (op< 1) || (op > 9) || (op==10){
            println!("Digite um número entre 1 e 9");
        }else{
            if !update_matrix(op as usize, &mut matrix,marc) {
                println!("Este campo já foi marcado");
                iterations-=1;
            }else{
                marcado+=1;
            }
        }
        if ganhou(&mut matrix, op as usize, marc){
            plot_matrix(&matrix);
            println!("{} ganhou",marc);
            break;
        }
        iterations+=1;
    }

}

fn read_keyboard() -> i32{
    println!("Digite um número:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    
    let input = input.trim();
    // Converte a string em um número
    let numero: i32 = match input.parse() {
        Ok(numero) => numero,
        Err(_) => {
            println!("Erro: Por favor, digite um número válido.");
            10// Encerra o programa se não for fornecido um número válido
        }
    };
    numero
}

fn update_matrix(i: usize, matrix: &mut [char], marcador: char ) -> bool {
    if  matrix[i-1] == '0' {
        matrix[i-1] = marcador;
    }else{
        return false;
    }
    return true;
}

fn plot_matrix(matrix: &[char]) {
    for (i, &elemento) in matrix.iter().enumerate() {
        print!("{} ", elemento);
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
    println!("------");
}

fn ganhou(matrix: &[char], i:usize,op:char) -> bool{ //Shadowing
    if i==1{
        if matrix[0] == op && matrix[1]==op && matrix[2]==op{
            return true;
        }
        if matrix[0] == op && matrix[4]==op && matrix[8]==op{
            return true;
        }
    }else if i==2{
        if matrix[0] == op && matrix[1]==op && matrix[2]==op{
            return true;
        }
        if matrix[1] == op && matrix[4]==op && matrix[7]==op{
            return true;
        }
    }else if i==3{
        if matrix[0] == op && matrix[1]==op && matrix[2]==op{
            return true;
        }
        if matrix[6] == op && matrix[4]==op && matrix[2]==op{
            return true;
        }
        if matrix[2] == op && matrix[5]==op && matrix[8]==op {
            return true;
        }
    }else if i==4{
        if matrix[3] == op && matrix[4]==op && matrix[5]==op{
            return true;
        }
        if matrix[0] == op && matrix[3]==op && matrix[6]==op{
            return true;
        }
    }else if i==5{
        if matrix[3] == op && matrix[4]==op && matrix[5]==op{
            return true;
        }
        if matrix[0] == op && matrix[3]==op && matrix[6]==op{
            return true;
        }
        if matrix[0] == op && matrix[4]==op && matrix[8]==op{
            return true;
        }
        if matrix[1] == op && matrix[4]==op && matrix[7]==op{
            return true;
        }
    }else if i==6{
        if matrix[3] == op && matrix[4]==op && matrix[5]==op{
            return true;
        }
        if matrix[2] == op && matrix[5]==op && matrix[8]==op{
            return true;
        }
    }else if i==7{
        if matrix[6] == op && matrix[7]==op && matrix[8]==op{
            return true;
        }
        if matrix[6] == op && matrix[4]==op && matrix[2]==op {
            return true;
        }
        if matrix[0] == op && matrix[3]==op && matrix[6]==op{
            return true;
        }
    }else if i==8{
        if matrix[6] == op && matrix[7]==op && matrix[8]==op{
            return true;
        }
        if matrix[1] == op && matrix[4]==op && matrix[7]==op{
            return true;
        }
    }else if i==9{
        if matrix[6] == op && matrix[7]==op && matrix[8]==op{
            return true;
        }
        if matrix[0] == op && matrix[4]==op && matrix[8]==op{
            return true;
        }
        if matrix[2] == op && matrix[5]==op && matrix[8]==op{
            return true;
        }
    }
    return false;
}