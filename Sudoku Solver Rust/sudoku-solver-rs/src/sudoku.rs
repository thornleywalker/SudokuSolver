use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub enum Error {
    // ((row, col), (old, new))
    CellAlreadySet((usize, usize), (u8, u8)),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CellAlreadySet((row, col), (old, new)) => {
                write!(
                    f,
                    "Cell {}, {} already set. Old: {} New: {}",
                    row, col, old, new
                )
            }
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Difficulty {
    Solved,
    Easy,
    Medium,
    Hard,
    Expert,
    Evil,
}

type CellAM = Arc<Mutex<Cell>>;

pub struct Board {
    _internal: [[CellAM; 9]; 9],
}
impl Board {
    pub fn new(json: &str) -> Self {
        let mut board = Board::new_empty();
        let mut col = 0;
        let mut row = 0;
        for letter in json.chars() {
            match letter {
                ']' => {
                    row += 1;
                    col = 0;
                }
                '0' => col += 1,
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    board.set(row, col, letter.to_digit(10).unwrap() as u8);
                    col += 1;
                }
                _ => {}
            }
        }
        board
    }
    pub fn new_empty() -> Self {
        Board {
            _internal: [
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
                [
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                    Arc::new(Mutex::new(Cell::new_empty())),
                ],
            ],
        }
    }
    // solves the puzzle and returns the difficulty
    pub fn solve(&mut self) -> Difficulty {
        eprintln!("Initial Board: \n{}", self);
        eprintln!("Beggining solve");
        // workers to board communications
        let (worker_tx, board_rx) = mpsc::channel::<Message>();

        // for each row/col/square, generate a worker
        let mut row_workers = vec![];
        let mut col_workers = vec![];
        let mut sqr_workers = vec![];

        eprintln!("Generating workers");
        for i in 0..9 {
            // cells to worker communications
            let (row_cell_tx, row_worker_rx) = mpsc::channel::<()>();
            let (col_cell_tx, col_worker_rx) = mpsc::channel::<()>();
            let (sqr_cell_tx, sqr_worker_rx) = mpsc::channel::<()>();

            row_workers.push(Worker::new(
                row_worker_rx,
                worker_tx.clone(),
                self.get_row(i),
            ));
            col_workers.push(Worker::new(
                col_worker_rx,
                worker_tx.clone(),
                self.get_col(i),
            ));
            sqr_workers.push(Worker::new(
                sqr_worker_rx,
                worker_tx.clone(),
                self.get_square(i / 3, i % 3),
            ));

            // add the tx to each corresponding cell
            for j in 0..9 {
                // rows
                self._internal[i][j]
                    .lock()
                    .unwrap()
                    .add_row_tx(row_cell_tx.clone());
                // cols
                self._internal[i][j]
                    .lock()
                    .unwrap()
                    .add_col_tx(col_cell_tx.clone());
                // squares
                self._internal[i][j]
                    .lock()
                    .unwrap()
                    .add_sqr_tx(sqr_cell_tx.clone());
            }
        }
        eprintln!("Starting workers");
        // put workers to work
        let mut threads = vec![];
        for row_worker in row_workers {
            threads.push(thread::spawn(move || row_worker.run()));
        }
        for col_worker in col_workers {
            threads.push(thread::spawn(move || col_worker.run()));
        }
        for sqr_worker in sqr_workers {
            threads.push(thread::spawn(move || sqr_worker.run()));
        }
        eprintln!("Starting board loop");
        // start updating the board
        let mut done_count = 0;
        loop {
            if let Ok(message) = board_rx.recv() {
                //eprintln!("Got: {:?}", message);
                match message.update {
                    Update::Set(val) => {
                        self.set(message.row, message.col, val);
                        done_count = 0;
                    }
                    Update::Remove(val) => {
                        self.remove(message.row, message.col, val);
                        done_count = 0;
                    }
                    Update::Done => {
                        done_count += 1;
                        eprintln!("Done: {}", done_count);
                    }
                }
                if done_count == 9 {
                    eprintln!("Breaking");
                    break;
                }
            } else {
                break;
            }
        }
        eprintln!("Board\n{}", &self);
        for thread in threads {
            thread.join().unwrap();
        }
        Difficulty::Solved
    }
    fn get_row(&self, index: usize) -> Row {
        let mut it = vec![];
        for i in 0..9 {
            it.push(Arc::clone(&self._internal[index][i]));
        }
        Row {
            _internal: it,
            index: index,
        }
    }
    fn get_col(&self, index: usize) -> Column {
        let mut it = vec![];
        for i in 0..9 {
            it.push(Arc::clone(&self._internal[index][i]));
        }
        Column {
            _internal: it,
            index: index,
        }
    }
    fn get_square(&self, row: usize, col: usize) -> Square {
        let mut it = vec![];
        let row_offset = 3 * row;
        let col_offset = 3 * col;
        for i in 0..3 {
            for j in 0..3 {
                it.push(Arc::clone(&self._internal[row_offset + i][col_offset + j]));
            }
        }
        Square {
            _internal: it,
            index: row * 3 + col,
        }
    }
    fn get_square_by_cell(&self, cell_row: usize, cell_col: usize) -> Square {
        self.get_square(cell_row / 3, cell_col / 3)
    }
    fn set(&mut self, row: usize, col: usize, val: u8) {
        // set the actual value
        if let Err(e) = self._internal[row][col].lock().unwrap().set(val) {
            match e {
                Error::CellAlreadySet(_, (old, new)) => {
                    Err(Error::CellAlreadySet((row, col), (old, new))).unwrap()
                }
            }
        }

        // remove that possibility from other cells
        let row_offset = (row / 3) * 3;
        let col_offset = (col / 3) * 3;
        for i in 0..9 {
            self.remove(row, i, val);
            self.remove(i, col, val);
            self.remove(row_offset + (i / 3), col_offset + (i % 3), val);
        }
    }
    fn remove(&mut self, row: usize, col: usize, val: u8) {
        self._internal[row][col].lock().unwrap().remove(val);
    }
    // fn get(&self, row: usize, col: usize) -> &Cell {
    //     &self._internal[row][col]
    // }
    // fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell {
    //     &mut self._internal[row][col]
    // }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret_string =
            " ------- ------- ------- ------- ------- ------- ------- ------- ------- \n"
                .to_string();
        for cell_row in &self._internal {
            let mut line1 = "|".to_string();
            let mut line2 = "|".to_string();
            let mut line3 = "|".to_string();
            let mut line4 = " ".to_string();

            for cellam in cell_row {
                match cellam.lock().unwrap().val {
                    CellValue::Value(val) => {
                        line1.push_str(" * * * |");
                        line2.push_str(format!(" * {} * |", val).as_str());
                        line3.push_str(" * * * |");
                        line4.push_str(" ------ ");
                    }
                    CellValue::Poss(poss) => {
                        let vals = poss.values();
                        for i in 1..=3 {
                            if vals.contains(&i) {
                                line1.push_str(format!(" {}", i).as_str());
                            } else {
                                line1.push_str("  ");
                            }
                        }
                        for i in 4..=6 {
                            if vals.contains(&i) {
                                line2.push_str(format!(" {}", i).as_str());
                            } else {
                                line2.push_str("  ");
                            }
                        }
                        for i in 7..=9 {
                            if vals.contains(&i) {
                                line3.push_str(format!(" {}", i).as_str());
                            } else {
                                line3.push_str("  ");
                            }
                        }
                        line1.push_str(" |");
                        line2.push_str(" |");
                        line3.push_str(" |");
                        line4.push_str("------- ");
                    }
                }
            }
            ret_string.push_str(format!("{}\n{}\n{}\n{}\n", line1, line2, line3, line4).as_str());
        }
        write!(f, "{}", ret_string)
    }
}

