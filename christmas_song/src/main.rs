fn main() {
    println!("\n The Twelve Days of Christmas\n");
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh","twelfth"];
    let gifts: [&str; 12] = [
        "A patridge in a pear tree.",
        "Two Turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    for i in 0..12 {
       println!("On the {} day of Christmas,\nmy true love gave to me", days[i]);
       for j in (0..i+1).rev() {
            println!("{}", gifts[j]);    
       }
       println!();
    }
}
