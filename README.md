# The main function
```
impl UtcDatetime{
    fn new(year:u16,month:u8,day:u8,hour:u8,minute:u8,second:u8)->Result<UTCDatetime, IllegalTimeError>;
    fn timestamp(&self)->Result<u32,IllegalTimeError>;
    fn weekday(&self)->u8;
    fn from_string(time_str:&str)->Result<UTCDatetime, IllegalTimeError>;
}
```
The UTCDatetime structure derives PartialEq and PartialOrd,
you can directly use <,>, ==, <=,>=,!= for comparison.
