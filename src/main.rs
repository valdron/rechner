#![allow(dead_code)]
use std::env;
use self::Operator::*;
use self::BerechnungOrZahl::*;
#[derive(Debug)]
enum Operator{
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn get_char(&self) -> char {
        match *self{
            Add => '+',
            Sub => '-',
            Mul => '*',
            Div => '/'
        }
    }

    fn berechne(&self,x: f32, y: f32) -> f32 {
        match *self {
            Add => x + y,
            Sub => x - y,
            Mul => x * y,
            Div => x / y
        }
    }
    fn get_next_operator(&self) -> Option<Operator> {
        match *self {
            Add => Some(Sub),
            Sub => Some(Mul),
            Mul => Some(Div),
            Div => None
        }
    }

}

#[derive(Debug)]
struct Berechner {
    zeichen: String,
    op: Operator,
}

impl Berechner {
    fn new(s: String, op: Operator) -> Berechner {
        Berechner{
            zeichen: s,
            op: op
        }
    }
    fn split_into_berechner(&self,c: char) -> Vec<BerechnungOrZahl> {
        self.zeichen.as_str().split(c).map(|s: &str|{BerechnungOrZahl::from_str(s.to_string())}).collect()
    }

    fn berechne(&self) -> f32 {
        println!("String: '{}' Operator:{:?}",self.zeichen, self.op);
        let mut v = self.split_into_berechner(self.op.get_char());
        for i in 0..v.len() {
            v[i] = match v[i]{
                Berechnung(ref s) => Zahl(Berechner::new(s.clone(),self.op.get_next_operator().unwrap()).berechne_zeichen()),
                Zahl(x) => Zahl(x)
            };
        }
        let mut ergebnis = v[0].unwrap_zahl();
        for arg in v[1..].iter() {
            match *arg {
                Zahl(x) => ergebnis = self.op.berechne(ergebnis, x),
                _ => panic!("non zahl in berechnung")
            }
        }
        ergebnis
    }
}
#[derive(Debug)]
enum BerechnungOrZahl{
    Berechnung(String),
    Zahl(f32)
}

impl BerechnungOrZahl{
    fn from_str(s: String) -> BerechnungOrZahl{
        match s.parse::<f32>(){
            Ok(x) => BerechnungOrZahl::Zahl(x),
            Err(_) => BerechnungOrZahl::Berechnung(s)
        }
    }
    fn unwrap_zahl(&self) -> f32 {
        match self {
            &Zahl(x) => x,
            &Berechnung(_) => panic!("unwrap called on non-zahl")
        }
    }
}

fn concat_args(args: Vec<String>) -> String {
    let mut str = "".to_string();
    for arg in &args[1..] {
        str.push_str(arg.as_str());
    }
    str
}

fn berechne_vec(v: &Vec<BerechnungOrZahl>, op: Operator) -> Result<f32, &'static str> {
    let mut ergebnis = match v[0] {
        Zahl(x) => x,
        Berechnung(_) => {return Err("isString");}
    };
    for st in v[1..].iter() {
        let zahl: f32 = match *st{
            Zahl(x) => x,
            Berechnung(_) => {return Err("isString");}
        };
        ergebnis = match op {
            Add => ergebnis + zahl,
            Sub => ergebnis - zahl,
            Mul => ergebnis * zahl,
            Div => ergebnis / zahl,
        }
    }
    Ok(ergebnis)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let rechnung = concat_args(args);
    let rechner = Berechner::new(rechnung,Add);
    let ergebnis = rechner.berechne();

    println!("{:?}", ergebnis);
}
