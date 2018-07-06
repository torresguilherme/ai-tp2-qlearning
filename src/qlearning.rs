extern crate rand;
use qlearning::rand::Rng;
use std::f32;

fn max(vec: &Vec<f32>) -> f32
{
    let mut ret = f32::NEG_INFINITY;
    for i in 0..vec.len()
    {
        if vec[i] > ret
        {
            ret = vec[i];
        }
    }
    ret
}

fn reward(block: &char) -> f32
{
    if(*block == '0')
    {
        10.0
    }
    else if(*block == '&')
    {
        -10.0
    }
    else
    {
        -1.0
    }
}

pub fn generateRandomPos(map: &Vec<Vec<char>>, posX: &mut usize, posY: &mut usize)
{
    let mut randomX = rand::thread_rng().gen_range(0, map.len());
    let mut randomY = rand::thread_rng().gen_range(0, map[0].len());
    while(map[randomX][randomY] != '-')
    {
        randomY += 1;
        if(randomY >= map[0].len())
        {
            randomY = 0;
            randomX += 1;
            if(randomX >= map.len())
            {
                randomX = 0;
            }
        }
    }

    *posX = randomX;
    *posY = randomY;
}

pub fn learn(map: &Vec<Vec<char>>, qValues: &mut Vec<Vec<Vec<f32>>>,
    learnRate: &f32, discount: &f32, posX: &mut usize, posY: &mut usize)
{
    // escolhe acao
    let mut act = 0;
    for i in 0..4
    {
        if(qValues[*posX][*posY][i] > qValues[*posX][*posY][act])
        {
            act = i;
        }
    }
    // atualiza o valor de q:
    // q(s, a) <- (1-a)q(s, a) + a(r + c*max q(s', a'))
    if(act == 0)
    {
        qValues[*posX][*posY][act] = (1.0 - learnRate) * qValues[*posX][*posY][act] + (learnRate 
            * (reward(&map[*posX - 1][*posY]) + discount * (max(&qValues[*posX - 1][*posY]))));
        if map[*posX - 1][*posY] != '#'
        {
            *posX -= 1;
        }
    }
    else if(act == 1)
    {
        qValues[*posX][*posY][act] = (1.0 - learnRate) * qValues[*posX][*posY][act] + (learnRate 
            * (reward(&map[*posX][*posY + 1]) + discount * (max(&qValues[*posX][*posY + 1]))));
        if map[*posX][*posY + 1] != '#'
        {
            *posY += 1;
        }
    }
    else if(act == 2)
    {
        qValues[*posX][*posY][act] = (1.0 - learnRate) * qValues[*posX][*posY][act] + (learnRate 
            * (reward(&map[*posX + 1][*posY]) + discount * (max(&qValues[*posX + 1][*posY]))));
        if map[*posX + 1][*posY] != '#'
        {
            *posX += 1;
        }
    }
    else
    {
        qValues[*posX][*posY][act] = (1.0 - learnRate) * qValues[*posX][*posY][act] + (learnRate 
            * (reward(&map[*posX][*posY - 1]) + discount * (max(&qValues[*posX][*posY - 1]))));
        if map[*posX][*posY - 1] != '#'
        {
            *posY -= 1;
        }
    }
}
