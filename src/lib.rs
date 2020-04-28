use std::fmt;

// 派生比较UTCDatetime的特性(=,>,<,<=,>=,!=)
#[derive(PartialEq,PartialOrd,Debug)]
pub struct UTCDatetime{
    year:u16,
    month:u8,
    day:u8,
    hour:u8,
    minute:u8,
    second:u8,
}

impl fmt::Display for UTCDatetime{
    fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        // 指定宽度输入数字
        write!(f,"{}-{:02}-{:02} {:02}:{:02}:{:02}",self.year,self.month,self.day,self.hour,self.minute,self.second)
    }
}

pub enum IllegalTimeError{
    YearNumberError,
    MonthNumberError,
    DayNumberError,
    HourNumberError,
    MinuteNumberError,
    SecondNumberError,
    TimeStringError
}

impl fmt::Debug for IllegalTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            IllegalTimeError::YearNumberError=>write!(f, "Year Number Error"),
            IllegalTimeError::MonthNumberError=>write!(f, "Month Number Error"),
            IllegalTimeError::DayNumberError=>write!(f, "Day Number Error"),
            IllegalTimeError::HourNumberError=>write!(f, "Hour Number Error"),
            IllegalTimeError::MinuteNumberError=>write!(f, "Minute Number Error"),
            IllegalTimeError::SecondNumberError=>write!(f, "Second Number Error"),
            IllegalTimeError::TimeStringError=>write!(f,"The format of the input time string is not standardized")
        }
    }
}

impl UTCDatetime{
    /// Create a new UTCTimedate structure
    pub fn new(year:u16,month:u8,day:u8,hour:u8,minute:u8,second:u8)->Result<UTCDatetime, IllegalTimeError>{
        if month==0 || month >12{
            // println!("月份非法");
            return Err(IllegalTimeError::MonthNumberError)
        }
        if day==0 || day >31{
            // println!("天数非法");
            return Err(IllegalTimeError::DayNumberError)
        }
        if hour >23{
            // println!("小时数非法");
            return Err(IllegalTimeError::HourNumberError)
        }
        if minute>59{
            // println!("分钟数非法");
            return Err(IllegalTimeError::MinuteNumberError)
        }
        if second>59{
            // println!("秒数非法");
            return Err(IllegalTimeError::SecondNumberError)
        }
        Ok(UTCDatetime{year,month,day,hour,minute,second})
    }
    /// Returns the number of seconds since January 1, 1970
    pub fn get_timestamp(&self)->Result<u32,IllegalTimeError>{
        if self.year<1970{
            return Err(IllegalTimeError::YearNumberError)
        }
        let second=self.second as u32;
        let minute=self.minute as u32;
        let hour=self.hour as u32;
        let day =self.day as u32;

        // 计算这个月的秒数
        let seconds_this_month=second+60*minute+60*60*hour+60*60*24*(day-1);
        
        let mut seconds_past_years:u32=0;
        for i in 1970..self.year{
            if is_leap_year(i){
                seconds_past_years+=366*24*60*60;
            }else{
                seconds_past_years+=365*24*60*60;   
            }
        }

        // 计算今年过去的月份的秒数
        let mut seconds_past_months=0;
        for i in 1..self.month{
            let days_num=days_of_the_month(self.year, i) as u32;
            seconds_past_months+=days_num*24*60*60;
        }
        Ok(seconds_past_years+seconds_past_months+seconds_this_month)
    }

    // 返回今天是星期几 星期一到六 返回1到6 星期天返回0
    /// Return today is the day of the week,Monday to Saturday Return 1 to 6,Sunday return 0
    pub fn day_of_the_week(&self)->u8{
        let ts=self.get_timestamp().unwrap();
        //7*24*3600 为7天的秒数
        let this_week_seconds=ts%(7*24*3600);
        // 24*3600为一天的秒数
        let this_week_days=this_week_seconds/(24*3600);
        // 1970年1月1日是周四
        let week_number=(4+this_week_days)%7;
        week_number as u8
    }
    // 输入一个时间字符串(如"2002-04-01 00:00:01") 返回一个时间对象
    /// Convert a string containing time to UTCDatetime.
    /// 
    /// Time strings must be sorted by year, month, day, hour, minute, and second,
    /// and Non-arabic numbers can be used as separators.
    /// 
    /// Parsable string example:"2020-12-31 23:59:59","2020z12z31z23z59z59".
    /// # Example
    /// ```
    /// use utc_datetime::UTCDatetime;
    /// let a_utc_datetime=UTCDatetime::from_string("时间:2020年12月31日23点59分59秒").unwrap();
    /// assert_eq!(a_utc_datetime,UTCDatetime::new(2020,12,31,23,59,59).unwrap());
    /// ```
    pub fn from_string(time_str:&str)->Result<UTCDatetime, IllegalTimeError>{
        let mut time_string_array:Vec<&str>=time_str.split(|x| (x as u8) < 48 || x as u8  >57).collect();
        // retain non-empty items in time_string_array
        time_string_array.retain(|&x|x.len()!=0);
        if time_string_array.len()!=6{
            return Err(IllegalTimeError::TimeStringError)
        }   
        let year=time_string_array[0].parse::<u16>().unwrap();
        let month=time_string_array[1].parse::<u8>().unwrap();
        let day=time_string_array[2].parse::<u8>().unwrap();
        let hour=time_string_array[3].parse::<u8>().unwrap();
        let minute=time_string_array[4].parse::<u8>().unwrap();
        let second=time_string_array[5].parse::<u8>().unwrap();
        UTCDatetime::new(year,month,day,hour,minute,second)
    }
}


fn is_leap_year(year:u16)->bool{
    // 1.能被4整除,但不能被100整除 2能被400整除
    if (year%4==0 && year%100!=0)||year%400==0{
        return true
    }
    false
}

fn days_of_the_month(year:u16,month:u8)->u8{
    match month{
        1|3|5|7|8|10|12=>31,
        4|6|9|11=>30,
        2=>{
            if is_leap_year(year){
                return 29
            }
            28
        }
        _=>{0}
    }
}

#[cfg(test)]
mod tests{
    use super::UTCDatetime;
    #[test]
    fn mytest() {
        let a_utc_datetime=UTCDatetime::from_string("时间:2020年12月31日23点59分59秒").unwrap();
        assert_eq!(a_utc_datetime,UTCDatetime::new(2020,12,31,23,59,59).unwrap());
    }

    #[test]
    fn test2(){
        let a=UTCDatetime::from_string("2020/12-31 23 59 59").unwrap(); 
        let b=UTCDatetime::from_string("2020/12-31 23 59 59").unwrap();   
        assert_eq!(a==b,true);
    }

    #[test]
    fn test_week(){
        let a=UTCDatetime::from_string("2020 4 28 12 12 12").unwrap();
        assert_eq!(a.day_of_the_week(),2);
    }
}
