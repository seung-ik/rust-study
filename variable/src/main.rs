fn main() {
    let width: u32 = 50;
    let height: u32 = 40;
    println!("the area of the rectangle is {} square pixels",area(width,height));

    let rect1 = (20,30);
    println!("the area of the rectangle is {} square pixels",area2(rect1));

    let rect2 = Rectangle { width: 20, height:90 };
    println!("the area of the rectangle is {} square pixels",area3(&rect2));

    
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32,height: u32) -> u32 {
    width*height
}

fn area2(dimension:(u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

