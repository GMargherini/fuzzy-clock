use chrono::Timelike;

fn main() {
    let now = chrono::offset::Local::now();
    let m = now.minute();
    let h = now.hour();
    println!("{}",get_time(h, m));
}

fn get_time(h:u32,m:u32) -> String{
    let hours_en = ["midnight","one","two","three","four","five","six","seven","eight","nine", "ten", "eleven", "noon"];
    let mins_en = ["","ten past","quarter past","twenty past","half past","twenty to","quarter to","ten to","almost"];
	let mut h_ind = ((h%12)) as usize;
	let m_ind = ((m) as f32 /6.67) as usize;
	if m_ind > 4{
		h_ind = (h_ind + 1)%12
	}
    let mut ht = hours_en[h_ind];
    if h_ind == 0 && (h == 11 || h == 12){
        ht = hours_en[12];
    }
    let mt = mins_en[m_ind];
    if m_ind == 0{
        let res = format!("{mt}{ht}").to_owned();
    	return res
    }
    let res = format!("{mt} {ht}").to_owned();
    return res
}
