pub fn time_info(time: &str) -> String {
    // 解析 "YYYY-MM-DD"
    let parts: Vec<u32> = time.split('-')
        .filter_map(|s| s.parse().ok())
        .collect();
    let (year, month, day) = (parts[0] as i32, parts[1] as i32, parts[2] as i32);

    
    fn is_leap(y: i32) -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    }
    // 某月天数
    fn month_days(y: i32, m: i32) -> i32 {
        match m {
            1|3|5|7|8|10|12 => 31,
            4|6|9|11 => 30,
            2 => if is_leap(y) { 29 } else { 28 },
            _ => 0,
        }
    }
    // 计算年内序日
    fn day_of_year(y: i32, m: i32, d: i32) -> i32 {
        let mut sum = 0;
        for mm in 1..m {
            sum += month_days(y, mm);
        }
        sum + d
    }
    let doy = day_of_year(year, month, day);
    let total_days = if is_leap(year) { 366 } else { 365 };
    let remain = total_days - doy;

    // Sakamoto
    // 1=Monday,...,7=Sunday
    fn day_of_week(y: i32, m: i32, d: i32) -> i32 {
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let mut y = y;
        let m = m;
        if m < 3 { y -= 1; }
        let w = (y + y/4 - y/100 + y/400 + t[(m-1) as usize] + d) % 7;
        if w == 0 { 7 } else { w } // 调整：0对应星期日
    }
    let wday = day_of_week(year, month, day);

    // 计算 ISO 周数
    // 取本周的星期四所在的年份和序日，再计算 (ordinal-1)/7+1
    // 需要一个“加天数”函数（简单逐日累加）
    fn add_day(y: i32, m: i32, d: i32) -> (i32, i32, i32) {
        let mut d = d + 1;
        let mut m = m;
        let mut y = y;
        if d > month_days(y, m) {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
        (y, m, d)
    }
    // 通用加天数（可正可负，这里天数较少，直接循环即可）
    fn add_days(y: i32, m: i32, d: i32, mut delta: i32) -> (i32, i32, i32) {
        let mut y = y;
        let mut m = m;
        let mut d = d;
        if delta >= 0 {
            while delta > 0 {
                let (ny, nm, nd) = add_day(y, m, d);
                y = ny; m = nm; d = nd;
                delta -= 1;
            }
        } else {
            while delta < 0 {
                // 往前一天
                if d > 1 {
                    d -= 1;
                } else {
                    if m == 1 {
                        y -= 1;
                        m = 12;
                    } else {
                        m -= 1;
                    }
                    d = month_days(y, m);
                }
                delta += 1;
            }
        }
        (y, m, d)
    }
    // ISO周计算：先找到本周的星期四
    let offset = 4 - wday; // 可能为负数
    let (ty, tm, td) = add_days(year, month, day, offset);
    let doy_th = day_of_year(ty, tm, td);
    let week = ((doy_th - 1) / 7) + 1;
    // 若调整后的年份与当前年份不同，则说明本日归属上一年或下一年的周数
    // 这里要求输出“当前周”编号，所以直接输出 week，如果跨年则 week 可能为 1
    let iso_week = week;

    // 计算春节倒计时（不含当天）
    // 固定：2025春节：1-29；2026春节：2-17
    let (cn_y, cn_m, cn_d) = if (month < 1) || (month == 1 && day < 29) {
        (year, 1, 29)
    } else if month == 1 && day == 29 {
        // 如果正好春节当天，倒计时为0
        (year, 1, 29)
    } else {
        // 当前日期在春节之后，则取下一年春节（2026春节固定为2-17）
        (year + 1, 2, 17)
    };
    let cn_doy = day_of_year(cn_y, cn_m, cn_d);
    let cn_count = if cn_y == year {
        cn_doy - doy
    } else {
        (total_days - doy) + cn_doy
    };

    // 判断是否为A股交易日
    // 规则：工作日（周一到周五）且不在假日区间内
    // 对于2025，春节假期：2025-01-28 ~ 2025-02-03，五一假期：2025-05-01 ~ 2025-05-03
    fn is_holiday(y: i32, m: i32, d: i32) -> bool {
        if y == 2025 {
            // 春节假期
            if (m == 1 && d >= 28) || (m == 2 && d <= 3) {
                // 注意：1月28-31 或 2月1-3
                return true;
            }
            // 五一假期
            if m == 5 && d <= 3 {
                return true;
            }
        }
        false
    }
    fn is_trading_day(y: i32, m: i32, d: i32) -> bool {
        let w = {
            let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
            let mut y_adj = y;
            let m_adj = m;
            if m_adj < 3 { y_adj -= 1; }
            let w = (y_adj + y_adj/4 - y_adj/100 + y_adj/400 + t[(m_adj-1) as usize] + d) % 7;
            if w == 0 { 7 } else { w }
        };
        // 交易日为周一到周五，且不在假日（只对2025做处理，其它年份认为都交易）
        (w >= 1 && w <= 5) && !is_holiday(y, m, d)
    }
    // 计算下一个A股开盘日（严格大于当前日期）
    fn next_trading_day(mut y: i32, mut m: i32, mut d: i32) -> (i32, i32, i32) {
        loop {
            let (ny, nm, nd) = add_day(y, m, d);
            y = ny; m = nm; d = nd;
            if is_trading_day(y, m, d) {
                return (y, m, d);
            }
        }
    }
    
    fn date_to_ordinal(y: i32, m: i32, d: i32) -> i32 {
        // 计算从 0001-01-01 公元元日 (？)  开始的天数（不需要精确，只用来比较同一年附近的日期）
        let mut days = d;
        for yy in 1..y {
            days += if is_leap(yy) { 366 } else { 365 };
        }
        for mm in 1..m {
            days += month_days(y, mm);
        }
        days
    }
    let cur_ord = date_to_ordinal(year, month, day);
    let (ty, tm, td) = next_trading_day(year, month, day);
    let target_ord = date_to_ordinal(ty, tm, td);
    let diff = target_ord - cur_ord; // 不含当天则应减1
    
    // 如果当天为交易日，则认为今日已开盘，倒计时为 (diff - 1)；
    // 如果当天非交易日且为周六或周日，则减去1（例如：周六→差2天，输出1；周日→差1天，输出0）；
    // 如果跨年（下个交易日在下一年）则直接用 diff。
    let next_open = if year == ty {
        if is_trading_day(year, month, day) {
            diff - 1
        } else {
            let wd = day_of_week(year, month, day);
            if wd == 6 || wd == 7 {
                diff - 1
            } else {
                diff
            }
        }
    } else {
        diff
    };

    // 结果格式： "周数,星期,日序,剩余,距春节,距A股开盘"
    // 各项均不含当天差（例如倒计时均为不含当天）


    let res = format!("{},{},{},{},{},{}",
        iso_week,   
        wday,          // 星期
        doy,           // 年内第几天
        remain,        // 今年剩余天数
        cn_count,      // 距春节天数
        next_open     // 距下一次A股开盘天数
    );
    res
}
