use advanced_4::is_self_closing_html;

fn main() {
    if is_self_closing_html!(<a) {
        println!("is html :D");
    } else {
        println!("is not html :(");
    }
}
