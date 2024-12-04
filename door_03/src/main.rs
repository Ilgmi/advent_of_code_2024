use common::read_lines;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::ops::{Add, AddAssign, Sub};

fn main() {
    let lines = read_lines("door_03/input.txt").unwrap();

    let result = parser(lines);
    println!("{:?}", result);
}

fn count_do_dont(lines: Lines<BufReader<File>>) -> (i32, i32) {
    let mut a = 0;
    let mut b = 0;
    let t = lines.flatten().map(String::from).for_each(|line| {
        let mut chars = line.chars().collect::<Vec<char>>();
        let mut index = 0;
        let mut do_mul = true;
        while index < chars.len() {
            let d = parse_do_or_dont(&chars, &mut index);
            match d {
                None => {}
                Some(true) => a += 1,
                Some(false) => b += 1,
            };
            index += 1;
        }
    });

    return (a, b);
}

fn parser(lines: Lines<BufReader<File>>) -> i32 {
    let mut do_mul = true;
    lines
        .flatten()
        .map(String::from)
        .map(|s| parse_line(&s, &mut do_mul))
        .sum()
}

fn parse_line(line: &str, do_mul: &mut bool) -> i32 {
    let mut resutl = 0;

    let mut chars = line.chars().collect::<Vec<char>>();
    let mut index = 0;
    while index < chars.len() {
        let mul = parse_mul(&mut chars, &mut index, do_mul);
        if let Some((x, y)) = mul {
            resutl.add_assign(x * y);
        }
    }

    resutl
}

fn parse_do_or_dont(chars: &Vec<char>, current_index: &mut usize) -> Option<bool> {
    let mut is_do_index = *current_index;
    let is_do = is_do(chars, &mut is_do_index);

    if is_do {
        *current_index += is_do_index.sub(*current_index);
        return Some(true);
    }

    let mut is_dont_index = *current_index;
    let is_dont = is_dont(chars, &mut is_dont_index);
    if is_dont {
        *current_index += is_dont_index.sub(*current_index);
        return Some(false);
    }

    None
}

fn is_do(chars: &Vec<char>, current_index: &mut usize) -> bool {
    for e_c in "do()".chars() {
        if e_c != chars[*current_index] {
            return false;
        }
        *current_index += 1
    }
    *current_index -= 1;
    true
}

fn is_dont(chars: &Vec<char>, current_index: &mut usize) -> bool {
    for e_c in "don't()".chars() {
        if e_c != chars[*current_index] {
            return false;
        }
        *current_index += 1
    }
    *current_index -= 1;
    true
}

fn parse_mul(
    chars: &Vec<char>,
    current_index: &mut usize,
    do_mul: &mut bool,
) -> Option<(i32, i32)> {
    let c = chars[*current_index];
    match c {
        'm' => {
            let is_mul = is_mul(chars, current_index);
            if is_mul {
                let var_one = parse_num(chars, current_index, ',');

                if var_one.is_some() && chars[*current_index] == ',' {
                    *current_index += 1;
                    let var_two = parse_num(chars, current_index, ')');
                    if var_two.is_some() && chars[*current_index] == ')' {
                        *current_index += 1;
                        return if *do_mul {
                            Some((var_one.unwrap(), var_two.unwrap()))
                        } else {
                            Some((0, 0))
                        };
                    }
                }
            }
        }
        'd' => match parse_do_or_dont(chars, current_index) {
            None => {}
            Some(t) => {
                *do_mul = t;
            }
        },
        _ => {}
    }
    *current_index += 1;
    None
}

fn is_mul(chars: &Vec<char>, current_index: &mut usize) -> bool {
    for e_c in "mul(".chars() {
        if e_c != chars[*current_index] {
            return false;
        }
        *current_index += 1
    }
    true
}

