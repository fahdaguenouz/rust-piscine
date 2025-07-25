pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {

    if horizontal('X',table)||vertical('X',table)||diagonal('X',table){
        "player X won".to_string()
    }else if horizontal('O',table)||vertical('O',table)||diagonal('O',table) {
        "player O won".to_string()
    }else {
         "tie".to_string()
    }

}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {

    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {


    for  row in table.iter() {
        // println!(" {}: {:?}", i, row);
        let mut count = 0;
        for  cell in row.iter() {
            // println!(" ({}, {}): {:?}", i, j, cell);
            if *cell==player{
                count+=1;
            }
            if count==3{
               return  true;
            }
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
   for col in 0..3 {
        let mut count = 0;
        for row in 0..3 {
            if table[row][col] == player {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        }
    }
    false
}