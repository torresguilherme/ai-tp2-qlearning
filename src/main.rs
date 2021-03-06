use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;
use std::io::{BufRead, BufReader};
mod qlearning;
mod output;

fn main()
{
    let args: Vec<String> = env::args().collect();

    // parse args
    let inputName = &args[1];
    let mut learnRate: f32 = args[2].parse().unwrap();
    let mut discount: f32 = args[3].parse().unwrap();
    let iterations: u32 = args[4].parse().unwrap();

    // open file
    let mut reader = BufReader::new(File::open(inputName).unwrap());

    // get number of lines and columns
    let mut numLine = String::new();
    reader.read_line(&mut numLine);
    let sizes: Vec<usize> = numLine.split(char::is_whitespace)
        .take(2)
        .map(|n| n.parse().unwrap())
        .collect();

    // get map
    let map: Vec<Vec<char>> = reader.lines()
        .take(sizes[0])
        .map(|l| l.unwrap().chars()
             .take(sizes[1])
             .collect())
        .collect();

    // init q-values
    let mut qValues: Vec<Vec<Vec<f32>>> = vec![];
    for i in 0..sizes[0]
    {
        qValues.push(vec![]);
        for j in 0..sizes[1]
        {
            qValues[i].push(vec![]);
            for k in 0..4
            {
                qValues[i][j].push(0.0);
            }
        }
    }

    // inicia posicoes com valores aleatorios
    let mut posX: usize = 0;
    let mut posY: usize = 0;
    qlearning::generateRandomPos(&map, &mut posX, &mut posY);

    for i in 0..iterations
    {
        // atualiza o valor para a posicao e acao atual
        qlearning::learn(&map, &mut qValues, &learnRate, &discount, &mut posX, &mut posY);
        // se for um terminal, reposiciona o agente aleatoriamente
        if(map[posX][posY] == '&' || map[posX][posY] == '0')
        {
            qlearning::generateRandomPos(&map, &mut posX, &mut posY);
        }
    }

    // imprime os resultados em um arquivo de saida
    output::writeOutput(&map, &qValues);
}
