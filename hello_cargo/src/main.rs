fn main() {
    let dice_roll = 7;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {
        println!("1")
    }
    fn remove_fancy_hat() {
        println!("2");
    }
    fn move_player(num_spaces: u8) {
        println!("{}",num_spaces)
    }

}
