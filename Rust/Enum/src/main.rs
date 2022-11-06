#[derive(Debug)]
enum PockSuit{
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }
#[derive(Debug)]
struct PokerCard{
    suit:PockSuit,
    data:u8,
}
#[derive(Debug)]
enum PockSuitPlus{
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}
fn main() {
    println!("Hello, world!");
    
    let heart=PockSuit::Hearts;
    let diamond=PockSuit::Diamonds;
    dbg!(&heart);
    print_suit(heart);
    print_suit(diamond);

    let c1=PokerCard{
        suit:PockSuit::Clubs,
        data:1,
    };
    dbg!(&c1);
    let c2=PockSuitPlus::Clubs(1);
    println!("{:?}",c2);

}
fn print_suit(card:PockSuit){
    println!("{:?}",card);
}
