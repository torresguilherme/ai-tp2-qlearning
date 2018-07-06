extern crate rand;
use qlearning::rand::Rng;

enum sides
{
    UP,
    RIGHT,
    DOWN,
    LEFT
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

pub fn learn(map: &Vec<Vec<char>>, qValues: &Vec<Vec<Vec<f32>>>,
    learnRate: &f32, discount: &f32, posX: &mut usize, posY: &mut usize)
{
}
