use std::fs::File;
use std::io::{BufWriter};
use std::io::prelude::*;
use std::f32;

fn maxIndex(vec: &Vec<f32>) -> usize
{
    let mut max = f32::NEG_INFINITY;
    let mut ind = 0;
    for i in 0..vec.len()
    {
        if vec[i] > max
        {
            ind = i;
            max = vec[i];
        }
    }
    ind
}

pub fn writeOutput(map: &Vec<Vec<char>>, qValues: &Vec<Vec<Vec<f32>>>)
{
    let mut q = File::create("q.txt").unwrap();
    let mut pi = File::create("pi.txt").unwrap();

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            let maxInd = maxIndex(&qValues[i][j]);
            let dir = 
            {
                if maxInd == 0
                {
                    "acima"
                }
                else if maxInd == 1
                {
                    "direita"
                }
                else if maxInd == 2
                {
                    "abaixo"
                }
                else
                {
                    "esquerda"
                }
            };
            let sign = 
            {
                if maxInd == 0
                {
                    '^'
                }
                else if maxInd == 1
                {
                    '>'
                }
                else if maxInd == 2
                {
                    'v'
                }
                else
                {
                    '<'
                }
            };

            if(map[i][j] == '-')
            {
                write!(&mut q, "{},{},{},{}\n", i, j, dir, qValues[i][j][maxInd]);
                write!(&mut pi, "{}", sign);
            }
            else
            {
                write!(&mut pi, "{}", map[i][j]);
            }
        }

        write!(&mut pi, "\n");
    }

    q.sync_all();
}