struct Worker<C>
where
    C: Container,
{
    // listens for updates from cells
    update_listener: mpsc::Receiver<()>,
    // sends update messages to board
    message_sender: mpsc::Sender<Message>,
    // the container the worker is responsible for
    domain: C,
}
impl<C> Worker<C>
where
    C: Container,
{
    pub fn new(rx: mpsc::Receiver<()>, tx: mpsc::Sender<Message>, domain: C) -> Self {
        Self {
            update_listener: rx,
            message_sender: tx,
            domain: domain,
        }
    }
    fn send_updates(&self) {
        let messages = self.domain.reduce();
        for message in messages {
            match message.update {
                // if the domain is complete, the worker is finished
                Update::Done => {
                    //eprintln!("Worker finished");
                    self.message_sender.send(message).unwrap();
                    return;
                }
                // otherwise, request the updates
                _ => {
                    // eprintln!("Sending {:?}", message);
                    self.message_sender.send(message).unwrap();
                }
            }
        }
    }
    pub fn run(&self) {
        // eprintln!("Starting worker");
        self.send_updates();
        loop {
            match self.update_listener.recv() {
                Ok(_) => {
                    self.send_updates();
                }
                Err(_) => {
                    eprintln!("Closing worker loop on completion");
                    break;
                }
            }
        }
        self.send_updates();
    }
}

