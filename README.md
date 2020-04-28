# The main function
```
impl UTCDatetime{
    fn new(year:u16,month:u8,day:u8,hour:u8,minute:u8,second:u8)->Result<UTCDatetime, IllegalTimeError>;
    fn get_timestamp(&self)->Result<u32,IllegalTimeError>;
    fn day_of_the_week(&self)->u8;
    fn from_string(time_str:&str)->Result<UTCDatetime, IllegalTimeError>;
}
```
The UTCDatetime structure derives PartialEq and PartialOrd,
you can directly use <,>, ==, <=,> = for comparison.