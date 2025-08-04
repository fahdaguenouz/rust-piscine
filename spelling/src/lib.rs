pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }

    let mut res = String::new();

    let alf = n / 1000;
    let below_alf = n % 1000;
        if alf > 0 {
        res.push_str(&format!("{} thousand", get_word(alf)));
        if below_alf > 0 {
            res.push(' ');
        }
    }

    if below_alf > 0 {
        res.push_str(&get_word(below_alf));
    }

    res
}

fn get_word(n: u64) -> String {
    let ones = [
        "", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    let mut parts = Vec::new();

    let hundred = n / 100;
    let rest = n % 100;

    if hundred > 0 {
        parts.push(format!("{} hundred", ones[hundred as usize]));
    }

    if rest >= 10 && rest < 20 {
        parts.push(teens[(rest - 10) as usize].to_string());
    } else {
        let ten = rest / 10;
        let one = rest % 10;

       if ten > 0 {
                if one > 0 {
              parts.push(format!("{}-{}", tens[ten as usize], ones[one as usize]));
             } else {
        parts.push(tens[ten as usize].to_string());
            }
        } else if one > 0 {
          parts.push(ones[one as usize].to_string());
        }
    }

    parts.join(" ")
}