fn parse_num(chars: &Vec<char>, curr_index: &mut usize, end: char) -> Option<i32> {
    let mut unparsed_num = String::new();

    let mut i = 0;
    while *curr_index < chars.len() && i != 3 {
        let ch = chars[*curr_index];
        match ch {
            c if c.is_ascii_digit() => unparsed_num.push(c),
            c if c == end => {
                break;
            }
            _ => return None,
        }
        *curr_index += 1;
        i.add_assign(1);
    }

    unparsed_num.parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_do_or_dont() {
        let mut tests = vec![
            ("do()", Some(true), 3),
            ("don't()", Some(false), 6),
            ("dos)", None, 0),
        ];

        for test in tests {
            let mut line = test.0.chars().collect();
            let mut index = 0;
            let res = parse_do_or_dont(&mut line, &mut index);
            println!("chars: {:?}", line);

            assert_eq!(res, test.1);
            assert_eq!(index, test.2);
        }
    }

    #[test]
    fn test_parse_do() {
        let mut tests = vec![("do()", true, 3), ("dos)", false, 2)];

        for test in tests {
            let mut line = test.0.chars().collect();
            let mut index = 0;
            let res = is_do(&mut line, &mut index);
            // println!("index: {:?}, chars: {:?}",line);

            assert_eq!(res, test.1);
            assert_eq!(index, test.2);
        }
    }

    #[test]
    fn test_parse_dont() {
        let mut tests = vec![("don't()", true, 6), ("dont()", false, 3)];

        for test in tests {
            let mut line = test.0.chars().collect();
            let mut index = 0;
            let res = is_dont(&mut line, &mut index);
            // println!("index: {:?}, chars: {:?}",line);

            assert_eq!(res, test.1);
            assert_eq!(index, test.2);
        }
    }

    #[test]
    fn test_parse_2() {
        let mut tests = vec![
            // ("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48),
            // ("who()]#^don't()select()select())mul(724,851)[>&mul(188,482)$mul(781,111)[who()<why(),!]mul(678,13)why()$#%who()mul(620,771)<!^}@^+what()mul(281,719)(]'what()where()>&from():!mul(147,678)how(){mul(938,510)where()!$?*['mul(103,563)where())mul(4,125)$*>>^mul(126,929)]& %~mul(161,418)who()>>", 0),
            ("don't()(?]{why()%}from()mul(367,653)~mul(910,873)^why()>mul(499,785)>what()[*:#where()*what()mul(765,210)*$[]mul(461,957)##)+}when()-@:mul(198,90)what()what()how()') )mul(258,966)]+(when()mul(535,417)where()!don't()@mul(939,319)?mul(751,538))! mul(758,675)~how()[how(),@>[where()when(29,965)mul(358,39){^what();/(where()how()mul(271,786)why():mul(792,761)do()$]%mul(740,232)>who(949,378)what()[(where()who(){who()#mul(595,343)%+mul(194,296)'mul(161,747): '{where(12,567),@mul(234,39)!+", 0),
            // ("?% mul(948,148)why() %how(670,744)mul(590,32);where())#}from()>how()mul(611,372)}{~^?>from()^mul(835,665)do()]-''?mul(416,366)~?/where()]who()mul(459,47))>what(){@[(mul(219,400)+do()when()from():who()when()]&{{%mul(804,830)-select()what()*what()%}mul(861,992)who()!',mul(159,874)#<)''<mul(460,777)?mul(909,244)how()+what()]<do()?}mul(749,87)from()(who();why()mul(430,124)/$>how()@$%mul(214,139)&how()>mul(112,835)select()*from()@why()?[{mul(209,568)/; ~)mul(630,749):mul(841,589)/;who()>[mul(778,567)+when() how()<#mul(544,851)what(){+mul(327,103)from()what()/[~-mul(995,415)/when()-mul(880,153)}:}mul(368,920)'how()mul(864,419)from()what()@mul(208,291)who()<?}?what()',[{mul(575,454)*&(<{how()[mul(557,489){{why(){how()@who()~mul(423,703)mul(910,916)+what()^/<-*from()'mul(746,826),-*)/+>}^from()mul(154,571)++:>,mul(601,458)why()<;how()~from(172,16)mul(333,315)?[mul(513,260) {*mul(117,759)%]mul(77,644){($%>]&~mul(238,306)~select()from();-'who()'mul(460,352); ?select()>[[(from() mul(337,294)why()how()</$<where()do()/who()[where()&'what()when()how())mul(138,925)),#;where()>{mul(738,864){mul(605,662)*when()%when()+( /~&mul(633,935)when()];mul(263/}*<!where(),- ~when()mul(512,798)]}where())when()who()mul(933,447)where()}mul(33,935*mul(15,975)mul(574,550)+#^;'$from(280,157)$^what()mul(919,849)@mul(18,160))$&^]how()what() when()where()mul(88,657):/from())+:/when()@]mul(71,74)from()'*:@{>mul(127,821)^how()$$select()select()@^{:mul(867,979)&%/>{%^how()what(499,657)+do()%what()(~;-:*mul(438,941)<]?]mul(208,834when()&^;]from()when(613,710)^}+$mul(809,573)mul^)*:from(379,983)mul(47,786)}when()-what()how(450,632)> where()how()mul(810,597 ;;{%(select()select()&,mul(356,249)from()/!{#&^mul(23,248)(!who()]-+,mul(873,987)]{what()<  )-{^mul(591,317)/mul(382,188)mul(476,338)*why()$]mul(865,625)who()})?select():*@[)don't()/ ,mul(737,418)select(318,357);+ what()<mul(41,445)mul(236,630)$}from()]$^$,(do()-select()mul(369,197)from()]#};^mul(561,752)+&#+}?}:mul(18,235)<'& ,(*mul(645,811)why()select()who()[>where()don't()%#>!>/@what()[mul(490,823)&^( ,'@ [do()@mul(855,491)*^why()[,mul(348,679)how()$who() '&how(16,459)/!;mul(43,422)#^from()![}select()mul(976,749)-}select()-where()select()mul(223,589)%[why()mul(868,881)mul(178,790)$,{who()from()#,mul(318,399):where()?[mul(182,864)where() mul(156,690) -]mul(857,353)#'%,},>?+@mul(914,528)where()$mul(785,748)<$who()[mul(453,859)%'@ mul(84,729)/{do()(?$<}mul(820,286)?:*?}#when()(%mul(245,958when()?from(),+mul(128,335)mul(463,102);:]@-~-%mul(914,398)", 127),

        ];

        for test in tests {
            let mut line = test.0;
            let res = parse_line(&mut line);
            // println!("index: {:?}, chars: {:?}",line);

            assert_eq!(res, test.1);
        }
    }

    #[test]
    fn test_parse() {
        let mut tests = vec![(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            161,
        )];

        for test in tests {
            let mut line = test.0;
            let res = parse_line(&mut line);
            // println!("index: {:?}, chars: {:?}",line);

            assert_eq!(res, test.1);
        }
    }

    #[test]
    fn test_parse_mul() {
        let mut tests = vec![
            ("mul(1,1)".chars().collect::<Vec<char>>(), Some((1, 1))),
            ("mul(1,12)".chars().collect::<Vec<char>>(), Some((1, 12))),
            ("mul(1,123)".chars().collect::<Vec<char>>(), Some((1, 123))),
            ("mul(12,1)".chars().collect::<Vec<char>>(), Some((12, 1))),
            ("mul(12,12)".chars().collect::<Vec<char>>(), Some((12, 12))),
            (
                "mul(12,123)".chars().collect::<Vec<char>>(),
                Some((12, 123)),
            ),
            ("mul(123,1)".chars().collect::<Vec<char>>(), Some((123, 1))),
            (
                "mul(123,12)".chars().collect::<Vec<char>>(),
                Some((123, 12)),
            ),
            (
                "mul(123,123)".chars().collect::<Vec<char>>(),
                Some((123, 123)),
            ),
            ("mul?(123,123)".chars().collect::<Vec<char>>(), None),
            ("mul(1232,123)".chars().collect::<Vec<char>>(), None),
            ("mul(123,1232)".chars().collect::<Vec<char>>(), None),
            ("mul(,)".chars().collect::<Vec<char>>(), None),
        ];

        for test in tests {
            let mut chars = test.0;
            let mut index = 0;
            let end = test.1;
            let mut do_mul = true;
            let res = parse_mul(&mut chars, &mut index, &mut do_mul);
            println!("index: {:?}, chars: {:?}", index, chars);

            assert_eq!(res, test.1);
        }
    }

    #[test]
    fn test_parse_num() {
        let mut tests = vec![
            ("1".chars().collect::<Vec<char>>(), ',', Some(1)),
            ("12".chars().collect::<Vec<char>>(), ',', Some(12)),
            ("132".chars().collect::<Vec<char>>(), ',', Some(132)),
            ("132".chars().collect::<Vec<char>>(), ')', Some(132)),
            ("1324".chars().collect::<Vec<char>>(), ')', Some(132)),
            ("1,".chars().collect::<Vec<char>>(), ',', Some(1)),
            ("12,".chars().collect::<Vec<char>>(), ',', Some(12)),
            ("123,".chars().collect::<Vec<char>>(), ',', Some(123)),
            ("1234,".chars().collect::<Vec<char>>(), ',', Some(123)),
        ];

        for test in tests {
            let mut chars = test.0;
            let mut index = 0;
            let end = test.1;
            let res = parse_num(&mut chars, &mut index, end);
            println!("index: {:?}, chars: {:?}", index, chars);

            assert_eq!(res, test.2);
        }
    }
}
