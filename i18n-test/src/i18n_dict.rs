use strum::EnumString;
#[derive(Debug, EnumString, PartialEq, Eq)]
pub enum I18nDict{
                    #[strum(serialize = "hello", to_string = "hello")]
                    HELLO,
                    #[strum(serialize = "satisfied", to_string = "satisfied")]
                    HAPPY,
                    #[strum(serialize = "good", to_string = "good")]
                    JOY
}