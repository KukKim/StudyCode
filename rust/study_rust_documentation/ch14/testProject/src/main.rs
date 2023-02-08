// use testProject::kinds::PrimaryColor;
// use testProject::utils::mix;
use vortiTestProject::PrimaryColor;
use vortiTestProject::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
