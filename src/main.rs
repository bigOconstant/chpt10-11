/*
Caleb McCarthy
Peaks and Valleys Chapter 10.11 page 151p,



*/


fn main() {
    println!("Chapter 10 10.11");
    //    let listNum: Vec<i32> = Vec::new();
    //pub fn remove(&mut self, index: usize) -> T
//    let mut listNum = vec![5,3,1,2,3];
    let mut listNum = vec![5,8,6,2,3,4,6];
    //pub fn push(&mut self, value: T)
    let mut copyvec = listNum.to_vec();

    listNum.sort_by(|a, b| a.cmp(b));
    copyvec.sort_by(|a,b| b.cmp(a));

    for x in 0..listNum.len(){
        println!("listNum[{}]={}",x,listNum[x]);
    }
    for x in 0..copyvec.len(){
        println!("listNum[{}]={}",x,copyvec[x]);
    }
         let sizeofthing = listNum.len();
    let mut finalvector = Vec::with_capacity(sizeofthing);

    let mut tester = true;
    for x in 0..listNum.len(){
        if tester {
            let input = listNum.pop();
            finalvector.push(input);
            let sizeofthing = copyvec.len();
            copyvec.remove(0);
            tester = !tester;
            
        }else{
            let input = copyvec.pop();
            finalvector.push(input);
            let sizeofthing = listNum.len();
            listNum.remove(0);
            tester = !tester;
        }
    }
    for x in 0..finalvector.len(){
        println!("output:{} = {:?}",x,finalvector[x]);
    }
}
