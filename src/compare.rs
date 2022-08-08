use crate::{card::Card, player::Hand};

#[derive(Clone)]
pub struct ToCompare {
    checkers: Vec<&'static dyn Fn(&Vec<Card>) -> (bool, i8)>,
}

impl ToCompare {
    pub fn new() -> Self {
        let mut funcs: Vec<&dyn Fn(&Vec<Card>) -> (bool, i8)> = Vec::new();
        funcs.push(&find_one_pair);
        funcs.push(&find_two_pair);
        funcs.push(&find_3ok);
        funcs.push(&find_straight);
        funcs.push(&find_flush);
        funcs.push(&find_full_house);
        funcs.push(&find_4ok);
        funcs.push(&find_straight_flush);
        funcs.push(&find_royal_flush);

        Self { checkers: funcs }
    }

    pub fn compare(&self, to_compare: &Vec<Card>) -> i8 {
        for f in self.checkers.iter().rev() {
            let r = f(to_compare);
            if r.0 == true {
                return r.1;
            }
        }
        0
    }
}

fn find_one_pair(comp: &Vec<Card>) -> (bool, i8) {
    let mut temp = comp.clone();
    temp.sort_by(|a, b| a.get_face().cmp(&b.get_face()));
    let mut ret = false;

    for idx in 1..temp.len() {
        if temp[idx-1].get_face() == temp[idx].get_face() {
            ret = true;
            break
        }
    }

    (ret, 1)
}

fn find_two_pair(comp: &Vec<Card>) -> (bool, i8) {
    let mut temp = comp.clone();
    temp.sort_by(|a, b| a.get_face().cmp(&b.get_face()));
    let mut ret = false;
    let mut t_char = '0';

    for idx in 1..temp.len() {
        if temp[idx-1].get_face() == temp[idx].get_face() {
            t_char = temp[idx].get_face();
            continue;
        }
        if temp[idx-1].get_face() == temp[idx].get_face() && temp[idx].get_face() != t_char {
            ret = true;
            break
        }
    }

    (ret, 2)
}

fn find_3ok(comp: &Vec<Card>) -> (bool, i8) {
    let mut temp = comp.clone();
    temp.sort_by(|a, b| a.get_face().cmp(&b.get_face()));
    let mut ret = false;

    for idx in 2..temp.len() {
        if temp[idx-2].get_face() == temp[idx].get_face() {
            ret = true;
            break
        }
    }

    (ret, 3)
}

fn find_straight(comp: &Vec<Card>) -> (bool, i8) {
    let mut temp = comp.clone();
    temp.sort_by(|a, b| a.get_face_value().cmp(&b.get_face_value()));
    let mut cnt = 0;
    let mut ace = false;
    let mut suit: char = 'S';

    for idx in 0..temp.len() {
        if cnt >= 5 {
            break;
        }
        if temp[idx].get_face_value() == 1 {
            ace = true;
            continue;
        }
        if temp[idx].get_suit() != suit {
            suit = temp[idx].get_suit();
            ace = false;
            cnt = 0;
            continue;
        }
        else if temp[idx].get_face_value() == 13 && ace {
            cnt += 2;
            continue;
        }
        else if temp[idx].get_face_value() == temp[idx].get_face_value() - 1 {
            cnt += 1;
            continue;
        }
    }

    let ret = cnt >= 5;

    (ret, 4)
}

fn find_flush(comp: &Vec<Card>) -> (bool, i8) {
    let temp = comp.clone();
    let mut cnts = 0;
    let mut cnth = 0;
    let mut cntd = 0;
    let mut cntc = 0;

    temp.iter().for_each(|f| {
        let _ = match f.get_suit() {
            'S' => cnts += 1,
            'H' => cnth += 1,
            'D' => cntd += 1,
            'C' => cntc += 1,
            _ => {},
        };
    });

    let ret = cnts >= 5 || cnth >= 5 || cntd >= 5 || cntc >= 5;

    (ret, 5)
}

fn find_full_house(comp: &Vec<Card>) -> (bool, i8) {
    //! TODO, HARD TO IMPLEMENT

    (false, 6)
}

fn find_4ok(comp: &Vec<Card>) -> (bool, i8) {
    let mut temp = comp.clone();
    temp.sort_by(|a, b| a.get_face().cmp(&b.get_face()));
    let mut ret = false;

    for idx in 3..temp.len() {
        if temp[idx-3].get_face() == temp[idx].get_face() {
            ret = true;
            break
        }
    }

    (ret, 7)
}

fn find_straight_flush(comp: &Vec<Card>) -> (bool, i8) {
    (find_straight(comp).0 && find_flush(comp).0, 8)
}

fn find_royal_flush(comp: &Vec<Card>) -> (bool, i8) {
    //? Need to implement, but it's not unnecessary
    (false, 9)
}
