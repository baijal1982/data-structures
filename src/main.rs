mod peek_finding;

fn main() {
    let data= vec![3,5,6,8,9,2];
    let length = data.len();
    peek_finding::peek::find_peek(data,0,length-1,length);
}
