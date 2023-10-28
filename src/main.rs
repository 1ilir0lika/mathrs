use std::io;

const SIMBOLI_DIS_DOPPI: [&str; 2] = [">=", "<="];
const SIMBOLI_DIS_SING: [&str; 2] = [">", "<"];
//const SIMBOLI_DIS=[SIMBOLI_DIS_DOPPI,SIMBOLI_DIS_SING];
const SIMBOLI_DIS: [&str; 4] = [">=", "<=", ">", "<"];
const SIMBOLI_EQ: [&str; 1] = ["="];
//const SIMBOLI_VALIDi=["/","+","-"];
const NUMERI:[char; 10] =['0','1','2','3','4','5','6','7','8','9'];
const LETTERE:[char; 3]=['a','b','c'];
fn only_one(arr: Vec<&str>, s: String) -> bool {
    let mut res = 0;
    for i in 0..arr.len() {
        match howmany(s.chars().collect(), arr[i].chars().collect()) {
            1 => {
                res += 1;
            }
            2.. => return false,
            _ => {}
        }
    }
    //println!("{}",res);
    res == 1
   
}
fn parameters(s: &str)->[&str; 2]{
    let pos=where_simbol(s.chars().collect());
    //println!("{}",pos);
    let (primo, secondo) = s.split_at(pos);
    println!("primo: {} secondo: {}",primo,secondo);
    return [primo,secondo];
}
fn howmany(s: Vec<char>, simb: Vec<char>) -> u8 {
    let mut res = 0;
    //print!("{} {:?} {:?}",simb.len(),s,simb);
    //-simb-len() per on uscire in caso len>1 ed il simbolo non può essere alla fine
    for i in 0..s.len() - simb.len() {
        //println!("{:?} {:?} {}",&s[i..i+simb.len()],simb,&s[i..i+simb.len()]==simb);
        if &s[i..i + simb.len()] == simb {
            res += 1;
        }
    }
    //println!("{}",res);
    res
}
//vedere se non contiene almeno un elemento in un array di reference a stringhe come possono essere la lista dei simboli delle disequazioni
fn not_contain(arr: Vec<&str>, s: String) -> bool {
    for i in 0..arr.len() {
        if howmany(s.chars().collect(), arr[i].chars().collect()) != 0 {
            return false;
        }
    }
    true
}
//trova dov'è il simbolo che sta nell'array dei simboli
fn where_simbol(s: Vec<char>)->usize{
for i in 0..s.len(){
if s[i]=='='||s[i]=='>'||s[i]=='<'{
return i;
}
}
0
}
fn syntax(s:String){
    if s.len()<=2{
        panic!("input non valido");
    }
    //verificare che abbia 2 termini ovvero qualcosa prima e dopo del segno
    if SIMBOLI_DIS
        .into_iter()
        .any(|x| x.chars().last() == s.chars().last())
        || SIMBOLI_EQ
            .into_iter()
            .any(|x| x.chars().last() == s.chars().last())
    {
        panic!("errore,non c'è l'ultimo termine");
    }
    if SIMBOLI_DIS
        .into_iter()
        .any(|x| x.chars().next() == s.chars().next())
        || SIMBOLI_EQ
            .into_iter()
            .any(|x| x.chars().next() == s.chars().next())
    {
        panic!("errore,non c'è il primo termine");
    }

    //verificare che non si usino simboli che non siano ^,/,.,*,...
    match s.as_str() {
        _ if only_one(SIMBOLI_DIS_DOPPI.to_vec(), s.clone())
            && only_one(SIMBOLI_EQ.to_vec(), s.clone())
            && only_one(SIMBOLI_DIS_SING.to_vec(), s.clone()) =>
        {
            println!("dis simb doppio");
        }
        _ if only_one(SIMBOLI_DIS_SING.to_vec(), s.clone())
            && s.contains("=") == false =>
        {
            //parameters(buffer.SIMBOLI_DIS);

            println!("dis");
        }
        _ if only_one(SIMBOLI_EQ.to_vec(), s.clone())
            && not_contain(SIMBOLI_DIS.to_vec(), s.clone()) =>
        {
            println!("eq");
        }
        //o non ci sono simboli o ce ne sono troppi
        _ => {
            //println!("dis doppi: {}\neq: {}\ndis SING: {} ",only_one(SIMBOLI_DIS_DOPPI.to_vec(),buffer.clone()),only_one(SIMBOLI_EQ.to_vec(),buffer.clone()),only_one(SIMBOLI_dis.to_vec(),buffer.clone()));
            panic!("errore,riformula la richiesta correttamente");
        }

    };
}


//sposta tutti i numeri a destra cambiandoli di segno
//ovviamente l incognita non si spostera a destra
fn normalize(mut s: Vec<char>)->Vec<char>{
    for i in 0..s.len() {
        if NUMERI.contains(&s[i]){
            if i< where_simbol(s.clone()){
                move_to_right(i,s.clone());
            }else{
                return s.clone();
            }
        }
    }
s.clone()
}
//sposta a destra cambiando di segno
    //guarda s destra,se ce un altro numero aggiungilo al buffer,fa cosi fino a quando non becchi
    //un segno,in tal caso sara necessario partire dall index iniziale e guardare indietro facendo
    //la stessa cosa di prima ma in questo caso trovando un segno si esce dal loop e si mette alla
    //fine del vettore di input un buffer con uno spazio conosciuto con l ultimo segno cambiato
    //in caso in tutto a sinistra non ci dovesse essere un segno dai per scontato che sia piu e
    //fai la stessa procedura
fn move_to_right(index: usize,mut s:Vec<char>){
    let index_simbolo_eq=where_simbol(s.clone());
}
//risolvi la parte di destra e ritorna "param1 simbolo soluzione" per le equazioni,per le altre cambiare segno nei vari casi
fn solve(){

}


fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.pop();
    syntax(buffer.clone());
    
}
