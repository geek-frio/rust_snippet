struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// color data被传入了print_color的scope,外部scope不再拥有这个data
fn print_color(c: Color) {}

// color data的引用传递进入了print_color的scope,外部scope依然拥有Color
fn print_color_by_reference(c: &Color) {}

pub fn run() {
    let color = Color { r: 1, g: 2, b: 3 };
     // 可以调用多次
     print_color_by_reference(&color);
    // 这种调用方法只能调用一次
    print_color(color);
   
}
