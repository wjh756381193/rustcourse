fn main() {

    let studentA = ("nameA", 12, 100, 98, 99, 99.0);
    println!("{}", studentA.2);

    let (name, no, chi, eng, mat, avg) = studentA;
    println!("{}", name);

}