#[derive(Debug)]
enum Update {
    // set the cell to the value
    Set(u8),
    // remove the possibility from the cell
    Remove(u8),
    // the container is complete
    Done,
}

#[derive(Clone)]
enum Times {
    None,
    Once(usize),
    Many,
}

#[derive(Debug)]
struct Message {
    row: usize,
    col: usize,
    update: Update,
}
impl Message {
    pub fn new(row: usize, col: usize, update: Update) -> Self {
        Self {
            row: row,
            col: col,
            update: update,
        }
    }
}

trait Container {
    fn contains(&self, val: u8) -> bool;
    // returns a list of updates to be made
    fn reduce(&self) -> Vec<Message>;
    fn easy_reduce(&self) -> Vec<Message>;
    fn medium_reduce(&self) -> Vec<Message>;
    fn hard_reduce(&self) -> Vec<Message>;
    fn solve_check(&self) -> bool;
}
struct Row {
    _internal: Vec<CellAM>,
    // the row of the board
    index: usize,
}
impl Container for Row {
    fn solve_check(&self) -> bool {
        let mut solved = vec![];
        for cellam in &self._internal {
            match cellam.lock().unwrap().val {
                CellValue::Poss(_) => return false,
                CellValue::Value(val) => solved.push(val),
            }
        }
        solved.len() == 9
    }
    fn contains(&self, val: u8) -> bool {
        for cell_am in &self._internal {
            let cell = cell_am.lock().unwrap();
            match (*cell).val {
                CellValue::Value(curr) => {
                    if curr == val {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    fn reduce(&self) -> Vec<Message> {
        // let mut ret_vec = vec![];
        // ret_vec.append(&mut self.easy_reduce());
        // ret_vec.append(&mut self.medium_reduce());
        // ret_vec.append(&mut self.hard_reduce());
        // if ret_vec.len() == 0 && self.solve_check() {
        //     ret_vec.push(Message::new(0, 0, Update::Done));
        // }
        let mut ret_vec = self.easy_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.medium_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.hard_reduce();
        if ret_vec.len() == 0 && self.solve_check() {
            ret_vec.push(Message::new(0, 0, Update::Done));
        }

        ret_vec
    }
    fn easy_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        for (col, cellam) in self._internal.iter().enumerate() {
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    if let Some(val) = poss.solve() {
                        let mess = Message::new(self.index, col, Update::Set(val));
                        eprintln!("row-{}:ez:{:?}", self.index, mess);
                        ret_vec.push(mess);
                    }
                }
                _ => {}
            }
        }
        ret_vec
    }
    fn medium_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        // check if any cell is the only one with any values

        // vector of occurences for each values (represented by the index)
        let mut only_vec = vec![Times::None; 10];
        for (index, cellam) in (&self._internal).iter().enumerate() {
            // TODO: may be able to speed up by cloning the cell possibilities, then doing the maths
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    // go through each of the existing possibilities for the cell
                    for option in poss.values() {
                        match only_vec[option as usize] {
                            // the value hasn't been seen yet
                            Times::None => only_vec[option as usize] = Times::Once(index),
                            // the value has already been seen
                            Times::Once(_) => only_vec[option as usize] = Times::Many,
                            // the value has been seen several times
                            Times::Many => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // anything in only_vec that is a Once, create an update message
        for (val, thing) in only_vec.iter().enumerate() {
            match thing {
                Times::Once(index) => {
                    let mess = Message::new(self.index, *index, Update::Set(val as u8));
                    eprintln!("row-{}:md:{:?}", self.index, mess);
                    ret_vec.push(mess);
                }
                // we don't care if it didn't appeare, or appeared more than once
                _ => {}
            }
        }
        ret_vec
    }
    fn hard_reduce(&self) -> Vec<Message> {
        vec![]
    }
}

struct Column {
    _internal: Vec<CellAM>,
    // the col of the board
    index: usize,
}
impl Container for Column {
    fn solve_check(&self) -> bool {
        let mut solved = vec![];
        for cellam in &self._internal {
            match cellam.lock().unwrap().val {
                CellValue::Poss(_) => return false,
                CellValue::Value(val) => solved.push(val),
            }
        }
        solved.len() == 9
    }
    fn contains(&self, val: u8) -> bool {
        for cell_am in &self._internal {
            let cell = cell_am.lock().unwrap();
            match (*cell).val {
                CellValue::Value(curr) => {
                    if curr == val {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    fn reduce(&self) -> Vec<Message> {
        // let mut ret_vec = vec![];
        // ret_vec.append(&mut self.easy_reduce());
        // ret_vec.append(&mut self.medium_reduce());
        // ret_vec.append(&mut self.hard_reduce());
        // if ret_vec.len() == 0 && self.solve_check() {
        //     ret_vec.push(Message::new(0, 0, Update::Done));
        // }
        let mut ret_vec = self.easy_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.medium_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.hard_reduce();
        if ret_vec.len() == 0 && self.solve_check() {
            ret_vec.push(Message::new(0, 0, Update::Done));
        }
        ret_vec
    }
    fn easy_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        for (row, cellam) in self._internal.iter().enumerate() {
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    if let Some(val) = poss.solve() {
                        let mess = Message::new(self.index, row, Update::Set(val));
                        eprintln!("col-{}:ez:{:?}", self.index, mess);
                        ret_vec.push(mess);
                    }
                }
                _ => {}
            }
        }
        ret_vec
    }
    fn medium_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        // check if any cell is the only one with any values

        // vector of occurences for each values (represented by the index)
        let mut only_vec = vec![Times::None; 10];
        for (index, cellam) in (&self._internal).iter().enumerate() {
            // TODO: may be able to speed up by cloning the cell possibilities, then doing the maths
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    // go through each of the existing possibilities for the cell
                    for option in poss.values() {
                        match only_vec[option as usize] {
                            // the value hasn't been seen yet
                            Times::None => only_vec[option as usize] = Times::Once(index),
                            // the value has already been seen
                            Times::Once(_) => only_vec[option as usize] = Times::Many,
                            // the value has been seen several times
                            Times::Many => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // anything in only_vec that is a Once, create an update message
        for (val, thing) in only_vec.iter().enumerate() {
            match thing {
                Times::Once(index) => {
                    let mess = Message::new(self.index, *index, Update::Set(val as u8));
                    eprintln!("col-{}:md:{:?}", self.index, mess);
                    ret_vec.push(mess);
                }
                // we don't care if it didn't appear, or appeared more than once
                _ => {}
            }
        }
        ret_vec
    }
    fn hard_reduce(&self) -> Vec<Message> {
        vec![]
    }
}
struct Square {
    _internal: Vec<CellAM>,
    // the square of the board (top down, left to right)
    index: usize,
}
impl Container for Square {
    fn solve_check(&self) -> bool {
        let mut solved = vec![];
        for cellam in &self._internal {
            match cellam.lock().unwrap().val {
                CellValue::Poss(_) => return false,
                CellValue::Value(val) => solved.push(val),
            }
        }
        solved.len() == 9
    }
    fn contains(&self, val: u8) -> bool {
        for cell_am in &self._internal {
            let cell = cell_am.lock().unwrap();
            match (*cell).val {
                CellValue::Value(curr) => {
                    if curr == val {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    fn reduce(&self) -> Vec<Message> {
        // let mut ret_vec = vec![];
        // ret_vec.append(&mut self.easy_reduce());
        // ret_vec.append(&mut self.medium_reduce());
        // ret_vec.append(&mut self.hard_reduce());
        // if ret_vec.len() == 0 && self.solve_check() {
        //     ret_vec.push(Message::new(0, 0, Update::Done));
        // }
        let mut ret_vec = self.easy_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.medium_reduce();
        if ret_vec.len() > 0 {
            return ret_vec;
        }
        ret_vec = self.hard_reduce();
        if ret_vec.len() == 0 && self.solve_check() {
            ret_vec.push(Message::new(0, 0, Update::Done));
        }
        ret_vec
    }
    fn easy_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        for (index, cellam) in self._internal.iter().enumerate() {
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    if let Some(val) = poss.solve() {
                        let mess = Message::new(
                            (self.index / 3) * 3 + index / 3,
                            (self.index % 3) * 3 + index % 3,
                            Update::Set(val),
                        );
                        eprintln!("sqr-{}:ez:{:?}", self.index, mess);
                        ret_vec.push(mess);
                    }
                }
                _ => {}
            }
        }
        ret_vec
    }
    fn medium_reduce(&self) -> Vec<Message> {
        let mut ret_vec = vec![];
        // check if any cell is the only one with any values

        // vector of occurences for each values (represented by the index)
        let mut only_vec = vec![Times::None; 10];
        for (index, cellam) in (&self._internal).iter().enumerate() {
            // TODO: may be able to speed up by cloning the cell possibilities, then doing the maths
            match cellam.lock().unwrap().val {
                CellValue::Poss(poss) => {
                    // go through each of the existing possibilities for the cell
                    for option in poss.values() {
                        match only_vec[option as usize] {
                            // the value hasn't been seen yet
                            Times::None => only_vec[option as usize] = Times::Once(index),
                            // the value has already been seen
                            Times::Once(_) => only_vec[option as usize] = Times::Many,
                            // the value has been seen several times
                            Times::Many => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // anything in only_vec that is a Once, create an update message
        for (val, thing) in only_vec.iter().enumerate() {
            match thing {
                Times::Once(index) => {
                    let mess = Message::new(
                        (self.index / 3) * 3 + index / 3,
                        (self.index % 3) * 3 + index % 3,
                        Update::Set(val as u8),
                    );
                    eprintln!("sqr-{}:md:{:?}", self.index, mess);
                    ret_vec.push(mess);
                }
                // we don't care if it didn't appeare, or appeared more than once
                _ => {}
            }
        }
        ret_vec
    }
    fn hard_reduce(&self) -> Vec<Message> {
        vec![]
    }
}

#[derive(Clone, Default)]
struct Cell {
    val: CellValue,
    row_update: Option<mpsc::Sender<()>>,
    col_update: Option<mpsc::Sender<()>>,
    sqr_update: Option<mpsc::Sender<()>>,
}
impl Cell {
    pub fn new(
        val: u8,
        row_update: mpsc::Sender<()>,
        col_update: mpsc::Sender<()>,
        sqr_update: mpsc::Sender<()>,
    ) -> Self {
        Cell {
            val: CellValue::new(val),
            row_update: Some(row_update),
            col_update: Some(col_update),
            sqr_update: Some(sqr_update),
        }
    }
    pub fn new_empty() -> Self {
        Cell {
            val: CellValue::new_empty(),
            row_update: None,
            col_update: None,
            sqr_update: None,
        }
    }
    // when the cell changes, send a message to the updates list
    pub fn set(&mut self, val: u8) -> Result<(), Error> {
        match &mut self.val {
            CellValue::Poss(_poss) => {
                self.val = CellValue::new(val);
                self.send_updates();
            }
            CellValue::Value(curr) => {
                if *curr != val {
                    return Err(Error::CellAlreadySet(
                        (usize::MAX, usize::MAX),
                        (*curr, val),
                    ));
                }
            }
        }

        self.row_update = None;
        self.col_update = None;
        self.sqr_update = None;
        Ok(())
    }
    pub fn remove(&mut self, val: u8) {
        if let CellValue::Poss(poss) = &mut self.val {
            if poss.contains(val) {
                poss.remove(val);
                self.send_updates();
            }
        }
    }
    // sends updates to the corresponding row, col, and square
    fn send_updates(&self) {
        if let Some(row_update) = &self.row_update {
            row_update.send(()).expect("Could not send message");
        }
        if let Some(col_update) = &self.row_update {
            col_update.send(()).expect("Could not send message");
        }
        if let Some(sqr_update) = &self.row_update {
            sqr_update.send(()).expect("Could not send message");
        }
    }
    pub fn add_row_tx(&mut self, tx: mpsc::Sender<()>) {
        if let CellValue::Poss(_) = self.val {
            self.row_update = Some(tx);
        }
    }
    pub fn add_col_tx(&mut self, tx: mpsc::Sender<()>) {
        if let CellValue::Poss(_) = self.val {
            self.col_update = Some(tx);
        }
    }
    pub fn add_sqr_tx(&mut self, tx: mpsc::Sender<()>) {
        if let CellValue::Poss(_) = self.val {
            self.sqr_update = Some(tx);
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Possibilities {
    _internal: u16,
}
impl Possibilities {
    pub fn new() -> Self {
        Self { _internal: 0x1FF }
    }
    pub fn remove(&mut self, val: u8) {
        self._internal = self._internal & !(1 << (val - 1));
    }
    pub fn values(&self) -> Vec<u8> {
        let mut ret_vals = vec![];
        for i in 0..9 {
            let mask = 1 << i;
            if (self._internal & mask) == mask {
                ret_vals.push(i + 1)
            }
        }
        //eprintln!("From {:x} got {:?}", self._internal, ret_vals);
        ret_vals
    }
    pub fn contains(&self, val: u8) -> bool {
        let mask = 1 << (val - 1);
        (self._internal & mask) == mask
    }
    pub fn solve(&self) -> Option<u8> {
        match self._internal {
            0 => None, //panic!("shouldn't be empty"),
            0x0001 => Some(1),
            0x0002 => Some(2),
            0x0004 => Some(3),
            0x0008 => Some(4),
            0x0010 => Some(5),
            0x0020 => Some(6),
            0x0040 => Some(7),
            0x0080 => Some(8),
            0x0100 => Some(9),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum CellValue {
    Value(u8),
    Poss(Possibilities),
}
impl CellValue {
    fn new(val: u8) -> Self {
        Self::Value(val)
    }
    fn new_empty() -> Self {
        Self::Poss(Possibilities::new())
    }
    // sets the concrete value
    fn set(&mut self, val: u8) {
        *self = Self::Value(val);
    }
    // removes the possibility, converts to a val if only one solution
    fn remove(&mut self, val: u8) {
        if let Self::Poss(poss) = self {
            poss.remove(val);
        }
    }
}
impl Default for CellValue {
    fn default() -> Self {
        Self::Poss(Possibilities::new())
    }
}
