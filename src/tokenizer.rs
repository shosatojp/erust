use std::borrow::Cow;
use std::io::Write;

fn out<'a, S: Into<Cow<'a, str>>>(fp: &mut std::fs::File, s: S) {
    fp.write(s.into().as_bytes()).expect("error");
}

fn main() {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("out.html")
        .expect("error");
    macro_rules! out {
        ($x:expr) => {
            out(&mut f, $x);
        };
    }
out!("");
let title = "記念祭";
let class = "school-name";
let list = vec![1,2,3];
out!("<html><head><title>");
out!(format!("{}",title));
out!("</title></head><body>");
/* TODO: ほげほげ */
out!("<p class=\"");
out!(format!("{}",class));
out!("\"></p>");
for i in list {
out!("<p>");
out!(format!("{}",i));
out!("</p>");
}
out!("</body></html>");
out!("");
}
