pub mod models{

    pub struct Placeholder;


    pub struct Mail{
        pub header: String,
        pub body: String
    }

    impl Default for Mail{
        fn default() -> Self {
            Self { header: "".to_string(), body:"".to_string()}
        }
    }

    impl Mail{
        pub fn new() -> Self{
            Self::default()
        }

        pub fn get_raw(&self,seperator: Option<&str>) -> String{
            self.header.clone() +if let Some(x) = seperator{x}else{""}+ &self.body
        }

        


    }




}