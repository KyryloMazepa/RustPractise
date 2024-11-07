const WIDTH: usize = 11;  
const HEIGHT: usize = 11; 

fn main() {
    
    let mut envelope = vec![vec![' '; WIDTH]; HEIGHT];

    
    for i in 0..WIDTH {
        envelope[0][i] = '*';        
        envelope[HEIGHT - 1][i] = '*';   
    }
    for i in 0..HEIGHT {
        envelope[i][0] = '*';           
        envelope[i][WIDTH - 1] = '*';   
    }

    
    for i in 0..HEIGHT {
        envelope[i][i] = '*';                 
        envelope[i][WIDTH - 1 - i] = '*';      
    }

    
    for row in envelope {
        for ch in row {
            print!("{}", ch);   
        }
        println!();             
    }
}