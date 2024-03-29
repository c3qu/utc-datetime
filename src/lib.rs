use core::panic;
use std::fmt;

// 派生比较UtcDatetime的特性(=,>,<,<=,>=,!=)
#[derive(PartialEq,PartialOrd,Debug)]
pub struct UtcDatetime{
    year:u16,
    month:u8,
    day:u8,
    hour:u8,
    minute:u8,
    second:u8,
}

impl fmt::Display for UtcDatetime{
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

impl UtcDatetime{
    /// Create a new UtcDateTime structure
    pub fn new(year:u16,month:u8,day:u8,hour:u8,minute:u8,second:u8)->Result<UtcDatetime, IllegalTimeError>{
        if year<1970{
            // println!("年份非法");
            return Err(IllegalTimeError::YearNumberError)
        }
		if month==0 || month >12{
            // println!("月份非法");
            return Err(IllegalTimeError::MonthNumberError)
        }
        if day==0 || day >days_of_the_month(year,month){
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
        Ok(UtcDatetime{year,month,day,hour,minute,second})
    }
    /// Returns the number of seconds since January 1, 1970
    /// # Example
    /// ```
    /// use utc_datetime::UtcDatetime;
    /// let anew_date=UtcDatetime::new(2020,2,2,2,2,2).unwrap();
    /// assert_eq!(anew_date.timestamp().unwrap(),1580608922)
    /// ```
    pub fn timestamp(&self)->Result<u32,IllegalTimeError>{
        if self.year<1970{
            return Err(IllegalTimeError::YearNumberError)
        }
        let second=self.second as u32;
        let minute=self.minute as u32;
        let hour=self.hour as u32;
        let day =self.day as u32;
        
        let mut total_seconds=0;

        // 计算1970年到去年的秒数   
        for i in 1970..self.year{
            total_seconds+=days_of_the_year(i)*24*60*60;
        }

        // 计算今年过去的月份的秒数
        for i in 1..self.month{
            let days_num=days_of_the_month(self.year, i) as u32;
            total_seconds+=days_num*24*60*60;
        }

        // 计算这个月时间的秒数
        total_seconds+=(day-1)*60*60*24+hour*60*60+minute*60+second;
        
        Ok(total_seconds)
    }

    // 返回今天是星期几:星期一到星期六依次返回1到6，星期天返回0
    /// Return today is the day of the week,Monday to Saturday Return 1 to 6,Sunday return 0
    /// # Example
    /// ```
    /// use utc_datetime::UtcDatetime;
    /// let a_date=UtcDatetime::new(2021,11,15,0,0,0).unwrap();
    /// assert_eq!(a_date.weekday(),1);
    /// ```
    pub fn weekday(&self)->u8{
        let ts=self.timestamp().unwrap();
        //7*24*3600 为7天的秒数
        let this_week_seconds=ts%(7*24*3600);
        // 24*3600为一天的秒数
        let this_week_days=this_week_seconds/(24*3600);
        // 1970年1月1日是周四
        let week_number=(4+this_week_days)%7;
        week_number as u8
    }
    // 输入一个时间字符串(如"2002-04-01 00:00:01") 返回一个时间对象
    /// Convert a string containing time to UtcDatetime.
    /// 
    /// Time strings must be sorted by year, month, day, hour, minute, and second,
    /// and Non-arabic numbers can be used as separators.
    /// 
    /// Parsable string example:"2020-12-31 23:59:59","2020z12z31z23z59z59".
    /// # Example
    /// ```
    /// use utc_datetime::UtcDatetime;
    /// let datetime=UtcDatetime::from_string("时间:2020年12月31日23点59分59秒").unwrap();
    /// assert_eq!(datetime,UtcDatetime::new(2020,12,31,23,59,59).unwrap());
    /// ```
    pub fn from_string(time_str:&str)->Result<UtcDatetime, IllegalTimeError>{
		// 能转换的字符串的日期必须为阿拉伯数字，且顺序必须按照年,月,日,小时,分,秒的顺序
		// 只保留字符串中的阿拉伯数字
		// '0'-'9'的ascii码为48-57
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
        UtcDatetime::new(year,month,day,hour,minute,second)
    }
}

/// Conditions for judging leap years
/// 1. Divisible by 4, but not divisible by 100
/// 2. Divisible by 400
/// # Example
/// ```
/// use utc_datetime::leap_year;
/// assert!(leap_year(2000));
/// assert_eq!(leap_year(2021),false);
/// assert_eq!(leap_year(1900),false);
/// ```
pub fn leap_year(year:u16)->bool{
	// 判断闰年的条件
    // 1.能被4整除,但不能被100整除 
	// 2.能被400整除
    (year%4==0 && year%100!=0)||year%400==0
}

/// Returns the number of days in a year
pub fn days_of_the_year(year:u16)->u32{
    if leap_year(year){366}else{365}
}

/// Returns the number of days in this month
/// # Example
/// ```
/// use utc_datetime::days_of_the_month;
/// assert_eq!(days_of_the_month(2020,2),29);
/// assert_eq!(days_of_the_month(2020,3),31)
/// ```
pub fn days_of_the_month(year:u16,month:u8)->u8{
    match month{
        1|3|5|7|8|10|12=>31,
        4|6|9|11=>30,
        2=>{
            if leap_year(year){
                return 29
            }
            28
        }
        _=>panic!("Illegal number of days in the month.")
    }
}

#[cfg(test)]
mod tests{
    use super::UtcDatetime;
    #[test]
    fn test1() {
        let a_utc_datetime=UtcDatetime::from_string("时间:2021年2月28日23点59分0秒").unwrap();
        assert_eq!(a_utc_datetime,UtcDatetime::new(2021,2,28,23,59,0).unwrap());
    }

    #[test]
    fn test2(){
        let a=UtcDatetime::from_string("2020-12-31 23:59:59").unwrap(); 
        let b=UtcDatetime::from_string("2020/12/31 23:59:59").unwrap();   
        assert!(a==b);
    }

    #[test]
    fn test3(){
        let a=UtcDatetime::new(2020,4,28,12,12,12).unwrap();
        assert_eq!(a.weekday(),2);
    }

    #[test]
    fn test4(){
        let dt_1=UtcDatetime::new(2020,4,28,12,30,12).unwrap();
        let dt_2=UtcDatetime::new(2020,4,28,12,12,29).unwrap();
        assert!(dt_1>dt_2);
    }
}
