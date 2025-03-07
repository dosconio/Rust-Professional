pub fn retire_time(time: &str, tp: &str) -> String {
    let (birth_year, birth_month) = parse_time(time);
    let (original_age, delay_fn): (i32, fn(i32,i32) -> i32) = match tp {
        "男职工" => (60, male_delay),
        s if s.starts_with("原法定退休年龄55周岁女职工") => (55, female_55_delay),
        s if s.starts_with("原法定退休年龄50周岁女职工") => (50, female_50_delay),
        _ => panic!("未知人员类型: {}", tp),
    };

    let y_original = birth_year + original_age;
    let delay_months = delay_fn(y_original, birth_month as i32);

    let original_total_months = y_original * 12 + (birth_month - 1) as i32;
    let new_total_months = original_total_months + delay_months as i32;

    let new_year = new_total_months / 12;
    let new_month = (new_total_months % 12) as u32 + 1;

    let total_months_diff = (new_year - birth_year) * 12 + (new_month as i32 - birth_month as i32);
    let age = total_months_diff as f64 / 12.0;

    let age_str = if (age - age.round()).abs() < 1e-2 {
        format!("{:.0}", age)
    } else {
        format!("{:.2}", age)
    };

    format!("{:04}-{:02},{},{}", new_year, new_month, age_str.trim_end_matches(".00"), delay_months)
}

fn parse_time(time: &str) -> (i32, u32) {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    (year, month)
}

/*
国务院关于渐进式延迟法定退休年龄的办法

坚持以习近平新时代中国特色社会主义思想为指导，深入贯彻党的二十大和二十届二中、三中全会精神，综合考虑我国人均预期寿命、健康水平、人口结构、国民受教育程度、劳动力供给等因素，按照小步调整、弹性实施、分类推进、统筹兼顾的原则，实施渐进式延迟法定退休年龄。为了做好这项工作，特制定本办法。

第一条　

从2025年1月1日起，男职工和原法定退休年龄为五十五周岁的女职工，法定退休年龄每四个月延迟一个月，分别逐步延迟至六十三周岁和五十八周岁；

原法定退休年龄为五十周岁的女职工，法定退休年龄每二个月延迟一个月，逐步延迟至五十五周岁。国家另有规定的，从其规定。
*/

fn male_delay(y_original: i32, m: i32) -> i32 {
    if y_original < 2025 {
        0
    } else {
		let new_month = 1 + (y_original - 2025) * 3;
        let d = new_month + ((m - 1) / 4) as i32;
        d.min(12 * 3)
    }
}

fn female_55_delay(y_original: i32, m: i32) -> i32 {
	if y_original < 2025 {
        0
    } else {
		let new_month = 1 + (y_original - 2025) * 3;
        let d = new_month + ((m - 1) / 4) as i32;
        d.min(12 * 3)
    }
}

fn female_50_delay(y_original: i32, m: i32) -> i32 {
    //60
	if y_original < 2025 {
        0
    } else {
		let new_month = 1 + (y_original - 2025) * 6;
        let d = new_month + ((m - 1) / 2) as i32;
        d.min(12 * 5)
    }
}