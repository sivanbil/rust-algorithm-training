pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String{
        let mut number = column_number;
        let mut column_title = String::new();

        while number > 0 {
            number -= 1;
            let operate_number = number % 26;
            // 利用ASCII码的连续性来获取字母A-Z
            column_title.insert(0, ((operate_number as u8 + b'A') as char));
            number /= 26;
        }

        column_title
    }
    pub fn convert_to_title_low(column_number: i32) -> String {
        let mut number = column_number;
        let mut column_title = String::from("");

        while number > 0 {
            number -= 1;
            let operate_number = number % 26;
            column_title = (match operate_number {
                0 => "A",
                1 => "B",
                2 => "C",
                3 => "D",
                4 => "E",
                5 => "F",
                6 => "G",
                7 => "H",
                8 => "I",
                9 => "J",
                10 => "K",
                11 => "L",
                12 => "M",
                13 => "N",
                14 => "O",
                15 => "P",
                16 => "Q",
                17 => "R",
                18 => "S",
                19 => "T",
                20 => "U",
                21 => "V",
                22 => "W",
                23 => "X",
                24 => "Y",
                25 => "Z",
                _ => "",
            }).to_owned() + &*column_title;
            number /= 26;
        }

        column_title
    }


}